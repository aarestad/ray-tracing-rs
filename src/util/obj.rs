//! Load Wavefront OBJ meshes as [`Hittable::Triangle`] lists (via `tobj`).

use std::path::Path;

use nalgebra::Vector3;

use crate::data::point64::Point64;
use crate::hittables::Hittable;
use crate::hittables::triangle::Triangle;
use crate::materials::Material;

/// Axis-aligned bounds of all vertex positions in the OBJ (model space).
#[derive(Clone, Copy, Debug)]
pub struct ObjAxisBounds {
    #[allow(dead_code)]
    pub x_min: f64,
    #[allow(dead_code)]
    pub x_max: f64,
    pub y_min: f64,
    pub y_max: f64,
    pub z_min: f64,
    pub z_max: f64,
}

impl ObjAxisBounds {
    pub fn height(&self) -> f64 {
        self.y_max - self.y_min
    }
}

pub fn obj_mesh_axis_bounds(path: &Path) -> anyhow::Result<ObjAxisBounds> {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    )?;

    let mut x_min = f64::INFINITY;
    let mut x_max = f64::NEG_INFINITY;
    let mut y_min = f64::INFINITY;
    let mut y_max = f64::NEG_INFINITY;
    let mut z_min = f64::INFINITY;
    let mut z_max = f64::NEG_INFINITY;

    for model in models {
        for chunk in model.mesh.positions.chunks(3) {
            if chunk.len() == 3 {
                let x = chunk[0] as f64;
                let y = chunk[1] as f64;
                let z = chunk[2] as f64;
                x_min = x_min.min(x);
                x_max = x_max.max(x);
                y_min = y_min.min(y);
                y_max = y_max.max(y);
                z_min = z_min.min(z);
                z_max = z_max.max(z);
            }
        }
    }

    if !y_min.is_finite() {
        anyhow::bail!("no vertices in {}", path.display());
    }

    Ok(ObjAxisBounds {
        x_min,
        x_max,
        y_min,
        y_max,
        z_min,
        z_max,
    })
}

/// Minimum and maximum **Y** among all vertex positions in the OBJ (model space).
#[allow(dead_code)]
pub fn obj_mesh_y_bounds(path: &Path) -> anyhow::Result<(f64, f64)> {
    let b = obj_mesh_axis_bounds(path)?;
    Ok((b.y_min, b.y_max))
}

/// Triangulate faces and build one [`Triangle`] per face, with uniform scale and translation.
pub fn load_obj_triangles(
    path: &Path,
    material: Material,
    scale: f64,
    offset: Vector3<f64>,
) -> anyhow::Result<Vec<Hittable>> {
    let (models, _materials) = tobj::load_obj(
        path,
        &tobj::LoadOptions {
            triangulate: true,
            ..Default::default()
        },
    )?;

    let mut out = Vec::new();

    for model in models {
        let mesh = &model.mesh;
        let pos = &mesh.positions;
        if pos.len() < 3 {
            continue;
        }

        for tri in mesh.indices.chunks(3) {
            if tri.len() != 3 {
                continue;
            }
            let i0 = tri[0] as usize * 3;
            let i1 = tri[1] as usize * 3;
            let i2 = tri[2] as usize * 3;
            if i2 + 2 >= pos.len() {
                continue;
            }

            let p1 = transform_vertex(pos, i0, scale, offset);
            let p2 = transform_vertex(pos, i1, scale, offset);
            let p3 = transform_vertex(pos, i2, scale, offset);

            out.push(Hittable::Triangle(Triangle::new(
                p1, p2, p3, material.clone(),
            )));
        }
    }

    if out.is_empty() {
        anyhow::bail!("no triangles in OBJ: {}", path.display());
    }

    Ok(out)
}

fn transform_vertex(positions: &[f32], i: usize, scale: f64, offset: Vector3<f64>) -> Point64 {
    Point64::new(
        positions[i] as f64 * scale + offset.x,
        positions[i + 1] as f64 * scale + offset.y,
        positions[i + 2] as f64 * scale + offset.z,
    )
}
