mod camera;
mod hittable;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use camera::Camera;
use hittable::{HitRecord, Hittable, HittableList};
use rand::{distributions::Uniform, prelude::Distribution};
use ray::Ray;
use rtweekend::random;
use sphere::Sphere;
use std::{
    io::{self, Write},
    rc::Rc,
};
use vec3::{
    random_in_hemisphere, random_in_unit_sphere, random_unit_vector, Color,
    Point3,
};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).trunc() as i32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    // Render
    write!(stdout, "P3\n{} {}\n255\n", image_width, image_height)?;
    for j in (0..image_height).rev() {
        write!(stderr, "\rScanlines remaining: {} ", j)?;
        stderr.flush()?;
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + random()) / (image_width - 1) as f64;
                let v = (j as f64 + random()) / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world, max_depth)
            }
            vec3::write_color(&mut stdout, &pixel_color, samples_per_pixel)?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec = HitRecord::default();
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    if world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let target = rec.p + random_in_hemisphere(&rec.normal);
        return 0.5
            * ray_color(&Ray::new(rec.p, target - rec.p), world, depth - 1);
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // lerp: linear blend of colors
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
