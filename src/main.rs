mod palette;
mod icon;

use palette::ColorPalette;
use icon::{IconTheme, Icon};
use std::path::PathBuf;

fn main() {
    let palette = ColorPalette::default();
    println!("Primary color: {}", palette.primary.to_hex());

    let mut theme = IconTheme::new("SigmaOS Default".to_string());
    theme.add_icon(Icon::new("test".to_string(), PathBuf::from("/test.png")));
    println!("Icon theme '{}' ready", theme.name);
}
