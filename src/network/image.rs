//!
//! Image fetching and parsing module to properly retrieve data
//! 

// Importing image
use image::{
    ImageBuffer, 
    Rgba,
    open,
};

/* Helper functions */

/// Returns an image buff given a file path
pub fn get_image(path: &str) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Box<dyn std::error::Error>> {
    // Open the image file
    let img = open(path)?;

    // Convert the image to RGBA format
    let img = img.to_rgba8();

    // Returning image buffer
    Ok(img)
}

/// Given a path it will return a vector of the lightness values from image
pub fn get_image_of_cells(path: &str) -> Result<(Vec<f32>, (usize, usize)), Box<dyn std::error::Error>> {
    let image = get_image(path)?;

    let (width, height) = image.dimensions();

    let mut cells = Vec::with_capacity((width * height) as usize);
    for y in 0..height {
        for x in 0..width {
            // Appending pixel lightness value to array
            let pixel = image.get_pixel(x, y);
            cells.push(pixel[0] as f32 / 255.) // Normalizing color
        }
    }

    Ok((cells, (width as usize, height as usize)))
}
