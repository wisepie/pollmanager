#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use poll_manager::config::Config;
use poll_manager::gui;
use std::io::Error;

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.get_or_build_path()?;

    gui::start();
    Ok(())
}
