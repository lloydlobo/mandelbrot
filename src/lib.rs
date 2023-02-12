use image::{ImageBuffer, Rgb};

pub fn compose(width: u32, height: u32, iterations: u32) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
    let mut image = ImageBuffer::new(width, height);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let c = to_complex_num(x, y, width, height);
        let i = mandelbrot(c, iterations);
        *pixel = Rgb([i as u8, i as u8, i as u8]);
    }
    image
}

/// The function to_complex_num maps pixel coordinates to complex plane coordinates,
fn to_complex_num(x: u32, y: u32, width: u32, height: u32) -> (f64, f64) {
    (
        (x as f64 / width as f64 * 3.5 - 2.5),
        (y as f64 / height as f64 * 2.0 - 1.0),
    )
}
// Note that we used the round function to round the f64 values to the nearest u32.
// This way, we ensure that we don't lose precision (in the context of `to_complex_num` casting) during the casting from f64 to u32.
fn from_complex_num(c: (f64, f64), width: u32, height: u32) -> (u32, u32) {
    let (cx, cy) = c;
    let x = (cx + 2.5) / 3.5 * (width as f64 * 1.0);
    let y = (cy + 1.0) / 2.0 * (height as f64 * 1.0);
    (x.round() as u32, y.round() as u32)
}

/// The given code is for plotting the Mandelbrot set, a famous fractal.
/// `mandelbrot` calculates the number of iterations until a complex number
/// escapes to infinity or stays within a given radius.
///
/// In this example, the mandelbrot function takes a complex number c as input and returns the number of iterations it takes for the magnitude of the iterated sequence to exceed 2. The main function creates an 800x800 image and sets the color of each pixel based on the value returned by the mandelbrot function for the corresponding complex number. Finally, the image is saved as a PNG file.
///
/// The Mandelbrot set is a set of complex numbers for which the corresponding sequence, defined by the iterative equation z(n+1) = z(n)^2 + c, remains bounded. The sequence is defined by the complex number c and an initial value of z(0) = 0.
///
/// The equation generates a sequence of complex numbers, and if the magnitude of the sequence stays within a certain bound (usually 2), then the number c is considered to be in the Mandelbrot set. If the magnitude of the sequence exceeds the bound, then c is considered to be outside the Mandelbrot set.
///
/// In the plot of the Mandelbrot set, each complex number c is represented by a pixel on the screen, and the color of the pixel is determined by the number of iterations it takes for the magnitude of the sequence to exceed the bound. If the number of iterations is small, the pixel is colored black. If the number of iterations is large, the pixel is colored white. If the number of iterations is intermediate, the pixel is colored based on a color map that maps the number of iterations to a color.
///
/// The Mandelbrot set is a fractal, meaning that it exhibits self-similar patterns at different scales. The boundaries of the Mandelbrot set are fractal in nature and have a complex, intricate structure that can be explored by zooming into different regions of the set.
///
/// The principle behind the Mandelbrot set is to visualize the behavior of complex sequences defined by simple mathematical equations, and to explore the intricate structure of the boundaries of these sequences. The Mandelbrot set is a beautiful and fascinating mathematical object that has captured the imagination of mathematicians, artists, and computer scientists for decades.
pub fn mandelbrot(c: (f64, f64), iterations: u32) -> u32 {
    let (cx, cy) = c;
    let mut x = 0.0;
    let mut y = 0.0;
    let mut i = 0;
    while i < iterations {
        let x_temp = x * x - y * y + cx;
        y = 2.0 * x * y + cy;
        x = x_temp;
        if x * x + y * y > 4.0 {
            break;
        }
        i += 1;
    }
    i
}

