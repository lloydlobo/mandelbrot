use clap::{command, Arg, ArgMatches};
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use log::info;
use pretty_env_logger::env_logger::Builder;

const ITERATIONS: u32 = 255;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const PATH: &str = "mandelbrot.png";

fn main() {
    Builder::from_default_env().format_timestamp(None).filter_level(log::LevelFilter::Info).init();

    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let matches: ArgMatches = command!()
        .arg(
            Arg::new("ascii")
                .long("ascii")
                .help("Generates the Mandelbrot set as ASCII art and print to terminal"),
        )
        .arg(
            Arg::new("text")
                .long("text")
                .help("Generates the Mandelbrot set as ASCII art and saves to text file"),
        )
        .arg(
            Arg::new("image")
                .long("image")
                .help("Generates the Mandelbrot set as an image and saves to file"),
        )
        .after_help(
            "Longer explanation to appear after the options when displaying the help information \
             from --help or -h",
        )
        .get_matches();

    if let Some(_ascii) = matches.get_one::<String>("ascii") {
        info!("Rendering image Mandelbrot set as {}", Style::new().bold().apply_to("ASCII"));
        let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        style_progress_bar(&pb);
        let image = mandelbrot::mandelbrot_ascii::collect_ascii();
        pb.finish();
        mandelbrot::mandelbrot_ascii::print_ascii(image);
    }

    if let Some(_text) = matches.get_one::<String>("text") {
        info!(
            "Rendering image Mandelbrot set as {} and saving to file",
            Style::new().bold().apply_to("ASCII")
        );
        let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        style_progress_bar(&pb);
        let image = mandelbrot::mandelbrot_ascii::collect_ascii();
        mandelbrot::mandelbrot_ascii::write_ascii_to_file(image);
        pb.finish_with_message("Wrote ascii to file");
    }

    if let Some(_image) = matches.get_one::<String>("image") {
        info!(
            "Rendering image Mandelbrot set as {} and saving to file",
            Style::new().bold().apply_to("image")
        );
        let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        style_progress_bar(&pb);
        mandelbrot::mandelbrot_img::compose(WIDTH, HEIGHT, ITERATIONS).save(PATH)?;
        pb.finish_with_message("Saved image to file");
    }

    Ok(())
}

fn style_progress_bar(pb: &ProgressBar) {
    pb.set_style(
        ProgressStyle::default_bar()
            .template("[{elapsed_precise}] [{bar:40.cyan/blue}] {percent}%")
            .unwrap()
            .progress_chars("##-"),
    );
}
