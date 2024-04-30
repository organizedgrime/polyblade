#![allow(dead_code)]
use glam::*;
use iced::widget::shader::wgpu;
use std::f32::consts::PI;

#[rustfmt::skip]
#[allow(unused)]
pub const OPENGL_TO_WGPU_MATRIX: Mat4 = mat4(
    vec4( 1.0, 0.0, 0.0, 0.0),
    vec4(0.0, 1.0, 0.0, 0.0),
    vec4(0.0, 0.0, 0.5, 0.0),
    vec4(0.0, 0.0, 0.5, 1.0),
);
pub fn create_view(camera_position: Vec3, look_direction: Vec3, up_direction: Vec3) -> Mat4 {
    Mat4::look_at_rh(camera_position, look_direction, up_direction)
}

pub fn create_view_projection(
    camera_position: Vec3,
    look_direction: Vec3,
    up_direction: Vec3,
    aspect: f32,
) -> (Mat4, Mat4, Mat4) {
    // construct view matrix
    let view_mat = Mat4::look_at_rh(camera_position, look_direction, up_direction);

    // construct projection matrix
    let project_mat: Mat4 =
        OPENGL_TO_WGPU_MATRIX * Mat4::perspective_lh(2.0 * PI / 5.0, aspect, 0.1, 100.0);

    // contruct view-projection matrix
    let view_project_mat = project_mat * view_mat;

    // return various matrices
    (view_mat, project_mat, view_project_mat)
}

pub fn create_transforms(translation: [f32; 3], rotation: [f32; 3], scaling: [f32; 3]) -> Mat4 {
    // create transformation matrices
    let trans_mat =
        Mat4::from_translation(Vec3::new(translation[0], translation[1], translation[2]));
    let rotate_mat_x = Mat4::from_rotation_x(rotation[0]);
    let rotate_mat_y = Mat4::from_rotation_y(rotation[1]);
    let rotate_mat_z = Mat4::from_rotation_z(rotation[2]);
    //let scale_mat = Mat4::from_nonuniform_scale(scaling[0], scaling[1], scaling[2]);

    // combine all transformation matrices together to form a final transform matrix: model matrix
    let model_mat = trans_mat * rotate_mat_z * rotate_mat_y * rotate_mat_x;

    // return final model matrix
    model_mat
}
