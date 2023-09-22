use editor::update_state;
use macroquad::miniquad::conf::Icon;
use macroquad::window::Conf;
use rfd::FileDialog;
use std::path::PathBuf;
use std::str::FromStr;
use undo::Record;
mod buffer;
mod config;
mod core;
mod editor;
mod render;
use crate::core::*;
use crate::render::render;

fn window_conf() -> Conf {
    Conf {
        window_title: "rustyed".to_owned(),
        window_resizable: true,
        icon: Some(Icon {
            small: include_bytes!("../assets/rustyed_icon16.rgba").to_owned(),
            medium: include_bytes!("../assets/rustyed_icon32.rgba").to_owned(),
            big: include_bytes!("../assets/rustyed_icon64.rgba").to_owned(),
        }),
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut ctx: Context = Default::default();
    let mut record = Record::new();
    if std::env::args().count() > 1 {
        let file = std::env::args().nth(1).unwrap();
        let path = PathBuf::from_str(&file)
            .and_then(|f| Ok(f))
            .expect("Document cannot open!");
        let config = PathBuf::from_str("./rustyed.conf")
            .expect("Config file (rustyed.conf) not found in current directory!");

        init(&mut ctx, &config, &path).await;

        while !ctx.is_exit {
            update_state(&mut ctx, &mut record).await;
            render(&mut ctx).await;
        }
    } else {
        if let Some(file) = FileDialog::new()
            .add_filter("text", &["txt", "rs"])
            .add_filter("rust", &["rs", "toml"])
            .set_directory("/")
            .pick_file()
        {
            let config = PathBuf::from_str("./rustyed.conf")
                .expect("Config file (rustyed.conf) not found in current directory!");

            init(&mut ctx, &config, &file).await;

            while !ctx.is_exit {
                update_state(&mut ctx, &mut record).await;
                render(&mut ctx).await;
            }
        } else {
            eprintln!("Invalid file selected!");
        }
    }
}
