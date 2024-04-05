use rndr_core::scene::SceneContext;
use rndr_math::prelude::{M3x3, V3};

pub fn raycast(start: V3, dir: V3, scene_context: &SceneContext) -> Option<V3> {
    for obj in &scene_context.objects {
        for triangle in &obj.triangles {
            let a = obj.vertices[triangle[0]].position;
            let b = obj.vertices[triangle[1]].position;
            let c = obj.vertices[triangle[2]].position;

            let conversion_matrix = M3x3::new([b - a, c - a, -1.0 * dir]).inv();
            if conversion_matrix.is_none() {
                continue;
            }

            let res = (start - a) * conversion_matrix.unwrap();

            let t = res.z;

            let v = res.x;
            let w = res.y;

            if v < 0.0 || w < 0.0 || v + w > 1.0 {
                continue;
            }

            let u = 1.0 - (v + w);

            return Some(a * u + b * v + c * w);
        }
    }

    None
}
