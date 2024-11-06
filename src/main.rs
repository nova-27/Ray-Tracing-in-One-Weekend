use ray_tracing_in_one_weekend::Color;

fn main() {
    const IMAGE_WIDTH: i32 = 256;
    const IMAGE_HEIGHT: i32 = 256;

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprintln!("Scanlines remaining: {}", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_color = Color::new(
                (i as f64) / ((IMAGE_WIDTH - 1) as f64),
                (j as f64) / ((IMAGE_HEIGHT - 1) as f64),
                0.25,
            );

            pixel_color.write_color();
        }
    }

    eprintln!("Done.")
}
