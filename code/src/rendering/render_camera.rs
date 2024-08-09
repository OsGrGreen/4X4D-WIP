use crate::world::layout::Point;

use super::render::{self};
use glam::{vec4, Mat4, Vec3, Vec3A, Vec4};


pub struct RenderCamera{
    camera_pos: Vec3,
    camera_target: Vec3,
    camera_direction: Vec3,
}

const SQRT3:f32 = 1.7320508;

impl RenderCamera{

    pub fn new(startPos: Vec3, target:Vec3) -> RenderCamera{

        let direction = (startPos-target).normalize();

        RenderCamera{camera_pos:startPos, camera_target:target,camera_direction:direction}
    }

    fn update_direction(&mut self){
        self.camera_direction = (self.camera_pos-self.camera_target).normalize();
    }

    pub fn r#move(&mut self, direction:Vec3){
        self.camera_pos += direction;
        self.update_direction();
    }

    pub fn r#move_target(&mut self, direction:Vec3){
        self.camera_target += direction;
        self.update_direction();
    }

    pub fn r#change_target(&mut self, new_target:Vec3){
        self.camera_target = new_target;
        self.update_direction();
    }

    pub fn get_view_matrix(&self, world_up: Vec3) -> Mat4{
        let right = world_up.cross(self.camera_direction).normalize();
        let up = self.camera_direction.cross(right);
        let coords = Mat4::from_cols(right.extend(0.0), up.extend(0.0), self.camera_direction.extend(0.0), vec4(0.0, 0.0, 0.0, 1.0));
        let posistion_mat = Mat4::from_diagonal(Vec4::ONE).add_mat4(&Mat4::from_cols(Vec4::ZERO, Vec4::ZERO, Vec4::ZERO, self.camera_pos.extend(0.0)));
        return coords*posistion_mat;
    }

    pub fn look_at_glm(pos:Vec3, target:Vec3,up:Vec3) -> Mat4{
        let f = (target-pos).normalize();
        let mut u = up.normalize();
        let s = (f.cross(u)).normalize();
        u = s.cross(f);

        let rotation = Mat4::from_cols(
            s.extend(0.0),
            u.extend(0.0),
            f.extend(0.0),
            Vec4::new(0.0, 0.0, 0.0, 1.0),
        );    
        let translation = Mat4::from_translation(-pos);

        rotation * translation

    }

    pub fn look_at(pos:Vec3,target:Vec3,world_up:Vec3) -> Mat4{
        let f: Vec3 = target.normalize_or(Vec3::new(0.0, 0.0, 1.0));
        let s = world_up.cross(f);
        let s_norm = s.normalize_or(Vec3::new(1.0,0.0,0.0));

        let u = f.cross(s_norm);

        let p = Vec3::new(
            -pos.dot(s_norm),
            -pos.dot(u),
            -pos.dot(f),
        );

        Mat4::from_cols_array(&[
            s_norm.x, u.x, f.x, 0.0,
            s_norm.y, u.y, f.y, 0.0,
            s_norm.z, u.z, f.z, 0.0,
            p.x, p.y, p.z, 1.0,
        ])
    }

    pub fn view_matrix(position: &[f32; 3], direction: &[f32; 3], up: &[f32; 3]) -> [[f32; 4]; 4] {
        let f = {
            let f = direction;
            let len = f[0] * f[0] + f[1] * f[1] + f[2] * f[2];
            let len = len.sqrt();
            [f[0] / len, f[1] / len, f[2] / len]
        };
    
        let s = [up[1] * f[2] - up[2] * f[1],
                 up[2] * f[0] - up[0] * f[2],
                 up[0] * f[1] - up[1] * f[0]];

        let s_norm = {
            let len = s[0] * s[0] + s[1] * s[1] + s[2] * s[2];
            let len = len.sqrt();
            [s[0] / len, s[1] / len, s[2] / len]
        };

    
        let u = [f[1] * s_norm[2] - f[2] * s_norm[1],
                 f[2] * s_norm[0] - f[0] * s_norm[2],
                 f[0] * s_norm[1] - f[1] * s_norm[0]];
    
        let p = [-position[0] * s_norm[0] - position[1] * s_norm[1] - position[2] * s_norm[2],
                 -position[0] * u[0] - position[1] * u[1] - position[2] * u[2],
                 -position[0] * f[0] - position[1] * f[1] - position[2] * f[2]];
    
        [
            [s_norm[0], u[0], f[0], 0.0],
            [s_norm[1], u[1], f[1], 0.0],
            [s_norm[2], u[2], f[2], 0.0],
            [p[0], p[1], p[2], 1.0],
        ]
    }

}