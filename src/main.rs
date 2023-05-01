use std::f64::consts::PI;

use image::Rgba;

use crate::star::Star;
mod coords;
mod star;

fn main() {
    use image::ImageBuffer;
    use noise::{NoiseFn, OpenSimplex};
    let width = 4096;
    let height = 4096;
    let noise = OpenSimplex::new(1);
    let noise_ref = &noise;
    let dark = Rgba([18, 18, 38, 255]);
    let light = Rgba([49, 50, 74, 255]);
    let mut buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let width_float = width as f64;
        let height_float = height as f64;
        let polar = (x as f64 / width_float - 0.5) * 2.0 * PI;
        let azimuthal = (y as f64 / height_float - 0.5) * PI;
        let radius = 2.0;
        let sphere = coords::Spherical {
            azimuthal,
            polar,
            radius,
        };
        let total = noise_ref.get(sphere.to_rectangular().arr());
        blend_pixel(total, light, dark)
    });
    let star = Star::new();
    for _ in 0..100 {
        star.draw_random_star(&mut buffer);
    }
    buffer.save("skymap.png").unwrap();
}

fn blend_pixel(mix: f64, c1: Rgba<u8>, c2: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        blend_element(mix, c1.0[0], c2.0[0]),
        blend_element(mix, c1.0[1], c2.0[1]),
        blend_element(mix, c1.0[2], c2.0[2]),
        blend_element(mix, c1.0[3], c2.0[3]),
    ])
}

fn blend_element(mix: f64, e1: u8, e2: u8) -> u8 {
    let e1 = e1 as f64;
    let e2 = e2 as f64;
    (e1 * mix + e2 * (1.0 - mix)) as u8
}
