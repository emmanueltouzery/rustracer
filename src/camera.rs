use crate::{v3color::*, shapes::*};

use std::f32::consts::PI;
use rand::{prelude as random, Rng};

pub struct Camera {
    origin: V3,
    lower_left_corner: V3,
    horizontal: V3,
    vertical: V3,
    u: V3,
    v: V3,
    lens_radius: f32
}

fn random_in_unit_disk() -> V3 {
    let mut rng = random::thread_rng();
    let mut p;
    let unit = V3 { x: 1.0, y: 1.0, z: 0.0};
    loop {
        p = 2.0 * V3 { 
            x: rng.gen::<f32>(),
            y: rng.gen::<f32>(),
            z: 0.0
        } - unit;
        if p.squared_length() < 1.0 { break p; }
    }
}

impl Camera {
    pub fn new(look_from: &V3, look_at: &V3,
               vup: &V3, vert_fov_deg: f32, aspect: f32,
               aperture: f32, focus_dist: f32) -> Camera {
        let theta = vert_fov_deg*PI/180.0;
        let half_height = f32::tan(theta/2.0)*focus_dist;
        let half_width = aspect * half_height;
        let w = (look_from - look_at).unit();
        let u = V3::cross(&vup, &w).unit();
        let v = V3::cross(&w, &u);
        Camera {
            origin: *look_from,
            lower_left_corner: look_from - half_width*u - half_height*v - focus_dist*w,
            u, v,
            horizontal: 2.0*half_width*u,
            vertical: 2.0*half_height*v,
            lens_radius: aperture/2.0
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius*random_in_unit_disk();
        let offset = self.u*rd.x + self.v*rd.y;
        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner 
                + s*self.horizontal + t*self.vertical - self.origin - offset
        }
    }
}