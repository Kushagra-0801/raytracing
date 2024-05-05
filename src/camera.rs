use crate::{color::Color, hittable::Hittable, position::Position, ray::Ray};

use paste::paste;

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    aspect_ratio: f64,
    image_width: i32,
    focal_length: f64,
    viewport_height: f64,
    camera_center: Position,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            focal_length: 1.0,
            viewport_height: 2.0,
            camera_center: Position::new(0.0, 0.0, 0.0),
        }
    }
}

macro_rules! with_field {
    ($($field: ident : $type: ty)+) => {
        paste! {
            $(
                pub fn [<with_ $field>](mut self, v: $type) -> Self {
                    self.$field = v;
                    self
                }
            )+
        }
    }
}

impl CameraOptions {
    with_field! {
        aspect_ratio: f64
        image_width: i32
        focal_length: f64
        viewport_height: f64
        camera_center: Position
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct Camera {
    image_width: i32,
    image_height: i32,
    center: Position,
    pixel00_loc: Position,
    pixel_delta_u: Position,
    pixel_delta_v: Position,
}

impl Camera {
    pub fn new(options: CameraOptions) -> Self {
        let aspect_ratio = options.aspect_ratio;
        let image_width = options.image_width;
        let image_height = (f64::from(image_width) / aspect_ratio).trunc() as i32;

        let focal_length = options.focal_length;
        let viewport_height = options.viewport_height;
        let viewport_width = viewport_height * (f64::from(image_width) / f64::from(image_height));
        let camera_center = options.camera_center;

        let viewport_u = Position::new(viewport_width, 0.0, 0.0);
        let viewport_v = Position::new(0.0, -viewport_height, 0.0);

        let pixel_delta_u = viewport_u / f64::from(image_width);
        let pixel_delta_v = viewport_v / f64::from(image_height);

        let viewport_upper_left = camera_center
            - Position::new(0.0, 0.0, focal_length)
            - viewport_u / 2.0
            - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
        }
    }

    pub fn render(&self, world: impl Hittable) {
        println!("P3");
        println!(
            "{image_width} {image_height}",
            image_width = self.image_width,
            image_height = self.image_height,
        );
        println!("255");
        for j in 0..self.image_height {
            eprintln!(
                "Lines remaining: {rem_lines}",
                rem_lines = self.image_height - j
            );
            for i in 0..self.image_width {
                let pixel_center = self.pixel00_loc
                    + (f64::from(i) * self.pixel_delta_u)
                    + (f64::from(j) * self.pixel_delta_v);
                let ray_direction = pixel_center - self.center;
                let ray = Ray::new(self.center, ray_direction.unit());

                let pixel_color = self.ray_color(ray, &world);

                println!("{pixel_color}");
            }
        }
        eprintln!("Done");
    }

    fn ray_color(&self, r: Ray, world: impl Hittable) -> Color {
        let ray_hit = world.hit(r, 0.0..=f64::INFINITY);
        if let Some(rec) = ray_hit {
            return Color::from(0.5 * (rec.n + Position::new(1.0, 1.0, 1.0)));
        }
        let a = 0.5 * (r.direction().y() + 1.0);
        Color::from((1.0 - a) * Position::new(1.0, 1.0, 1.0) + a * Position::new(0.5, 0.7, 1.0))
    }
}
