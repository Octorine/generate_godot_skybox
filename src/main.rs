fn main() {
    use perlin_noise::PerlinNoise;
    use png::Encoder;
    use std::fs::File;
    use std::io::BufWriter;
    let width = 4096;
    let height = 4096;
    let file = File::create("test.png").unwrap();
    let writer = BufWriter::new(file);
    let mut png = Encoder::new(writer, width, height);
    let noise = PerlinNoise::new();
    let noise_ref = &noise;
    png.set_color(png::ColorType::Grayscale);
    let mut writer = png.write_header().unwrap();

    let data: Vec<u8> = (0..width)
        
        .flat_map(|i| {
            (0..height).map(move |j| {
                let ifloat = i as f64;
                let jfloat = j as f64;
                let ftotal = noise_ref.get2d([ifloat / 256.0, jfloat / 256.0]);
                (ftotal * 256.0).floor() as u8
            })
        })
        .collect();

    writer.write_image_data(&data).unwrap();
}
