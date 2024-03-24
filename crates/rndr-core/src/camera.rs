use std::collections::HashMap;

use rndr_math::{matrix::M3x3, transform::Transform, vector::V3};

use crate::prelude::Object;

pub struct Camera {
    pub transform: Transform,

    /// Wether the camera should use perspective projectino
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

    last_projection_matrix: Option<M3x3>,
}

impl Camera {
    pub fn new(perspective: bool) -> Camera {
        Camera {
            perspective,
            display_surface_offset: if perspective {
                Some(V3::new(0.0, 0.0, -10.0))
            } else {
                None
            },
            last_projection_matrix: None,
            near_plane: 0.1,
            zero_threshold: 0.01,
            transform: Transform {
                rotation: V3::new(0.0, 0.0, 270.0),
                ..Default::default()
            },
        }
    }

    /// Will generate a projection matix use it for projection and save it for possible future use
    pub fn project_point(
        &mut self,
        cache: &mut HashMap<usize, V3>,
        shape: &Object,
        index: usize,
    ) -> V3 {
        if cache.contains_key(&index) {
            return cache[&index];
        }

        let projection_matrix = self.get_and_save_projection_matrix();
        let px = self.project_point_with_matrix(shape, index, &projection_matrix);

        cache.insert(index, px);
        px
    }

    /// Will project a point reusing the previous projection matrix. Note that it won't be accurate
    /// if the camera context like position/rotation changed. Only use this function after calling
    /// `project_point` within the same frame context.
    pub unsafe fn project_point_unsafe(
        &self,
        cache: &mut HashMap<usize, V3>,
        shape: &Object,
        index: usize,
    ) -> V3 {
        if cache.contains_key(&index) {
            return cache[&index];
        }

        let px =
            self.project_point_with_matrix(shape, index, &self.last_projection_matrix.unwrap());

        cache.insert(index, px);
        px
    }

    fn get_and_save_projection_matrix(&mut self) -> M3x3 {
        let cam_fwd = self.transform.fwd();
        let cam_right = self.transform.right();
        let cam_up = self.transform.up();

        let projection_matrix = M3x3::new([
            V3::new(cam_right.x, cam_up.x, cam_fwd.x),
            V3::new(cam_right.y, cam_up.y, cam_fwd.y),
            V3::new(cam_right.z, cam_up.z, cam_fwd.z),
        ]);
        self.last_projection_matrix = Some(projection_matrix);
        projection_matrix
    }

    fn project_point_with_matrix(
        &self,
        shape: &Object,
        index: usize,
        projection_matrix: &M3x3,
    ) -> V3 {
        let mut point = shape.vertices[index];
        point.rotate(shape.transform.rotation);
        point += shape.transform.position;

        let point = point.relative_to(&self.transform.position);

        let mut px = *projection_matrix * point;

        if self.perspective && px.z.abs() > self.zero_threshold {
            let display_surface_offset = self.display_surface_offset.unwrap();
            px.x = display_surface_offset.z / px.z * px.x + display_surface_offset.x;
            px.y = display_surface_offset.z / px.z * px.y + display_surface_offset.y;
        }

        px
    }
}
