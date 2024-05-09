use camera::{Camera, CameraOptions};
use hittable::Sphere;
use position::Position;

mod camera;
mod color;
mod hittable;
mod interval;
mod position;
mod ray;

fn main() {
    let ball = Sphere::new(Position::new(0.0, 0.0, -1.0), 0.5);
    let ground = Sphere::new(Position::new(0.0, -50.5, -1.0), 50.0);
    let world = vec![ball, ground];

    let camera_options = CameraOptions::default()
        .with_aspect_ratio(16.0 / 9.0)
        .with_camera_center(Position::new(0.0, 0.0, 0.0))
        .with_focal_length(1.0)
        .with_image_width(400)
        .with_viewport_height(2.0)
        .with_max_bounces(50);
    let camera = Camera::new(camera_options);

    camera.render(world);
}
