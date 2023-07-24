// Título (série) : “Ray Tracing em uma série de fim de semana”
// Título (livro) : “Ray Tracing em um fim de semana”
// Autor : Peter Shirley
// Editores : Steve Hollasch, Trevor David Black
// Versão/Edição : v3.2.3
// Data : 2020-12-07
// URL (série) : https://raytracing.github.io/
// URL (livro) : https://raytracing.github.io/books/RayTracingInOneWeekend.html

// Transposing challenge code from c++ to Rust

// Indicate modules
mod vec3;
mod color;
mod ray;

//use std::io::{self, Write};
//use color::write_color;

use crate::vec3::{Point3, Vec3};
//use vec3::ColorVec;
use crate::ray::Ray;

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - *center;
    let a = Vec3::dot(r.direction(), r.direction());
    let b = 2.0 * Vec3::dot(oc, r.direction());
    let c = Vec3::dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

fn ray_color(r: &Ray) -> vec3::ColorVec {
    let t: f64 = hit_sphere(&Point3::new_with_values(0.0, 0.0, -1.0), 0.5, r);
    if (t > 0.0) {
        let N = Vec3::unit_vector(r.at(t) - Vec3::new_with_values(0.0, 0.0, -1.0));
        return 0.5 * vec3::ColorVec::new_with_values(N.x() + 1.0, N.y() + 1.0, N.z() + 1.0);
    }
    let unit_direction = Vec3::unit_vector(r.direction());
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * vec3::ColorVec::new_with_values(1.0, 1.0, 1.0) + t * vec3::ColorVec::new_with_values(0.5, 0.7, 1.0)
}

fn main() -> std::io::Result<()> {
    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new_with_values(0.0, 0.0, 0.0);
    let horizontal = Vec3::new_with_values(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new_with_values(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new_with_values(0.0, 0.0, focal_length);
    
    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for j in (0..image_height).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..image_width {
            let u = i as f64 / (image_width - 1) as f64;
            let v = j as f64 / (image_height - 1) as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);
            color::write_color(&mut std::io::stdout(), pixel_color)?;
        }
    }

    eprint!("\nDone!\n");
    Ok(())
}
