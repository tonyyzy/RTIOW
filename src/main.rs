use std::io::{self, Write};
fn main() -> io::Result<()>{
    let image_width = 256;
    let image_height = 256;

    let mut stdout = io::stdout();
    write!(stdout, "P3\n{} {}\n255\n", image_width, image_height)?;
    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let r = i as f64 / (image_width - 1) as f64;
            let g = j as f64 / (image_height - 1) as f64;
            let b = 0.25f64;

            let ir = (255.999 * r).floor() as u8;
            let ig = (255.999 * g).floor() as u8;
            let ib = (255.999 * b).floor() as u8;
            write!(stdout, "{} {} {}\n", ir, ig, ib)?;
        }
    }
    Ok(())
}
