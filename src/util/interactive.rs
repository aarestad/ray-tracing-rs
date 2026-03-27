use std::sync::atomic::{AtomicU32, AtomicU64, Ordering};
use std::sync::mpsc::channel;
use std::sync::{Arc, Mutex};
use std::time::Duration;

use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use nalgebra::{Rotation3, Unit};
use rand::Rng;
use threadpool::ThreadPool;

use crate::camera::Camera;
use crate::data::color64::Color64;
use crate::data::point64::Point64;
use crate::util::worlds::World;

const PITCH_LIMIT: f64 = 1.553;
const DRAG_SENS: f64 = 0.0035;
const ROLL_SENS: f64 = 0.004;
/// Vertical wheel: scale distance by exp(-sy * this). Tune with minifb’s scroll delta units.
const ZOOM_WHEEL_EXP: f64 = 0.11;
const MIN_ORBIT_DISTANCE: f64 = 0.08;
const MAX_ORBIT_DISTANCE: f64 = 5_000.0;

/// Orbit the camera around `world.camera_target`; roll twists around the view axis.
#[derive(Clone)]
struct OrbitState {
    yaw: f64,
    pitch: f64,
    roll: f64,
    distance: f64,
}

impl OrbitState {
    fn from_world(world: &World) -> Self {
        let look_from = world.camera.origin();
        let target = world.camera_target;
        let offset = look_from - target;
        let d = offset.0.norm().max(1e-6);
        let pitch = (offset.y() / d).clamp(-0.999, 0.999).asin();
        let yaw = f64::atan2(offset.x(), offset.z());
        Self {
            yaw,
            pitch,
            roll: 0.,
            distance: d,
        }
    }

    fn look_from(&self, target: Point64) -> Point64 {
        let cp = self.pitch.cos();
        let x = self.distance * cp * self.yaw.sin();
        let y = self.distance * self.pitch.sin();
        let z = self.distance * cp * self.yaw.cos();
        target + Point64::new(x, y, z)
    }

    fn to_camera(&self, world: &World) -> Camera {
        let look_at = world.camera_target;
        let look_from = self.look_from(look_at);
        let forward = Unit::new_normalize((look_at - look_from).0);
        let rolled_vup = Rotation3::from_axis_angle(&forward, self.roll) * world.camera_v_up;
        let aspect = world.image_width as f64 / world.image_height as f64;
        Camera::new(
            look_from,
            look_at,
            rolled_vup,
            world.camera_vfov_deg,
            aspect,
            world.camera_aperture,
            world.camera_focus_distance,
            world.camera_exposure_time.clone(),
        )
    }
}

struct SharedRender {
    orbit: Mutex<OrbitState>,
    generation: Arc<AtomicU64>,
    accum: Mutex<Vec<Color64>>,
    samples: AtomicU32,
    display: Mutex<Vec<u32>>,
}

fn render_pass(
    pool: &ThreadPool,
    world: &Arc<World>,
    camera: &Camera,
    accum: &mut [Color64],
    view_gen: u64,
    generation: &Arc<AtomicU64>,
) -> bool {
    let (tx, rx) = channel::<(u32, Vec<Color64>)>();
    let h = world.image_height;
    let w = world.image_width as usize;

    for y in 0..h {
        let tx = tx.clone();
        let world = world.clone();
        let camera = camera.clone();
        let generation = Arc::clone(generation);
        pool.execute(move || {
            if generation.load(Ordering::Acquire) != view_gen {
                return;
            }
            let flipped_y = world.image_height - y - 1;
            let mut row = Vec::with_capacity(w);
            let mut rng = rand::rng();

            for x in 0..world.image_width {
                let du = world.image_width.saturating_sub(1).max(1) as f64;
                let dv = world.image_height.saturating_sub(1).max(1) as f64;
                let u = (x as f64 + rng.random::<f64>()) / du;
                let v = (flipped_y as f64 + rng.random::<f64>()) / dv;
                let ray = camera.get_ray(u, v);
                let c = ray.color_in_world(
                    world.hittable.as_ref(),
                    &world.background_color,
                    &mut rng,
                );
                row.push(c);
            }

            let _ = tx.send((flipped_y, row));
        });
    }

    drop(tx);

    for (flipped_y, row) in rx.iter() {
        if generation.load(Ordering::Acquire) != view_gen {
            return false;
        }
        let base = flipped_y as usize * w;
        for (x, c) in row.into_iter().enumerate() {
            accum[base + x] += c;
        }
    }

    generation.load(Ordering::Acquire) == view_gen
}

fn tonemap_to_display(accum: &[Color64], samples: u32, out: &mut [u32]) {
    let s = samples.max(1);
    for (i, px) in accum.iter().enumerate() {
        out[i] = px.to_minifb_rgb(s);
    }
}

