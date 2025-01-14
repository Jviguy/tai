use image::RgbaImage;
// TODO: make the "scale" dynamic by the user to control how many colors will be inside.
// This algorithm to make a dithered image, it's error diff algorithm check the source below.
// source : https://en.wikipedia.org/wiki/Floyd%E2%80%93Steinberg_dithering

pub fn floyd_dither(img: &mut RgbaImage) {
    // this will control the colors in the image(more value==more colors).
    let scale = 16.0;

    for y in 0..img.height() - 1 {
        for x in 1..img.width() - 1 {
            let old_rgb: [u8; 4] = img.get_pixel(x, y).0;
            let new_rgb: [u8; 4] = find_closest_color(old_rgb, scale);

            img.get_pixel_mut(x, y).0[..3].clone_from_slice(&new_rgb[..3]);

            let mut pixel = img.get_pixel_mut(x, y).0;
            pixel[0] = new_rgb[0];
            pixel[1] = new_rgb[1];
            pixel[2] = new_rgb[2];

            let err_r: f32 = old_rgb[0] as f32 - new_rgb[0] as f32;
            let err_g: f32 = old_rgb[1] as f32 - new_rgb[1] as f32;
            let err_b: f32 = old_rgb[2] as f32 - new_rgb[2] as f32;
            let err_pixel = [err_r, err_g, err_b];

            calculate_and_assign_pixel(img, (x + 1, y), err_pixel, 7.0);
            calculate_and_assign_pixel(img, (x - 1, y + 1), err_pixel, 3.0);
            calculate_and_assign_pixel(img, (x, y + 1), err_pixel, 5.0);
            calculate_and_assign_pixel(img, (x + 1, y + 1), err_pixel, 1.0);
        }
    }
}

// TODO: FIXME, IM UGLY you fucking asshole
// this helper function will calculate the the neighbor pixel and add value from the error pixel as refrenced in wikipedia.
fn calculate_and_assign_pixel(
    img: &mut RgbaImage,     // imagebuffer
    pixel_coord: (u32, u32), // coordinate (x,y)
    err_pixel: [f32; 3],     // error pixel [R, G, B]
    val: f32,                // value will be added to the calculation
) {
    // R
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[0] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[0] as f32 + err_pixel[0] * val / 16.0) as u8;
    // G
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[1] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[1] as f32 + err_pixel[1] * val / 16.0) as u8;
    // B
    img.get_pixel_mut(pixel_coord.0, pixel_coord.1).0[2] =
        (img.get_pixel(pixel_coord.0, pixel_coord.1).0[2] as f32 + err_pixel[2] * val / 16.0) as u8;
}

// this helper function to calculate the rgb values for the floyed dither algorithm.
fn find_closest_color(pixel: [u8; 4], factor: f32) -> [u8; 4] {
    [
        ((factor * pixel[0] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[1] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        ((factor * pixel[2] as f32 / 255.0).ceil() * (255.0 / factor)) as u8,
        pixel[3],
    ]
}
