// use console::style;
use indicatif::{ProgressBar, ProgressStyle};

const ITERATIONS: u32 = 255;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PATH: &str = "mandelbrot.png";

fn main() {
    {
        let progress_bar = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%")
                .unwrap()
                .progress_chars("##-"),
        );
        let image = mandelbrot::mandelbrot_ascii::collect_ascii();
        progress_bar.finish();
        mandelbrot::mandelbrot_ascii::print_ascii(image);
    };

    let progress_bar = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
    progress_bar.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%")
            .unwrap()
            .progress_chars("##-"),
    );
    if let Err(e) = mandelbrot::mandelbrot_img::compose(WIDTH, HEIGHT, ITERATIONS).save(PATH) {
        eprintln!("{}", e);
        std::process::exit(1);
    }
    progress_bar.finish();
}
