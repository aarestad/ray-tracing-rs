use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::mpsc::channel;
use std::sync::Arc;

use rand::Rng;
use threadpool::ThreadPool;

use crate::camera::Camera;
use crate::data::color64::Color64;
use crate::hittables::Hittable;
use crate::util::worlds::World;

/// Renders one sample per pixel across `row_y`, returning `(flipped_y, samples)`.
///
/// `flipped_y` is `render_h - row_y - 1` so the caller can write directly into a
/// top-left-origin image buffer without extra math.
pub fn render_row(
    camera: &Camera,
    hittable: &Hittable,
    background: &Color64,
    row_y: u32,
    render_w: u32,
    render_h: u32,
    max_depth: i32,
    rng: &mut impl Rng,
) -> (u32, Vec<Color64>) {
    let du = render_w.saturating_sub(1).max(1) as f64;
    let dv = render_h.saturating_sub(1).max(1) as f64;
    let flipped_y = render_h - row_y - 1;
    let row = (0..render_w)
        .map(|x| {
            let u = (x as f64 + rng.random::<f64>()) / du;
            let v = (row_y as f64 + rng.random::<f64>()) / dv;
            camera
                .get_ray(u, v)
                .color_in_world(hittable, background, max_depth, rng)
        })
        .collect();
    (flipped_y, row)
}

/// Renders `num_samples` samples per pixel across all rows, dispatching work in
/// groups of `rows_per_task` rows to an internal thread pool.
///
/// Returns `Some(rows)` on completion, where each `Color64` is the *sum* of
/// `num_samples` samples (divide by `num_samples` when converting to final pixel
/// values). Returns `None` if `cancel` is `Some((gen, expected))` and `gen` no
/// longer equals `expected` mid-pass (interactive view-changed abort). Pass
/// `None` for `cancel` to disable cancellation (batch rendering).
pub fn render_frame(
    camera: Camera,
    world: Arc<World>,
    render_w: u32,
    render_h: u32,
    max_depth: i32,
    rows_per_task: u32,
    num_samples: u32,
    cancel: Option<(Arc<AtomicU64>, u64)>,
) -> Option<Vec<(u32, Vec<Color64>)>> {
    let pool = ThreadPool::new(num_cpus::get());
    let (tx, rx) = channel::<(u32, Vec<Color64>)>();

    let mut y = 0u32;
    while y < render_h {
        let y_end = (y + rows_per_task).min(render_h);
        let tx = tx.clone();
        let world = world.clone();
        let camera = camera.clone();
        let cancel = cancel.clone();
        pool.execute(move || {
            if let Some((ref generation, expected)) = cancel
                && generation.load(Ordering::Acquire) != expected
            {
                return;
            }
            let mut rng = rand::rng();
            for row_y in y..y_end {
                let mut accumulated = vec![Color64::new(0., 0., 0.); render_w as usize];
                for _ in 0..num_samples {
                    let (_, row) = render_row(
                        &camera,
                        &world.hittable,
                        &world.background_color,
                        row_y,
                        render_w,
                        render_h,
                        max_depth,
                        &mut rng,
                    );
                    for (i, c) in row.into_iter().enumerate() {
                        accumulated[i] += c;
                    }
                }
                let flipped_y = render_h - row_y - 1;
                let _ = tx.send((flipped_y, accumulated));
            }
        });
        y = y_end;
    }

    drop(tx);

    let mut rows = Vec::with_capacity(render_h as usize);
    for (idx, row) in rx.iter().enumerate() {
        if let Some((ref generation, expected)) = cancel
            && generation.load(Ordering::Acquire) != expected
        {
            return None;
        }
        rows.push(row);
        println!("{} / {} scanlines done", idx,  world.image_height);
    }

    if cancel.is_none_or(|(generation, expected)| generation.load(Ordering::Acquire) == expected) {
        Some(rows)
    } else {
        None
    }
}
