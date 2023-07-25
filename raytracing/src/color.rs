use crate::vec3::ColorVec;
use std::io::{self, Write};

pub fn write_color(out: &mut dyn Write, pixel_color: ColorVec) -> io::Result<()> {
    // Write the translated [0,255] value of each color component
    writeln!(
        out,
        "{} {} {}",
        (255.999 * pixel_color.x()) as i32,
        (255.999 * pixel_color.y()) as i32,
        (255.999 * pixel_color.z()) as i32
    )
}
