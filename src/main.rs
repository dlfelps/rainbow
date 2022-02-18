use image::ImageError;
use image::io::Reader;
use image::ImageFormat;

use palette::Srgb;
use palette::Pixel;
use palette::Lch;
use palette::FromColor;
use palette::Hue;

fn distance(p1: [u32;2], p2: [u32;2]) -> f64 {
    let x_diff = (p1[0] as f64) - (p2[0] as f64);
    let x_2 = x_diff.powi(2);
    let y_diff = (p1[1] as f64) - (p2[1] as f64);
    let y_2 = y_diff.powi(2);
    (x_2 + y_2).sqrt()
    
}

fn main() -> Result<(), ImageError> {

    let mut img = Reader::open("test.jpg")?.decode()?.into_rgb8(); 
    let origin:[u32; 2] = [1260, 2090];
    let max_shift = -90.0;
    let ext = "bmp";
    let center = [img.width() as f64 / 2.0, img.height() as f64 / 2.0];

    let pole = match ((origin[0] as f64) < center[0], (origin[1] as f64) < center[1] ){
        (false, false) => [0,0],
        (false, true) => [0, img.height()],
        (true, false) => [img.width(), 0],
        (true, true) => [img.width(), img.height()],
    };

    let max_distance = distance(origin, pole);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let color = Srgb::from_raw(&pixel.0).into_format();

        let current_distance = distance(origin, [x,y]);

        let hue_shifted = Lch::from_color(color).shift_hue(max_shift*current_distance/max_distance);
        pixel.0 = Srgb::from_color(hue_shifted).into_format().into_raw()
    }

    let output_format: ImageFormat = match ext {
        "bmp" => ImageFormat::Bmp,
        "jpg" | "jpeg" => ImageFormat::Jpeg,
        "png" => ImageFormat::Png,
        "tiff" => ImageFormat::Tiff,
        _ => ImageFormat::Bmp,
    };

    img.save_with_format(format!("{}{}","output.", ext), output_format)?;

    Ok(())
    
}