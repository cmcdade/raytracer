use crate::vec3::Color;

pub fn write_color(pixel: Color) {
    println!("{}", pixel * 255.999);
}
