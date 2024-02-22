use std::fs;

fn main() {
    let filename = "testimg.ppm";
    let image_width = 256;
    let image_height = 256;

    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        print!("\rScanlines remaining: {}  ", image_height-j);
        for i in 0..image_width {
            let r: f32 = (i as f32) / (image_width-1) as f32;
            let g: f32 = (j as f32) / (image_height-1) as f32;
            let b: f32 = 0.0;

            let ir: i32 = (255.999 * r) as i32;
            let ig: i32 = (255.999 * g) as i32;
            let ib: i32 = (255.999 * b) as i32;

            let row = format!("{} {} {}\n", ir, ig, ib);

            output.push_str(&row);
        }
    }

    print!("\rDone!.                    \n");
    _ = fs::write(filename, output);
}
