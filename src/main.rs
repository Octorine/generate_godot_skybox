use std::f64::consts::PI;

use image::Rgb;
mod coords;

fn main() {
    use image::ImageBuffer;
    use noise::{NoiseFn, OpenSimplex};
    let width = 4096;
    let height = 4096;
    let noise = OpenSimplex::new(1);
    let noise_ref = &noise;
    let dark = Rgb([18, 18, 38]);
    let light = Rgb([49, 50, 74]);
    let buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let width_float = width as f64;
        let height_float = height as f64;
        let theta = (x as f64 / width_float - 0.5) * 2.0 * PI;
        let phi = (y as f64 / height_float - 0.5) * PI;
        let rho = 2.0;
        let total = noise_ref.get(coords::sphere_to_cart(theta, phi, rho));
        blend_pixel(total, light, dark)
    });
    buffer.save("skymap.png").unwrap();
}

fn blend_pixel(mix: f64, c1: Rgb<u8>, c2: Rgb<u8>) -> Rgb<u8> {
        Rgb([blend_element(mix, c1.0[0], c2.0[0]),
        blend_element(mix, c1.0[1], c2.0[1]),
        blend_element(mix, c1.0[2], c2.0[2])])
}

fn blend_element(mix: f64, e1: u8, e2: u8) -> u8 {
    let e1 = e1 as f64;
    let e2 = e2 as f64;
    (e1 * mix + e2 * (1.0 - mix)) as u8
}
