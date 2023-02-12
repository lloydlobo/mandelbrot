use mandelbrot::{ascii::print_mandelbrot_ascii_art, compose};

const ITERATIONS: u32 = 255;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PATH: &str = "mandelbrot.png";

fn main() {
    print_mandelbrot_ascii_art();
    if let Err(e) = compose(WIDTH, HEIGHT, ITERATIONS).save(PATH) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
