use editor::update_state;
use std::path::PathBuf;
mod buffer;
mod config;
mod core;
mod editor;
mod render;
use crate::core::*;
use crate::render::render;
use clap::Parser;

#[derive(Parser)]
#[command(name = "rustyed")]
#[command(version = "1.0")]
#[command(about = "Quite simple text editor", long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "CONFIG")]
    config: Option<PathBuf>,
    #[arg(short, long, value_name = "FILE", required = true)]
    file: PathBuf,
}

const DEF_CONF_PATH: &str = "./rustyed.conf.ini";

#[macroquad::main("Rustyed")]
async fn main() {
    let mut ctx: Context = Default::default();
    let args = Cli::parse();
    if args.file.is_file() {
        // init
        if let Some(config_path) = args.config.as_deref() {
            println!("Using this file for config: {}", config_path.display());
            init(&mut ctx, &config_path.display().to_string(), &args.file).await;
        } else {
            println!(
                "No config file specified! Using default config file: {}",
                DEF_CONF_PATH
            );
            init(&mut ctx, DEF_CONF_PATH, &args.file).await;
        }

        while !ctx.is_exit {
            update_state(&mut ctx).await;
            render(&mut ctx).await;
        }
    } else {
        eprintln!("Specified --file argument is not a file!");
    }
}
