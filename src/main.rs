use crate::{color::Color, position::Position, ray::Ray};

mod color;
mod position;
mod ray;

fn main() {
    let aspect_ratio = 16.0 / 9.0;

    let image_width = 400;
    let image_height = (f64::from(image_width) / aspect_ratio).trunc() as i32;
    let image_height = image_height.max(1);

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
    let camera_center = Position::new(0.0, 0.0, 0.0);

    let viewport_u = Position::new(viewport_width, 0.0, 0.0);
    let viewport_v = Position::new(0.0, -viewport_height, 0.0);

    let pixel_delta_u = viewport_u / f64::from(image_width);
    let pixel_delta_v = viewport_v / f64::from(image_height);

    let viewport_upper_left =
        camera_center - Position::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Lines remaining: {rem_lines}", rem_lines = image_height - j);
        for i in 0..image_width {
            let pixel_center =
                pixel00_loc + (f64::from(i) * pixel_delta_u) + (f64::from(j) * pixel_delta_v);
            let ray_direction = pixel_center - camera_center;
            let ray = Ray::new(camera_center, ray_direction.unit());

            let pixel_color = ray_color(ray);

            println!("{pixel_color}");
        }
    }
    eprintln!("Done");
}

fn ray_color(r: Ray) -> Color {
    let a = 0.5 * (r.direction().y() + 1.0);
    Color::from((1.0 - a) * Position::new(1.0, 1.0, 1.0) + a * Position::new(0.5, 0.7, 1.0))
}
