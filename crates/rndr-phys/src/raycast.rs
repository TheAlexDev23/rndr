use std::ops::ControlFlow;

use rndr_core::default_components::{render::MeshRenderable, Transform};

use rndr_core::object::{Object, ObjectManager};
use rndr_math::prelude::{M3x3, Vertex, V3};

pub struct Ray<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub objects: &'a ObjectManager,
}

impl<'a> Ray<'a> {
    pub fn cast(&self) -> Option<Vertex> {
        for obj in self.objects.objects_iter() {
            let mesh = match obj.component::<MeshRenderable>() {
                Some(obj) => obj,
                None => continue,
            };
            let transform = obj.component::<Transform>().unwrap();

            for triangle in &mesh.triangles {
                if let Some(v) = ray_mesh_triangle_intersect(
                    self.dir,
                    self.start,
                    self.max_distance,
                    mesh,
                    triangle,
                    transform,
                ) {
                    return Some(v);
                }
            }
        }

        None
    }
}

pub struct ObjectIntersectionRay<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub object: &'a Object,
}

impl<'a> ObjectIntersectionRay<'a> {
    pub fn cast(&self) -> Vec<Vertex> {
        let mesh = self.object.component::<MeshRenderable>().unwrap();
        let transform = self.object.component::<Transform>().unwrap();
        let mut ret = Vec::new();

        for triangle in &mesh.triangles {
            if let Some(v) = ray_mesh_triangle_intersect(
                self.dir,
                self.start,
                self.max_distance,
                mesh,
                triangle,
                transform,
            ) {
                ret.push(v);
            }
        }
        ret
    }
}

fn ray_mesh_triangle_intersect(
    dir: V3,
    start: V3,
    max_distance: Option<f32>,
    mesh: &MeshRenderable,
    triangle: &[usize; 3],
    transform: &Transform,
) -> Option<Vertex> {
    let a_v = mesh.vertices[triangle[0]];
    let b_v = mesh.vertices[triangle[1]];
    let c_v = mesh.vertices[triangle[2]];

    let mut a = a_v.position;
    let mut b = b_v.position;
    let mut c = c_v.position;

    a = a.rotate(transform.rotation);
    b = b.rotate(transform.rotation);
    c = c.rotate(transform.rotation);

    a += transform.position;
    b += transform.position;
    c += transform.position;

    let conversion_matrix = M3x3::new([b - a, c - a, -1.0 * dir]).inv();

    if conversion_matrix.is_none() {
        return None;
    }

    let res = (start - a) * conversion_matrix.unwrap();

    let t = res.z;
    let v = res.x;
    let w = res.y;

    if t < 0.0 || v < 0.0 || w < 0.0 || v + w > 1.0 {
        return None;
    }

    if let Some(max_distance) = max_distance {
        // Apparently making this a one liner is unstable; so 2 nested ifs are required
        if t > max_distance {
            return None;
        }
    }

    let u = 1.0 - (v + w);
    let v = Vertex::interpolate((a_v, u), (b_v, v), (c_v, w));

    Some(v)
}
