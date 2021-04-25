use std::io::stderr;
use std::io::Write;

mod vector;
pub use vector::Vec3;

mod color;
use color::Color;

mod ray;
use ray::{Point3, Ray};

fn main() {
    // Image
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: i32 = 400;
    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = ASPECT_RATIO * VIEWPORT_HEIGHT;
    const FOCAL_LENGTH: f64 = 1.0;

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal: Vec3 = Vec3::new(VIEWPORT_WIDTH, 0.0, 0.0);
    let vertical: Vec3 = Vec3::new(0.0, VIEWPORT_HEIGHT, 0.0);
    let lower_left_corner: Vec3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, FOCAL_LENGTH);

    // Render
    println!("P3\n{} {}\n255", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        stderr().flush().unwrap();
        for i in 0..IMAGE_WIDTH {
            let (u, v) = (
                i as f64 / (IMAGE_WIDTH - 1) as f64,
                j as f64 / (IMAGE_HEIGHT - 1) as f64,
            );
            let pixel_color = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            )
            .color();
            println!("{}", pixel_color);
        }
    }

    eprintln!("\rImage Generated.");
    stderr().flush().unwrap();
}
