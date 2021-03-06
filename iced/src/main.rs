#![warn(rust_2018_idioms)]
#![warn(clippy::all)]
#![windows_subsystem = "windows"]

use std::convert::TryInto;

use failure::Error;
use rebootinto_core as core;

use iced::{window, Application, Settings};

mod app;

type Result<T> = std::result::Result<T, Error>;

fn main() {
    if let Err(err) = run() {
        match std::env::var("PANIC_ON_ERROR") {
            Ok(ref val) if val == "true" => panic!("Error: {}", err),
            _ => {}
        }

        eprintln!("Error: {}", err);
        std::process::exit(1);
    }
}

fn estimate_window_height(items: usize) -> u32 {
    let padding = (app::LAYOUT_PADDING as u32) * 2;

    let displayed_items_estimate = items.try_into().unwrap_or(10);

    let spacing = if displayed_items_estimate > 0 {
        (displayed_items_estimate - 1) * (app::LAYOUT_SPACING as u32)
    } else {
        0
    };

    let assumed_button_height = 30;
    let content = displayed_items_estimate * assumed_button_height;

    padding + spacing + content
}

fn run() -> Result<()> {
    let mut backend = core::Backend::init()?;
    let load_options = backend
        .load_options()
        .collect::<Result<Vec<core::LoadOption>>>()?;

    let size = (350, estimate_window_height(load_options.len()));

    app::App::run(Settings {
        flags: app::Init {
            backend,
            load_options,
        },
        window: window::Settings {
            size,
            resizable: true,
            decorations: true,
        },
        antialiasing: false,
        default_font: None,
    });
    Ok(())
}
