use std::{env, path::Path, sync::RwLock};

use clap::{command, Arg, ArgMatches};
use config::{builder::DefaultState, Config, ConfigBuilder};
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, LevelFilter::Info};
use mandelbrot::mandelbrot_ascii;
use once_cell::sync::Lazy;
use pretty_env_logger::env_logger::Builder;
use serde::{Deserialize, Serialize};

const ITERATIONS: u32 = 255;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const DEFAULT_SETTINGS_FILE: &str = "settings.toml";
const DEFAULT_IMAGE_PATH: &str = "mandelbrot.png";
const DEFAULT_TEXT_PATH: &str = "mandelbrot.txt";

static CONFIG_BUILDER: Lazy<RwLock<ConfigBuilder<DefaultState>>> =
    Lazy::new(|| RwLock::new(Config::builder()));

fn main() {
    Builder::from_default_env().format_timestamp(None).filter_level(Info).init();

    if let Err(e) = try_main() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn try_main() -> anyhow::Result<()> {
    let mut curr_path = env::current_dir().unwrap();
    curr_path.push(DEFAULT_SETTINGS_FILE);
    let settings_new: Config = CONFIG_BUILDER
        .write()
        .unwrap()
        .clone()
        .set_default("verbose", "1")? // This is not in the settings file.
        .add_source(config::File::with_name(&curr_path.to_string_lossy()))
        .add_source(config::Environment::with_prefix("APP"))
        .build_cloned()?;
    let config_manager: ConfigManager = settings_new.try_deserialize()?;

    // Parse clap args.
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
        let image = mandelbrot_ascii::collect_ascii();
        pb.finish();
        mandelbrot_ascii::print_ascii(image);
    }

    if let Some(_text) = matches.get_one::<String>("text") {
        info!(
            "Rendering image Mandelbrot set as {} and saving to file",
            Style::new().bold().apply_to("ASCII")
        );
        let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        style_progress_bar(&pb);
        let image = mandelbrot_ascii::collect_ascii();
        mandelbrot_ascii::write_ascii_to_file(image, Path::new(&config_manager.text));
        pb.finish_with_message("Wrote ascii to file");
    }

    if let Some(_image) = matches.get_one::<String>("image") {
        info!(
            "Rendering image Mandelbrot set as {} and saving to file",
            Style::new().bold().apply_to("image")
        );
        let pb = ProgressBar::new(WIDTH as u64 * HEIGHT as u64);
        style_progress_bar(&pb);
        if config_manager.image.is_empty() {
            mandelbrot::mandelbrot_img::compose(WIDTH, HEIGHT, ITERATIONS)
                .save(DEFAULT_IMAGE_PATH)?;
        } else {
            mandelbrot::mandelbrot_img::compose(WIDTH, HEIGHT, ITERATIONS)
                .save(config_manager.image)?;
        }
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

/// `build_config_settings` is a wrapper around `config` crate.
///
/// * `build_cloned` method - Does not take ownership of `ConfigBuilder` to allow later reuse.
/// let map_new = settings_new.try_deserialize::<HashMap<String, String>>()?;
// NOTE: Directly mutate `CONFIG_BUILDER` without assigning it.
pub fn build_config_settings(path: &str) -> Result<Config, config::ConfigError> {
    let mut curr_path = env::current_dir().unwrap();
    curr_path.push(path);

    // Add in `./settings.toml`
    // Add in settings from the environment (with a prefix of APP)
    // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    Config::builder()
        .add_source(config::File::with_name(&curr_path.to_string_lossy()))
        .add_source(config::Environment::with_prefix("APP"))
        .build()
}

#[derive(Debug, Deserialize, Serialize)]
struct ConfigManager {
    text: String,
    image: String,
}

impl Default for ConfigManager {
    fn default() -> Self {
        Self::new()
    }
}

impl ConfigManager {
    fn new() -> Self {
        Self { text: DEFAULT_TEXT_PATH.to_string(), image: DEFAULT_IMAGE_PATH.to_string() }
    }
}
