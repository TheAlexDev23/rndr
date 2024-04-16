use rndr_core::default_components::{render::MeshRenderable, Transform};

use rndr_core::object::{Object, ObjectManager};
use rndr_math::prelude::{M3x3, Vertex, V3};

pub struct HitInfo {
    pub vertex: Vertex,
    pub distance: f32,
}

pub struct Ray<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub objects: &'a ObjectManager,
}

impl<'a> Ray<'a> {
    pub fn cast(&self) -> Option<HitInfo> {
        let mut intersects = Vec::new();
        for obj in self.objects.objects_iter() {
            let mesh = match obj.component::<MeshRenderable>() {
                Some(mesh) => mesh,
                None => continue,
            };
            let transform = obj.component::<Transform>().unwrap();
            intersects.extend(find_all_mesh_ray_intersections(
                mesh,
                transform,
                self.dir,
                self.start,
                self.max_distance,
            ))
        }

        intersects
            .into_iter()
            .reduce(|a, b| if a.distance < b.distance { a } else { b })
    }
}

pub struct ObjectIntersectionRay<'a> {
    pub start: V3,
    pub dir: V3,
    pub max_distance: Option<f32>,

    pub object: &'a Object,
}

impl<'a> ObjectIntersectionRay<'a> {
    pub fn cast(&self) -> Vec<HitInfo> {
        let mesh = self.object.component::<MeshRenderable>().unwrap();
        let transform = self.object.component::<Transform>().unwrap();
        find_all_mesh_ray_intersections(mesh, transform, self.dir, self.start, self.max_distance)
    }
}

fn find_all_mesh_ray_intersections(
    mesh: &MeshRenderable,
    transform: &Transform,
    dir: V3,
    start: V3,
    max_distance: Option<f32>,
) -> Vec<HitInfo> {
    let mut ret = Vec::new();
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

        let conversion_matrix = M3x3::new([b - a, c - a, -1.0 * dir]).inv();

        if conversion_matrix.is_none() {
            continue;
        }

        let res = (start - a) * conversion_matrix.unwrap();

        let t = res.z;
        let v = res.x;
        let w = res.y;

        if t < 0.0 || v < 0.0 || w < 0.0 || v + w > 1.0 {
            continue;
        }

        if let Some(max_distance) = max_distance {
            // Apparently making this a one liner is unstable; so 2 nested ifs are required
            if t > max_distance {
                continue;
            }
        }

        let u = 1.0 - (v + w);
        let vertex = Vertex::interpolate((a_v, u), (b_v, v), (c_v, w));

        ret.push(HitInfo {
            vertex,
            distance: t,
        });
    }
    ret
}
