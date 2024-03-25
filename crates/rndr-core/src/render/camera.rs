use std::collections::HashMap;

use rndr_math::{matrix::M3x3, transform::Transform, vector::V3};

use crate::prelude::Object;

pub struct Camera {
    pub transform: Transform,

    /// Wether the camera should use perspective projection
    pub perspective: bool,

    /// Displacement of the display surface relative to the camera pinhole.
    /// None if not using perspective
    pub display_surface_offset: Option<V3>,

    /// Projected screen points with z smaller than this value won't be shown
    pub near_plane: f32,

    /// If |z| < `this value` where z is the projected pixel's z,
    /// projection won't be applied to that pixel. This is done in order to
    /// prevent extremely small z values from being used in projection calculations,
    /// outputing lines millions of pixels long and affecting performance.
    pub zero_threshold: f32,
}

const PROJECTION_MATRIX: M3x3 = M3x3 {
    columns: [
        V3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
        V3 {
            x: -1.0,
            y: 0.0,
            z: 0.0,
        },
        V3 {
            x: 0.0,
            y: 1.0,
            z: 0.0,
        },
    ],
};

impl Camera {
    pub fn new(perspective: bool) -> Camera {
        Camera {
            perspective,
            display_surface_offset: if perspective {
                Some(V3::new(0.0, 0.0, -10.0))
            } else {
                None
            },
            near_plane: 0.1,
            zero_threshold: 0.01,
            transform: Transform {
                rotation: V3::new(0.0, 0.0, 0.0),
                ..Default::default()
            },
        }
    }

    pub fn project_point(
        &mut self,
        cache: &mut HashMap<usize, V3>,
        shape: &Object,
        index: usize,
    ) -> V3 {
        if cache.contains_key(&index) {
            return cache[&index];
        }

        let mut point = shape.vertices[index].position;

        // Rotating by shape.transform.rotation - self.transform.rotation apparently isn't the same
        point = point.rotate(shape.transform.rotation);
        point = point.rotate(-1.0 * self.transform.rotation);

        let transformed_pos = shape
            .transform
            .position
            .relative_to(&self.transform.position)
            .rotate(-1.0 * self.transform.rotation);

        point += transformed_pos;

        let mut px = PROJECTION_MATRIX * point;

        if self.perspective && px.z.abs() > self.zero_threshold {
            let display_surface_offset = self.display_surface_offset.unwrap();
            px.x = display_surface_offset.z / px.z * px.x + display_surface_offset.x;
            px.y = display_surface_offset.z / px.z * px.y + display_surface_offset.y;
        }

        cache.insert(index, px);
        px
    }
}
