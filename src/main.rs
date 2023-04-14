use std::f64::consts::PI;

fn main() {
    use image::ImageBuffer;
    use perlin_noise::PerlinNoise;

    let width = 4096;
    let height = 4096;

    let noise = PerlinNoise::new();
    let noise_ref = &noise;

    let buffer = ImageBuffer::from_fn(width, height, |x, y| {
        let widthfloat = width as f64;
        let heightfloat = height as f64;
        let theta = (x as f64 / widthfloat - 0.5) * PI * 2.0;
        let phi = (y as f64 / heightfloat - 0.5) * PI * 2.0;
        let rho = 1.0;
        let xfloat = rho * theta.sin() * phi.cos();
        let zfloat = rho * theta.sin() * phi.sin();
        let yfloat = rho * theta.cos();

        let ftotal = noise_ref.get3d([xfloat, yfloat, zfloat]);

        image::Luma([(ftotal * 255.0).floor() as u8])
    });
    buffer.save("skymap.png").unwrap();
}