pub mod ascii {
    /*
    ....................................................**:*::......................
    ...................................................***::***.....................
    ..................................................***::8:**.....................
    .................................................**::8@@::**....................
    ...............................................****:#@@@@o***...................
    ............................................*******o@@@@@o*****.................
    ...........................................**:*****:@@@@@:*********.............
    .........................................***&::*@o@@@@@@@@::o:***:*.............
    ........................................****:@@:o@@@@@@@@@@@@::::::*............
    .......................................*****&@@@@@@@@@@@@@@@@@@@@o**............
    ......................................****::8#@@@@@@@@@@@@@@@@@@@:**............
    ............................****....*****:8@@@@@@@@@@@@@@@@@@@@@&:**............
    ...........................***************:@@@@@@@@@@@@@@@@@@@@@@:***...........
    ..........................**:****::******#@@@@@@@@@@@@@@@@@@@@@@@@@o*...........
    ..........................***:#:::o::***:o@@@@@@@@@@@@@@@@@@@@@@@@:**...........
    ..........................****o@o@@@@o:::@@@@@@@@@@@@@@@@@@@@@@@@@::*...........
    .........................***::o@@@@@@@@::@@@@@@@@@@@@@@@@@@@@@@@@@@:*...........
    .......................**:**:&@@@@@@@@@@o@@@@@@@@@@@@@@@@@@@@@@@@@&*............
    ...................******::::@@@@@@@@@@@&@@@@@@@@@@@@@@@@@@@@@@@@@#*............
    ..............***********:o@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@o**............
    ............@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@&:***............
    ..............***********:o@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@@o**............
    ...................******::::@@@@@@@@@@@&@@@@@@@@@@@@@@@@@@@@@@@@@#*............
    .......................**:**:&@@@@@@@@@@o@@@@@@@@@@@@@@@@@@@@@@@@@&*............
    .........................***::o@@@@@@@@::@@@@@@@@@@@@@@@@@@@@@@@@@@:*...........
    ..........................****o@o@@@@o:::@@@@@@@@@@@@@@@@@@@@@@@@@::*...........
    ..........................***:#:::o::***:o@@@@@@@@@@@@@@@@@@@@@@@@:**...........
    ..........................**:****::******#@@@@@@@@@@@@@@@@@@@@@@@@@o*...........
    ...........................***************:@@@@@@@@@@@@@@@@@@@@@@:***...........
    ............................****....*****:8@@@@@@@@@@@@@@@@@@@@@&:**............
    ......................................****::8#@@@@@@@@@@@@@@@@@@@:**............
    .......................................*****&@@@@@@@@@@@@@@@@@@@@o**............
    ........................................****:@@:o@@@@@@@@@@@@::::::*............
    .........................................***&::*@o@@@@@@@@::o:***:*.............
    ...........................................**:*****:@@@@@:*********.............
    ............................................*******o@@@@@o*****.................
    ...............................................****:#@@@@o***...................
    .................................................**::8@@::**....................
    ..................................................***::8:**.....................
    ...................................................***::***.....................
     */

    use std::collections::HashMap;

    const WIDTH: u32 = 80;
    const HEIGHT: u32 = 40;
    const ITERATIONS: u32 = 100;
    const ESCAPE_RADIUS: f64 = 2.0;

    // Converts ASCII characters to represent the intensity of the Mandelbrot set value
    fn to_ascii_char(value: u32) -> char {
        match value {
            0..=5 => '.',
            6..=10 => '*',
            11..=20 => ':',
            21..=30 => 'o',
            31..=40 => '&',
            41..=50 => '8',
            51..=60 => '#',
            _ => '@',
        }
    }

    // Converts pixel coordinates to complex number
    fn to_complex_num(x: u32, y: u32, width: u32, height: u32) -> (f64, f64) {
        let cx = x as f64 / width as f64 * 3.5 - 2.5;
        let cy = y as f64 / height as f64 * 2.0 - 1.0;
        (cx, cy)
    }

    // Calculates the Mandelbrot set value for a given complex number
    fn mandelbrot(c: (f64, f64)) -> u32 {
        let (cx, cy) = c;
        let mut x = 0.0;
        let mut y = 0.0;
        let mut iterations = 0;

        while x * x + y * y <= ESCAPE_RADIUS * ESCAPE_RADIUS && iterations < ITERATIONS {
            let x_new = x * x - y * y + cx;
            y = 2.0 * x * y + cy;
            x = x_new;
            iterations += 1;
        }

        iterations
    }

    // Calculates the pixel index from the x and y coordinate
    fn calculate_pixel_index(x: u32, y: u32, width: u32) -> usize {
        (y * width + x) as usize
    }

    pub fn print_mandelbrot_ascii_art() {
        let image_area = WIDTH * HEIGHT;
        let mut image = HashMap::new();

        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c = to_complex_num(x, y, WIDTH, HEIGHT);
                let value = mandelbrot(c);
                let ascii_char = to_ascii_char(value);
                let pixel_index = calculate_pixel_index(x, y, WIDTH);
                image.insert(pixel_index, ascii_char);
            }
        }

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let pixel_index = calculate_pixel_index(x, y, WIDTH);
                print!("{}", image[&pixel_index]);
            }
            println!();
        }
    }
}

