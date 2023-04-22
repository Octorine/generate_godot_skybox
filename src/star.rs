use std::f32::consts::PI;

use image::imageops::overlay;
use image::io::Reader;
use image::{ImageBuffer, Rgba};
use imageproc::geometric_transformations::*;
use rand::prelude::*;

use crate::coords;

type Img = ImageBuffer<Rgba<u8>, Vec<u8>>;
pub struct Star {
    img: Img,
}

impl Star {
    pub fn new() -> Star {
        Star {
            img: Reader::open("star.png")
                .unwrap()
                .decode()
                .unwrap()
                .into_rgba8(),
        }
    }
    pub fn blit(&self, canvas: &mut Img, x: i64, y: i64, theta: f32, scale: f32) {
        let cw = self.img.width() as f32 * 0.5;
        let ch = self.img.height() as f32 * 0.5;

        let proj = Projection::translate(cw, ch)
            * Projection::scale(scale, scale)
            * Projection::rotate(theta)
            * Projection::translate(-cw, -ch);
        overlay(
            canvas,
            &warp(&self.img, &proj, Interpolation::Nearest, Rgba([0, 0, 0, 1])),
            x,
            y,
        );
    }
    pub fn draw_random_star(&self, canvas: &mut Img) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let z = rng.gen_range(-1.0..1.0);
        let [theta, phi, rho] = coords::cart_to_sphere(x, y, z);
        let theta = theta * canvas.height() as f64 / (PI as f64);
        let theta = theta.round() as i64;
        let phi = phi * canvas.width() as f64 / PI as f64;
        let phi = phi as i64;
        self.blit(
            canvas,
            theta,
            phi,
            rng.gen_range(0.0..PI),
            rng.gen_range(0.25..1.0),
        )
    }
}
