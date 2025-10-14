use std::io::Error;

use poll_manager::config::Config;
use poll_manager::gui;

fn main() -> Result<(), Error> {
    let mut config = Config::default();
    config.get_or_build_path()?;

    gui::start();
    Ok(())
}
