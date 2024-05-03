use crate::{color::Color, position::Position};

mod color;
mod position;
mod ray;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{image_width} {image_height}");
    println!("255");

    for j in 0..image_height {
        eprintln!("Lines remaining: {rem_lines}", rem_lines = image_height - j);
        for i in 0..image_width {
            let r = f64::from(i) / f64::from(image_width - 1);
            let g = f64::from(j) / f64::from(image_height - 1);
            let b = 0f64;

            let c = Color::from(Position::new(r, g, b));

            println!("{c}");
        }
    }
    eprintln!("Done");
}
