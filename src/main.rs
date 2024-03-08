use rust_ray_tracer::{camera::Camera, hittable_list::HittableList, sphere::Sphere, vec3::Point3};


fn main() {
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

    let aspect_ratio = 16.0 / 9.0;
    let image_width: i64 = 400;
    let samples_per_pixel = 100;
    let max_depth = 10;
    let filename = "testimg.ppm";
    
    let camera = Camera::new(aspect_ratio, image_width,
        samples_per_pixel, max_depth, filename);
    camera.render(Box::new(&world));
}