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
use sphere::Sphere;
use std::{
    io::{self, Write},
    rc::Rc,
};
use vec3::{Color, Point3};

fn main() -> io::Result<()> {
    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio).trunc() as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // Camera
    let camera = Camera::default();

    // rand
    let mut rng = rand::thread_rng();
    let zero_to_one = Uniform::from(0.0..1.0);

    // Render
    write!(stdout, "P3\n{} {}\n255\n", image_width, image_height)?;
    for j in (0..image_height).rev() {
        write!(stderr, "\rScanlines remaining: {} ", j)?;
        stderr.flush()?;
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + zero_to_one.sample(&mut rng))
                    / (image_width - 1) as f64;
                let v = (j as f64 + zero_to_one.sample(&mut rng))
                    / (image_height - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_color += ray_color(&r, &world)
            }
            vec3::write_color(&mut stdout, &pixel_color, samples_per_pixel)?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}

fn ray_color(r: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::default();
    if world.hit(r, 0.0, f64::INFINITY, &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.direction().unit_vector();
    let t = 0.5 * (unit_direction.y() + 1.0);
    // lerp: linear blend of colors
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}
