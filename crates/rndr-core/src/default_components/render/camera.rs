use std::any::TypeId;

use rndr_math::prelude::{M3x3, Vertex, V3};

use crate::default_components::Transform;
use crate::object::Component;

#[derive(Debug)]
pub struct Camera {
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

impl Component for Camera {
    fn get_type(&self) -> std::any::TypeId {
        TypeId::of::<Camera>()
    }
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
        }
    }

    pub fn get_projection_matrix(&self, camera_transform: &Transform) -> M3x3 {
        let (fwd, right, up) = camera_transform.get_orientations_in_bulk();

        M3x3::new([
            V3::new(right.x, up.x, fwd.x),
            V3::new(right.y, up.y, fwd.y),
            V3::new(right.z, up.z, fwd.z),
        ])
    }

    pub fn project_point(
        &self,
        projection_matrix: M3x3,
        mut vertex: Vertex,
        object_transform: &Transform,
        camera_transform: &Transform,
    ) -> V3 {
        object_transform.apply_to_vertex(&mut vertex);

        vertex.position -= camera_transform.position;

        let mut px = projection_matrix * vertex.position;

        if self.perspective && px.z > 0.0 && px.z > self.zero_threshold {
            let display_surface_offset = self.display_surface_offset.unwrap();
            px.x = display_surface_offset.z / px.z * px.x + display_surface_offset.x;
            px.y = display_surface_offset.z / px.z * px.y + display_surface_offset.y;
        }

        px
    }
}
