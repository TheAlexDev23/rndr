use std::sync::Arc;

use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use rndr_math::prelude::{M3x3, Vertex};

use crate::object::Object;
use crate::render::FragData;
use crate::{
    default_components::{
        render::{Camera, MeshRenderable},
        Transform,
    },
    prelude::PixelGrid,
};

pub struct MeshRendererSystem;

impl MeshRendererSystem {
    pub fn render_mesh_object(
        &mut self,
        pixel_grid: &mut PixelGrid,
        width: u32,
        height: u32,
        object: &Object,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let projection_matrix = camera.get_projection_matrix(camera_transform);

        let camera = Arc::from(camera);

        let object_transform = object.component::<Transform>();
        let object_mesh = object.component::<MeshRenderable>();

        let pixel_changes: Vec<_> = object_mesh
            .triangles
            .par_iter()
            .map(|triangle| {
                self.render_triangle(
                    width,
                    height,
                    &object_mesh,
                    &object_transform,
                    &camera,
                    &camera_transform,
                    &projection_matrix,
                    triangle,
                )
            })
            .collect();

        for pixel_change in pixel_changes {
            for pixel in pixel_change {
                pixel_grid.set_pixel(pixel.0, pixel.1, pixel.2);
            }
        }
    }

    fn render_triangle(
        &self,
        width: u32,
        height: u32,
        object_mesh: &MeshRenderable,
        object_transform: &Transform,
        camera: &Camera,
        camera_transform: &Transform,
        projection_matrix: &M3x3,
        triangle: &[usize; 3],
    ) -> Vec<(u32, u32, (f32, [u8; 3]))> {
        let first_projected = camera.project_point(
            *projection_matrix,
            object_mesh.vertices[triangle[0]],
            &object_transform,
            &camera_transform,
        );

        let second_projected = camera.project_point(
            *projection_matrix,
            object_mesh.vertices[triangle[1]],
            &object_transform,
            &camera_transform,
        );

        let third_projected = camera.project_point(
            *projection_matrix,
            object_mesh.vertices[triangle[2]],
            &object_transform,
            &camera_transform,
        );

        let near_plane = camera.near_plane;

        if first_projected.z <= near_plane
            || second_projected.z <= near_plane
            || third_projected.z <= near_plane
        {
            return Vec::new();
        }

        // Values for the raster_triangle function to take. As it takes touples and
        // the projected values would still be used in the future
        let first = (first_projected.x, first_projected.y);
        let second = (second_projected.x, second_projected.y);
        let third = (third_projected.x, third_projected.y);

        self.raster_triangle(width, height, first, second, third, |f, s, t| {
            // V means vertex
            let first_v = object_mesh.vertices[triangle[0]];
            let second_v = object_mesh.vertices[triangle[1]];
            let third_v = object_mesh.vertices[triangle[2]];

            let interpolated_vertex =
                Vertex::interpolate((first_v, f), (second_v, s), (third_v, t));

            let interpolated_virtual_color = (
                first_projected.z * f + second_projected.z * s + third_projected.z * t,
                interpolated_vertex.color,
            );

            let mut data = FragData {
                relative_position: interpolated_vertex.position,
                space_position: interpolated_vertex.position + object_transform.position,
                output_pixel: interpolated_virtual_color,
            };

            object_mesh.shader.frag(&mut data);

            data.output_pixel
        })
    }

    fn raster_triangle<F>(
        &self,
        width: u32,
        height: u32,
        first: (f32, f32),
        second: (f32, f32),
        third: (f32, f32),
        pixel: F,
    ) -> Vec<(u32, u32, (f32, [u8; 3]))>
    where
        F: Fn(f32, f32, f32) -> (f32, [u8; 3]),
    {
        let total_area = triangle_area(first, second, third);

        let width = (width / 2) as f32;
        let height = (height / 2) as f32;

        let x_start = first.0.min(second.0).min(third.0).max(-1.0 * width).round() as i32;

        let x_end = first.0.max(second.0).max(third.0).min(width).round() as i32;

        let y_start = first
            .1
            .min(second.1)
            .min(third.1)
            .max(-1.0 * height)
            .round() as i32;

        let y_end = first.1.max(second.1).max(third.1).min(height).round() as i32;

        let mut ret = Vec::with_capacity(total_area as usize);

        for x in x_start..x_end {
            for y in y_start..y_end {
                let pt = (x as f32, y as f32);

                let first_second = triangle_area(pt, first, second);
                let first_third = triangle_area(pt, first, third);
                let third_second = triangle_area(pt, third, second);

                const EPSILON: f32 = 0.01;

                if (first_second + first_third + third_second - total_area).abs() <= EPSILON {
                    let screen_x = x + width as i32;
                    let screen_y = height as i32 + y;

                    let first = third_second / total_area;
                    let second = first_third / total_area;
                    let third = first_second / total_area;

                    ret.push((
                        screen_x as u32,
                        screen_y as u32,
                        pixel(first, second, third),
                    ));
                }
            }
        }

        ret
    }
}

fn triangle_area(p1: (f32, f32), p2: (f32, f32), p3: (f32, f32)) -> f32 {
    0.5 * ((p1.0 * (p2.1 - p3.1) + p2.0 * (p3.1 - p1.1) + p3.0 * (p1.1 - p2.1)).abs())
}
