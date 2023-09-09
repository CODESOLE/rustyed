use editor::update_state;
use std::path::PathBuf;
use std::str::FromStr;
mod buffer;
mod config;
mod core;
mod editor;
mod render;
use crate::core::*;
use crate::render::render;

#[macroquad::main("Rustyed")]
async fn main() {
    let mut ctx: Context = Default::default();
    if std::env::args().count() > 1 {
        let file = std::env::args().nth(1).unwrap();
        let path = PathBuf::from_str(&file)
            .and_then(|f| Ok(f))
            .expect("Document cannot open!");
        let config = PathBuf::from_str("./rustyed.conf")
            .expect("Config file (rustyed.conf) not found in current directory!");
        // init
        init(&mut ctx, &config, &path).await;

        while !ctx.is_exit {
            update_state(&mut ctx).await;
            render(&mut ctx).await;
        }
    } else {
        eprintln!("File to open should be given as a command line argument!");
    }
}
