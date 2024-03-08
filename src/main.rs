use rust_ray_tracer::{camera::Camera, color::Color, hittable_list::HittableList, material::{Lambertian, Metal}, sphere::Sphere, vec3::Point3};


fn main() {
	// World

	let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
	let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
	let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
	let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 0.1);

	let sphere_ground = Box::new(Sphere::new(
		Point3::new(0.0,-100.5,-1.0),
		100.0, Box::new(material_ground)));
	let sphere_center = Box::new(Sphere::new(
		Point3::new(0.0,0.0,-1.0),
		0.5, Box::new(material_center)));
	let sphere_left = Box::new(Sphere::new(
		Point3::new(-1.0,0.0,-1.0),
		0.5, Box::new(material_left)));
	let sphere_right = Box::new(Sphere::new(
		Point3::new(1.0,0.0,-1.5),
		0.5, Box::new(material_right)));

	let world = HittableList {
		objects: vec![sphere_ground, sphere_center, sphere_left, sphere_right]
	};

	// Camera

	let aspect_ratio = 16.0 / 9.0;
	let image_width: i64 = 400;
	let samples_per_pixel = 100;
	let max_depth = 10;
	let filename = "testimg.ppm";
	
	let camera = Camera::new(aspect_ratio, image_width,
		samples_per_pixel, max_depth,  filename);
	camera.render(Box::new(&world));
}