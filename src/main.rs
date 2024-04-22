use std::env::{self, args};

use change_helix_theme::change_helix_theme;
use dotenvy::dotenv;

fn main() {
    dotenv().ok();

    let mut args = args();

    args.next();

    let new_theme = args
        .next()
        .expect("missing theme, please make sure to call this command with a theme");

    let helix_config_path = env::var("HELIX_CONFIG_PATH").expect("Error loading helix config, make sure there is an environment variable named HELIX_CONFIG_PATH");

    change_helix_theme(&helix_config_path, &new_theme).unwrap();
}
