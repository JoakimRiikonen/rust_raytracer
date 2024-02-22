use std::fs;

use rust_ray_tracer::color::Color;


fn main() {
    let filename = "testimg.ppm";
    let image_width = 256;
    let image_height = 256;

    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        print!("\rScanlines remaining: {}  ", image_height-j);
        for i in 0..image_width {
            let color = Color::new(
                (i as f64) / (image_width-1) as f64,
                (j as f64) / (image_height-1) as f64,
                0.0);

            let row = format!("{}\n", color.to_color_string());

            output.push_str(&row);
        }
    }

    print!("\rDone!.                    \n");
    _ = fs::write(filename, output);
}