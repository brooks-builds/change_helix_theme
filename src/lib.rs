use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use eyre::{Context, ContextCompat, Result};
use toml_edit::{DocumentMut, Item, Value};

pub fn change_helix_theme(config_path: impl AsRef<Path>, new_theme: &str) -> Result<()> {
    let config_file =
        load_helix_config(config_path.as_ref()).context("loading the helix config into a file")?;
    let mut helix_config = config_file
        .parse::<DocumentMut>()
        .context("parsing config file into editable format")?;
    let theme_value = Value::from(new_theme);
    let theme = Item::Value(theme_value);

    helix_config
        .insert("theme", theme)
        .context("inserting theme")?;

    save_helix_config(config_path.as_ref(), helix_config.to_string().as_str())
        .context("saving helix config")?;

    reload_helix_config().context("reloading helix config")?;

    announce_helix_config_changed(new_theme).context("announce theme changed")?;

    Ok(())
}

fn load_helix_config(config_path: &Path) -> Result<String> {
    let mut file = File::open(config_path).context("opening helix config file")?;
    let mut config = String::new();

    file.read_to_string(&mut config)
        .context("reading helix config file to string")?;

    Ok(config)
}

fn save_helix_config(path: &Path, config: &str) -> Result<()> {
    let mut file =
        std::fs::File::create(path).context("opening helix config file in write mode")?;

    file.write(config.as_bytes())
        .context("writing config to file")?;
    Ok(())
}

fn reload_helix_config() -> Result<()> {
    std::process::Command::new("pkill")
        .arg("-USR1")
        .arg("hx")
        .output()
        .context("reloading helix config")?;

    Ok(())
}

fn announce_helix_config_changed(new_config: &str) -> Result<()> {
    std::process::Command::new("say")
        .arg("changing helix theme to ")
        .arg(new_config)
        .output()
        .context("speaking")?;

    Ok(())
}