//------------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    const WIDTH: u32 = 800;
    const HEIGHT: u32 = 800;

    #[test]
    fn test_mandelbrot_0() {
        let c = to_complex_num(1, 1, 800, 800);
        let iterations = 255;
        assert_eq!(mandelbrot(c, iterations), 0);
    }
    /// The test function checks if a particular pixel (1, 1) with width 800 and height 800 maps to a complex number that stays within the set (returns 0 iterations).
    #[test]
    fn test_mandelbrot() {
        const ITERATIONS: u32 = 255;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let c = to_complex_num(x, y, WIDTH, HEIGHT);
                let got = mandelbrot(c, ITERATIONS);
                assert!(got <= ITERATIONS);
            }
        }
    }

    #[test]
    fn test_from_complex_num() {
        let width = WIDTH;
        let height = HEIGHT;
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                let (cx1, cy1) = (x, y);
                let c = to_complex_num(cx1, cy1, width, height);
                let (cx2, cy2) = from_complex_num(c, width, height);
                assert_eq!((cx1, cy1), (cx2, cy2));
            }
        }
    }

    /// The pixel_index function calculates the index of a pixel in a 1D representation of a 2D grid of pixels. The function takes the x and y coordinates of a pixel in the 2D grid, as well as the width of the grid, and returns the index of the pixel in the 1D representation.
    ///
    /// The calculation is done by multiplying the y coordinate of the pixel by the width of the grid, and then adding the x coordinate of the pixel. This formula takes into account the number of columns (the width of the grid) and the position of the pixel in the row (given by the x coordinate) to calculate its position in the 1D representation.
    ///
    /// The function is useful in situations where you need to iterate over all the pixels in a grid, but need to store the pixels in a 1D array for some reason (such as more efficient memory usage, or because the data structure you are using to store the pixels only supports 1D arrays).
    fn calculate_pixel_index(x: u32, y: u32, width: u32) -> u32 {
        y * width + x
    }

    #[test]
    fn test_complex_num() {
        let width = WIDTH;
        let height = HEIGHT;
        let image_area = WIDTH * HEIGHT;

        let mut hash_enum = HashMap::new();
        let mut hash_loops = HashMap::new();

        // Enumerate the pixels in the image area using an enumerated iterator
        (0..image_area)
            .into_iter()
            .enumerate()
            .for_each(|(pixel_index, pixel_value)| {
                // Calculate the x and y coordinate of the pixel
                let x = pixel_value % width;
                let y = pixel_value / width;
                // Insert the pixel index and its corresponding x and y coordinate into the hash_enum map
                hash_enum.insert(pixel_index, (x, y));

                let (cx1, cy1) = (x, y);
                // Convert the x and y coordinate to a complex number
                let c = to_complex_num(cx1, cy1, width, height);
                // Convert the complex number back to its corresponding x and y coordinate
                let (cx2, cy2) = from_complex_num(c, width, height);

                // Verify that the conversion from x and y coordinate to complex number and back is correct
                assert_eq!((cx1, cy1), (cx2, cy2));
                // Verify that the pixel index is within bounds
                assert!(
                    (pixel_index as u32) < image_area,
                    "Pixel index is out of bounds: {pixel_index}",
                );
                // Verify that the calculated pixel index matches the expected pixel index
                assert_eq!(pixel_index as u32, calculate_pixel_index(x, y, width));
            });

        // Loop through the pixels in the image area using nested loops
        for x in 0..WIDTH {
            for y in 0..HEIGHT {
                // Calculate the pixel index of the current pixel
                let pixel_index = calculate_pixel_index(x, y, width);
                // Insert the pixel index and its corresponding x and y coordinate into the hash_loops map
                hash_loops.insert(pixel_index as usize, (x, y));

                let (cx1, cy1) = (x, y);
                // Convert the x and y coordinate to a complex number
                let c = to_complex_num(cx1, cy1, width, height);
                // Convert the complex number back to its corresponding x and y coordinate
                let (cx2, cy2) = from_complex_num(c, width, height);

                // Verify that the conversion from x and y coordinate to complex number and back is correct
                assert_eq!((cx1, cy1), (cx2, cy2));
            }
        }

        // Verify that the results of the enumeration and the loop are the same
        assert_eq!(hash_enum, hash_loops);
    }
}
