use std::{collections::HashMap, env, sync::RwLock};

use clap::{command, Arg, ArgMatches};
use config::{builder::DefaultState, Config, ConfigBuilder};
use console::Style;
use indicatif::{ProgressBar, ProgressStyle};
use log::{info, LevelFilter::Info};
use once_cell::sync::Lazy;
use pretty_env_logger::env_logger::Builder;

const ITERATIONS: u32 = 255;
const WIDTH: u32 = 800;
const HEIGHT: u32 = 800;
const DEFAULT_SETTINGS_FILE: &str = "settings.toml";
const DEFAULT_IMAGE_PATH: &str = "mandelbrot.png";

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
    // TODO: Directly mutate `CONFIG_BUILDER` without assigning it.
    let settings_builder: ConfigBuilder<DefaultState> = CONFIG_BUILDER
        .write()
        .unwrap()
        .clone()
        .set_default("verbose", "1")? // This is not in the settings file.
        .add_source(config::File::with_name(&curr_path.to_string_lossy()))
        .add_source(config::Environment::with_prefix("APP"));
    // Does not take ownership of `ConfigBuilder` to allow later reuse.
    let settings_new: Config = settings_builder.build_cloned()?;
    // {"key": "189rjfadoisfj8923fjio", "verbose": "1", "priority": "32", "debug": "false"}
    let _map_new = settings_new.try_deserialize::<HashMap<String, String>>()?;

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
        mandelbrot::mandelbrot_img::compose(WIDTH, HEIGHT, ITERATIONS).save(DEFAULT_IMAGE_PATH)?;
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
/// # Examples
///
/// ```rust,ignore
/// use config::Config;
/// use mandelbrot::build_config_settings;
///
/// fn main() {
///     // TODO: Add `tempfile` crate or `include_str!()`.
///     let settings = build_config_settings("settings.toml")?;
///     let map = settings.try_deserialize::<HashMap<String, String>>().unwrap();
///     let expect = r#"{"key": "189rjfadoisfj8923fjio", "debug": "false", "priority": "32"}"#;
///     assert_eq!(map, expect);
/// }
/// ```
/// # Panics
///
/// Panics if .
///
/// # Errors
///
/// This function will return an error if .
pub fn build_config_settings(path: &str) -> Result<Config, config::ConfigError> {
    let mut curr_path = env::current_dir().unwrap();
    curr_path.push(path);

    Config::builder()
        // Add in `./settings.toml`
        .add_source(config::File::with_name(&curr_path.to_string_lossy()))
        // Add in settings from the environment (with a prefix of APP)
        // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
        .add_source(config::Environment::with_prefix("APP"))
        .build()
}