fn render_thread(world: Arc<World>, shared: Arc<SharedRender>) {
    let pool = ThreadPool::new(num_cpus::get());
    let w = world.image_width as usize;
    let h = world.image_height as usize;
    let len = w * h;

    loop {
        let view_gen = shared.generation.load(Ordering::Acquire);
        {
            let mut acc = shared.accum.lock().unwrap();
            if acc.len() != len {
                acc.resize(len, Color64::new(0., 0., 0.));
            } else {
                acc.fill(Color64::new(0., 0., 0.));
            }
            shared.samples.store(0, Ordering::Release);
        }

        let mut local_samples = 0u32;
        let spp_cap = world.samples_per_pixel.max(1);

        loop {
            let g = shared.generation.load(Ordering::Acquire);
            if g != view_gen {
                break;
            }

            if local_samples >= spp_cap {
                std::thread::sleep(Duration::from_millis(32));
                continue;
            }

            let orbit = shared.orbit.lock().unwrap().clone();
            let camera = orbit.to_camera(world.as_ref());

            let mut accum = shared.accum.lock().unwrap();
            if !render_pass(
                &pool,
                &world,
                &camera,
                accum.as_mut_slice(),
                view_gen,
                &shared.generation,
            ) {
                drop(accum);
                break;
            }
            local_samples += 1;
            shared.samples.store(local_samples, Ordering::Release);

            let mut display = shared.display.lock().unwrap();
            tonemap_to_display(&accum, local_samples, &mut display);
            drop(accum);
            drop(display);
        }
    }
}

/// Opens a window, runs progressive path tracing with the current `samples_per_pixel` as the
/// target count per pixel (restarts accumulation when the view changes).  
/// **LMB drag:** yaw / pitch. **RMB drag:** roll. **Mouse wheel:** zoom (orbit distance). **Esc:** close.
pub fn run_interactive(world: Arc<World>) -> Result<(), String> {
    let w = world.image_width as usize;
    let h = world.image_height as usize;
    let len = w * h;

    let orbit = OrbitState::from_world(world.as_ref());
    let shared = Arc::new(SharedRender {
        orbit: Mutex::new(orbit),
        generation: Arc::new(AtomicU64::new(0)),
        accum: Mutex::new(vec![Color64::new(0., 0., 0.); len]),
        samples: AtomicU32::new(0),
        display: Mutex::new(vec![0u32; len]),
    });

    let world_render = world.clone();
    let shared_render = shared.clone();
    std::thread::spawn(move || render_thread(world_render, shared_render));

    let mut window = Window::new(
        "ray-tracer (LMB orbit, RMB roll, wheel zoom)",
        w,
        h,
        WindowOptions::default(),
    )
    .map_err(|e| e.to_string())?;

    #[allow(deprecated)]
    window.limit_update_rate(Some(Duration::from_millis(33)));

    let mut last_left: Option<(f32, f32)> = None;
    let mut last_right: Option<(f32, f32)> = None;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let pos = window.get_mouse_pos(MouseMode::Clamp);

        if let Some((mx, my)) = pos {
            let left = window.get_mouse_down(MouseButton::Left);
            let right = window.get_mouse_down(MouseButton::Right);

            if left {
                if let Some((lx, ly)) = last_left {
                    let dx = (mx - lx) as f64;
                    let dy = (my - ly) as f64;
                    if dx != 0. || dy != 0. {
                        let mut o = shared.orbit.lock().unwrap();
                        o.yaw += dx * DRAG_SENS;
                        o.pitch -= dy * DRAG_SENS;
                        o.pitch = o.pitch.clamp(-PITCH_LIMIT, PITCH_LIMIT);
                        shared.generation.fetch_add(1, Ordering::AcqRel);
                    }
                }
                last_left = Some((mx, my));
            } else {
                last_left = None;
            }

            if right {
                if let Some((lx, ly)) = last_right {
                    let dx = (mx - lx) as f64;
                    let dy = (my - ly) as f64;
                    if dx != 0. || dy != 0. {
                        let mut o = shared.orbit.lock().unwrap();
                        o.roll += (dx + dy) * ROLL_SENS;
                        shared.generation.fetch_add(1, Ordering::AcqRel);
                    }
                }
                last_right = Some((mx, my));
            } else {
                last_right = None;
            }
        }

        if let Some((_sx, sy)) = window.get_scroll_wheel() {
            let sy = sy as f64;
            if sy.abs() > f64::EPSILON {
                let mut o = shared.orbit.lock().unwrap();
                o.distance *= (-sy * ZOOM_WHEEL_EXP).exp();
                o.distance = o.distance.clamp(MIN_ORBIT_DISTANCE, MAX_ORBIT_DISTANCE);
                shared.generation.fetch_add(1, Ordering::AcqRel);
            }
        }

        if let Ok(guard) = shared.display.try_lock() {
            let _ = window.update_with_buffer(&guard, w, h);
        } else {
            let _ = window.update();
        }

        std::thread::sleep(Duration::from_millis(8));
    }

    Ok(())
}
