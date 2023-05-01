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

        let distort_projection = &Projection::from_matrix([
            1.0,
            0.0,
            0.0,
            (1.0 * y as f32 * PI / canvas.height() as f32).sin(),
            1.0,
            0.0,
            0.0,
            0.0,
            1.0,
        ])
        .unwrap();

        let img = warp(
            &self.img,
            &(Projection::translate(cw, ch)
                * Projection::scale(scale, scale)
                * Projection::rotate(theta)
                * Projection::translate(-cw, -ch)),
            Interpolation::Nearest,
            Rgba([0, 0, 0, 1]),
        );
        let img = stretch(&img, canvas.width());
        overlay(canvas, &img, x, y);
    }
    pub fn draw_random_star(&self, canvas: &mut Img) {
        let mut rng = thread_rng();
        let x = rng.gen_range(-1.0..1.0);
        let y = rng.gen_range(-1.0..1.0);
        let z = rng.gen_range(-1.0..1.0);
        let spherical_coords = coords::Rectangular { x, y, z }.to_spherical();
        let theta = spherical_coords.azimuthal * canvas.height() as f64 / PI as f64
            + canvas.height() as f64 * 0.5;
        let theta = theta.round() as i64;
        let phi = spherical_coords.polar * canvas.width() as f64 / PI as f64;
        let phi = phi as i64;
        self.blit(
            canvas,
            phi,
            theta,
            rng.gen_range(0.0..PI),
            rng.gen_range(0.25..0.5),
        )
    }
}
pub fn stretch(
    img: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    new_width: u32,
) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
    image::ImageBuffer::from_fn(new_width, img.height(), |x, y| {
        let left = (new_width - img.width()) / 2;
        let right = (new_width + img.width()) / 2;
        if x < left || x >= right {
            Rgba([0, 0, 0, 0])
        } else {
            *img.get_pixel(x - left, y)
        }
    })
}
