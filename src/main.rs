mod vec3;

use std::io::{self, Write};
use vec3::{write_color, Color};
fn main() -> io::Result<()> {
    let image_width = 256;
    let image_height = 256;

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    write!(stdout, "P3\n{} {}\n255\n", image_width, image_height)?;
    for j in (0..image_height).rev() {
        write!(stderr, "\rScanlines remaining: {} ", j)?;
        stderr.flush()?;
        for i in 0..image_width {
            let pixel_color = Color::new(
                i as f64 / (image_width - 1) as f64,
                j as f64 / (image_height - 1) as f64,
                0.25f64,
            );
            write_color(&mut stdout, &pixel_color)?;
        }
    }
    write!(stderr, "\nDone.\n")?;
    Ok(())
}
