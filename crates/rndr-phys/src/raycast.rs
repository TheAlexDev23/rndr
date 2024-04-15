use std::any::TypeId;

use rndr_core::default_components::{render::MeshRenderable, Transform};

use rndr_core::object::ObjectManager;
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
            let mesh = match obj.component(TypeId::of::<MeshRenderable>()) {
                Some(obj) => obj.downcast_ref::<MeshRenderable>().unwrap(),
                None => continue,
            };
            let transform = match obj.component(TypeId::of::<Transform>()) {
                Some(obj) => obj.downcast_ref::<Transform>().unwrap(),
                None => continue,
            };

            for triangle in &mesh.triangles {
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

                let conversion_matrix = M3x3::new([b - a, c - a, -1.0 * self.dir]).inv();
                if conversion_matrix.is_none() {
                    continue;
                }

                let res = (self.start - a) * conversion_matrix.unwrap();

                let t = res.z;

                let v = res.x;
                let w = res.y;

                if t < 0.0 || v < 0.0 || w < 0.0 || v + w > 1.0 {
                    continue;
                }

                if let Some(max_distance) = self.max_distance {
                    // Apparently making this a one liner is unstable; so 2 nested ifs are required
                    if t > max_distance {
                        continue;
                    }
                }

                let u = 1.0 - (v + w);

                return Some(Vertex::interpolate((a_v, u), (b_v, v), (c_v, w)));
            }
        }

        None
    }
}
