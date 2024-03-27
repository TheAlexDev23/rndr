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

impl Camera {
    pub fn new(perspective: bool) -> Camera {
        Camera {
            perspective,
            display_surface_offset: if perspective {
                Some(V3::new(0.0, 0.0, -100.0))
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

    pub fn get_projection_matrix(&mut self) -> M3x3 {
        let (fwd, right, up) = self.transform.get_orientations_in_bulk();

        M3x3::new([
            V3::new(right.x, up.x, fwd.x),
            V3::new(right.y, up.y, fwd.y),
            V3::new(right.z, up.z, fwd.z),
        ])
    }

    pub fn project_point(&mut self, projection_matrix: M3x3, shape: &Object, index: usize) -> V3 {
        let mut point = shape.vertices[index].position;

        point = point.rotate(shape.transform.rotation);

        let transformed_pos = shape
            .transform
            .position
            .relative_to(&self.transform.position);

        point += transformed_pos;

        let mut px = projection_matrix * point;

        if self.perspective && px.z.abs() > self.zero_threshold {
            let display_surface_offset = self.display_surface_offset.unwrap();
            px.x = display_surface_offset.z / px.z * px.x + display_surface_offset.x;
            px.y = display_surface_offset.z / px.z * px.y + display_surface_offset.y;
        }

        px
    }
}
