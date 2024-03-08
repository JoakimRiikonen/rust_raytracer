use std::fs;

use rust_ray_tracer::{color::Color, common::Common, hittable::Hittable, hittable_list::HittableList, interval::Interval, ray::Ray, sphere::Sphere, vec3::{Point3, Vec3}};


fn main() {
    let filename = "testimg.ppm";

    // Image

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;
    // Calculate the image height, and ensure it't at least 1
    let image_height = (image_width as f64 / aspect_ratio) as i64;
    let image_height = if image_height < 1 { 1 } else { image_height };

    // World

    let sphere1 = Box::new(Sphere::new(
        Point3::new(0.0,0.0,-1.0),
        0.5));
    let sphere2 = Box::new(Sphere::new(
        Point3::new(0.0,-100.5,-1.0),
        100.0
    ));

    let world = HittableList {
        objects: vec![sphere1, sphere2]
    };

    // Camera

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (image_width as f64 / image_height as f64);
    let camera_center = Point3::new(0.0, 0.0, 0.0);

    // Calculate vectors across the horizontal and down the vertical viewport edges
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel
    let pixel_delta_u = &viewport_u / image_width as f64;
    let pixel_delta_v = &viewport_v / image_height as f64;

    // Calculate location of upper-left pixel
    let viewport_upper_left = &camera_center
        - &Vec3::new(0.0, 0.0, focal_length)
        - &viewport_u / 2.0
        - &viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + (&pixel_delta_u + &pixel_delta_v) * 0.5;

    let mut output = format!("P3\n{} {}\n255\n", image_width, image_height);

    for j in 0..image_height {
        print!("\rScanlines remaining: {}  ", image_height-j);
        for i in 0..image_width {
            let pixel_center = &pixel00_loc + &(&pixel_delta_u * i as f64) + (&pixel_delta_v * j as f64);
            let ray_direction = &pixel_center - &camera_center;

            let r = Ray::new(&camera_center, &ray_direction);

            let pixel_color = ray_color(&r, Box::new(&world));

            let row = format!("{}\n", pixel_color.to_color_string());

            output.push_str(&row);
        }
    }

    print!("\rDone!.                    \n");
    _ = fs::write(filename, output);
}

fn ray_color<'a>(ray: &Ray, world: Box<dyn Hittable + 'a>) -> Color {
    let interval = Interval::new_from_range(0.0, Common::INFINITY);
    let (hit, rec) = world.hit(ray, &interval);
    if hit {
        return rec.unwrap().normal + Color::new(1.0, 1.0, 1.0) * 0.5;
    }

    let unit_direction = ray.direction.unit_vector();
    let a = 0.5 * (unit_direction.y + 1.0);
    Color::new(1.0, 1.0, 1.0) * (1.0 - a) + Color::new(0.5, 0.7, 1.0) * a
}