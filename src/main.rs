use rust_ray_tracer::{camera::Camera, color::Color, common::Common, hittable::Hittable, hittable_list::HittableList, material::{Dielectric, Lambertian, Metal}, sphere::Sphere, vec3::{Point3, Vec3}};


fn main() {
	// World

	let material_ground = Lambertian::new(Color::new(0.5, 0.5, 0.5));

	let sphere_ground = Box::new(Sphere::new(
		Point3::new(0.0,-1000.0,0.0),
		1000.0, Box::new(material_ground)));

	let mut objects: Vec<Box<(dyn Hittable + 'static)>> = vec![sphere_ground];

	for i in -11..10 {
		for j in -11..10 {
			let choose_mat = Common::random_float();
			let center = Point3::new(f64::from(i) + 0.9 * Common::random_float(), 0.2, f64::from(j) + 0.9* Common::random_float());

			if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
				if choose_mat < 0.8 {
					// diffuse
					let albedo = Color::random() * Color::random();
					let sphere_material = Lambertian::new(albedo);
					let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
					objects.push(Box::new(sphere));
				}
				else if choose_mat < 0.95 {
					// metal
					let albedo = Color::random_in_range(0.5, 1.0);
					let fuzz = Common::random_float_in_range(0.0, 0.5);
					let sphere_material = Metal::new(albedo, fuzz);
					let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
					objects.push(Box::new(sphere));
				} else {
					// glass
					let sphere_material = Dielectric::new(1.5);
					let sphere = Sphere::new(center, 0.2, Box::new(sphere_material));
					objects.push(Box::new(sphere));
				}
			}
		}
	}

	let material1 = Dielectric::new(1.5);
	let sphere1 = Sphere::new(Point3::new(0.0, 1.0, 0.0), 1.0, Box::new(material1));
	objects.push(Box::new(sphere1));

	let material2 = Lambertian::new(Point3::new(0.4, 0.2, 0.1));
	let sphere2 = Sphere::new(Point3::new(-4.0, 1.0, 0.0), 1.0, Box::new(material2));
	objects.push(Box::new(sphere2));

	let material3 = Metal::new(Color::new(0.7, 0.6, 0.5), 0.0);
	let sphere3 = Sphere::new(Point3::new(4.0, 1.0, 0.0), 1.0, Box::new(material3));
	objects.push(Box::new(sphere3));

	let world = HittableList {
		objects: objects
	};


	// Camera

	let aspect_ratio = 16.0 / 9.0;
	let image_width: i64 = 1200;
	let samples_per_pixel = 500;
	let max_depth = 50;
	let vfov = 20.0;
	let look_from = Point3::new(13.0, 2.0, 3.0);
	let look_at = Point3::new(0.0, 0.0, 0.0);
	let vup = Vec3::new(0.0, 1.0, 0.0);
	let defocus_angle = 0.6;
	let focus_dist = 10.0;
	let filename = "testimg.ppm";
	
	let camera = Camera::new(aspect_ratio, image_width,
		samples_per_pixel, max_depth, vfov, look_from,
		look_at, vup, defocus_angle, focus_dist, filename);
	camera.render(Box::new(&world));
}