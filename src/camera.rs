use crate::{color::Color, hittable::Hittable, interval::Interval, position::Position, ray::Ray};

use paste::paste;
use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct CameraOptions {
    aspect_ratio: f64,
    image_width: i32,
    focal_length: f64,
    viewport_height: f64,
    camera_center: Position,
    samples_per_pixel: i32,
    max_bounces: i32,
}

impl Default for CameraOptions {
    fn default() -> Self {
        Self {
            aspect_ratio: 16.0 / 9.0,
            image_width: 400,
            focal_length: 1.0,
            viewport_height: 2.0,
            camera_center: Position::new(0.0, 0.0, 0.0),
            samples_per_pixel: 10,
            max_bounces: 10,
        }
    }
}

macro_rules! with_field {
    ($($field: ident : $type: ty)+) => {
        paste! {
            $(
                #[allow(dead_code)]
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
        samples_per_pixel: i32
        max_bounces: i32
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
    samples_per_pixel: i32,
    max_bounces: i32,
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

        let samples_per_pixel = options.samples_per_pixel;
        let max_bounces = options.max_bounces;

        Self {
            image_width,
            image_height,
            center: camera_center,
            pixel00_loc,
            pixel_delta_u,
            pixel_delta_v,
            samples_per_pixel,
            max_bounces,
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
                let rays = (0..self.samples_per_pixel).map(|_| {
                    let offset_x = rand::thread_rng().gen_range(-0.5..=0.5);
                    let offset_y = rand::thread_rng().gen_range(-0.5..=0.5);
                    let pixel_center = self.pixel00_loc
                        + ((f64::from(i) + offset_x) * self.pixel_delta_u)
                        + ((f64::from(j) + offset_y) * self.pixel_delta_v);
                    let ray_direction = pixel_center - self.center;
                    Ray::new(self.center, ray_direction.unit())
                });

                let pixel_color = self.avg_ray_color(rays, &world);

                println!("{pixel_color}");
            }
        }
        eprintln!("Done");
    }

    fn ray_color_intensity(r: Ray, bounces_left: i32, world: impl Hittable) -> Position {
        if bounces_left <= 0 {
            return Position::default();
        }
        let ray_hit = world.hit(
            r,
            Interval {
                start: 0.001,
                end: f64::INFINITY,
            },
        );
        if let Some(rec) = ray_hit {
            let reflection_direction = Self::reflect_ray_diffuse(r, rec.normal_vector);
            let reflected_ray = Ray::new(rec.incidence_point, reflection_direction);
            0.2 * Self::ray_color_intensity(reflected_ray, bounces_left - 1, world)
        } else {
            let a = 0.5 * (r.direction().y() + 1.0);
            (1.0 - a) * Position::new(1.0, 1.0, 1.0) + a * Position::new(0.5, 0.7, 1.0)
        }
    }

    #[allow(dead_code)]
    fn ray_color(&self, r: Ray, world: impl Hittable) -> Color {
        Color::with_gamma_correction(Self::ray_color_intensity(r, self.max_bounces, world))
    }

    fn avg_ray_color(&self, rs: impl Iterator<Item = Ray>, world: impl Hittable + Copy) -> Color {
        let mut rays = 0;
        let total_intensity: Position = rs
            .inspect(|_| rays += 1)
            .map(|r| Self::ray_color_intensity(r, self.max_bounces, world))
            .sum();
        let avg_intensity = total_intensity / f64::from(rays.max(1));
        Color::with_gamma_correction(avg_intensity)
    }

    fn reflect_ray_diffuse(incident_ray: Ray, surface_normal: Position) -> Position {
        let _ = incident_ray;
        let rand_vec = Position::random_on_unit_sphere();
        let rand_vec = if surface_normal.dot(rand_vec) > 0.0 {
            rand_vec
        } else {
            -rand_vec
        };
        (surface_normal + rand_vec).unit()
    }
}
