fn main() {
    use perlin_noise::PerlinNoise;
    use image::ImageBuffer;

    let width = 4096;
    let height = 4096;

    let noise = PerlinNoise::new();
    let noise_ref = &noise;

    let buffer = ImageBuffer::from_fn(height, width, |x, y| {
                let xfloat = x as f64;
                let yfloat = y as f64;
                let ftotal = noise_ref.get2d([xfloat / 256.0, yfloat / 256.0]);
                image::Luma([(ftotal * 256.0).floor() as u8])
    });

    buffer.save("skymap.png").unwrap();
}
