// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-theme-color — Palette and Accent Colour Engine for SigmaOS Omarchy & Mint Personas.
//!
//! Provides color parsing (hex/RGB), palette harmonization, WCAG contrast checking,
//! and stylesheet/compositor color generation for Wayland desktop themes.

#![forbid(unsafe_code)]

use std::fmt;
use std::collections::BTreeMap;

/// Errors for color operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ColorError {
    InvalidHex(String),
    EmptyName,
    NotFound(String),
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHex(s) => write!(f, "invalid hex color: '{s}' (expected #RRGGBB or #RGB)"),
            Self::EmptyName => write!(f, "color/palette name must not be empty"),
            Self::NotFound(s) => write!(f, "palette not found: '{s}'"),
        }
    }
}

impl std::error::Error for ColorError {}

/// An RGB color representation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RgbColor {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl RgbColor {
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    /// Parse hex color like "#cba6f7", "cba6f7", or "#f00".
    pub fn parse_hex(hex: &str) -> Result<Self, ColorError> {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() == 6 {
            let r = u8::from_str_radix(&clean[0..2], 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            let g = u8::from_str_radix(&clean[2..4], 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            let b = u8::from_str_radix(&clean[4..6], 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            Ok(Self { r, g, b })
        } else if clean.len() == 3 {
            let r_ch = &clean[0..1];
            let g_ch = &clean[1..2];
            let b_ch = &clean[2..3];
            let r = u8::from_str_radix(&format!("{r_ch}{r_ch}"), 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            let g = u8::from_str_radix(&format!("{g_ch}{g_ch}"), 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            let b = u8::from_str_radix(&format!("{b_ch}{b_ch}"), 16)
                .map_err(|_| ColorError::InvalidHex(hex.to_string()))?;
            Ok(Self { r, g, b })
        } else {
            Err(ColorError::InvalidHex(hex.to_string()))
        }
    }

    /// Return 6-character lowercase hex string without hash.
    pub fn to_hex(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Return hex string with `#` prefix.
    pub fn to_hex_hash(&self) -> String {
        format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }

    /// Compute relative luminance according to WCAG 2.1 formula.
    pub fn relative_luminance(&self) -> f32 {
        fn channel(c: u8) -> f32 {
            let s = (c as f32) / 255.0;
            if s <= 0.03928 {
                s / 12.92
            } else {
                ((s + 0.055) / 1.055).powf(2.4)
            }
        }
        0.2126 * channel(self.r) + 0.7152 * channel(self.g) + 0.0722 * channel(self.b)
    }

    /// Calculate contrast ratio between two colors (range 1.0 to 21.0).
    pub fn contrast_ratio(&self, other: &Self) -> f32 {
        let l1 = self.relative_luminance();
        let l2 = other.relative_luminance();
        let lighter = l1.max(l2);
        let darker = l1.min(l2);
        (lighter + 0.05) / (darker + 0.05)
    }
}

/// A comprehensive desktop color palette.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Palette {
    pub name: String,
    pub background: RgbColor,
    pub foreground: RgbColor,
    pub accent: RgbColor,
    pub secondary: RgbColor,
    pub alert: RgbColor,
    pub success: RgbColor,
    pub surface: RgbColor,
}

impl Palette {
    /// Catppuccin Mocha preset (Omarchy default).
    pub fn catppuccin_mocha() -> Self {
        Self {
            name: "Catppuccin Mocha".to_string(),
            background: RgbColor::new(0x1e, 0x1e, 0x2e),
            foreground: RgbColor::new(0xcd, 0xd6, 0xf4),
            accent: RgbColor::new(0xcb, 0xa6, 0xf7),
            secondary: RgbColor::new(0x89, 0xb4, 0xfa),
            alert: RgbColor::new(0xf3, 0x8b, 0xa8),
            success: RgbColor::new(0xa6, 0xe3, 0xa1),
            surface: RgbColor::new(0x31, 0x32, 0x44),
        }
    }

    /// Linux Mint-Y Emerald preset.
    pub fn mint_emerald() -> Self {
        Self {
            name: "Mint-Y Emerald".to_string(),
            background: RgbColor::new(0x2f, 0x34, 0x3f),
            foreground: RgbColor::new(0xdf, 0xdf, 0xdf),
            accent: RgbColor::new(0x8f, 0xbc, 0x8f),
            secondary: RgbColor::new(0x5a, 0x9e, 0x6e),
            alert: RgbColor::new(0xcc, 0x57, 0x5d),
            success: RgbColor::new(0x73, 0xd2, 0x16),
            surface: RgbColor::new(0x38, 0x3c, 0x4a),
        }
    }

    /// Arch Minimalist preset.
    pub fn arch_minimal() -> Self {
        Self {
            name: "Arch Minimal".to_string(),
            background: RgbColor::new(0x12, 0x14, 0x17),
            foreground: RgbColor::new(0xee, 0xf0, 0xf2),
            accent: RgbColor::new(0x17, 0x93, 0xd1),
            secondary: RgbColor::new(0x33, 0xaa, 0xdd),
            alert: RgbColor::new(0xe0, 0x6c, 0x75),
            success: RgbColor::new(0x98, 0xc3, 0x79),
            surface: RgbColor::new(0x21, 0x24, 0x2b),
        }
    }

    /// Generate Waybar / GTK CSS variable definitions.
    pub fn render_css(&self) -> String {
        format!(
            "/* Palette: {} */\n\
             @define-color background {};\n\
             @define-color foreground {};\n\
             @define-color primary {};\n\
             @define-color secondary {};\n\
             @define-color alert {};\n\
             @define-color success {};\n\
             @define-color surface {};\n",
            self.name,
            self.background.to_hex_hash(),
            self.foreground.to_hex_hash(),
            self.accent.to_hex_hash(),
            self.secondary.to_hex_hash(),
            self.alert.to_hex_hash(),
            self.success.to_hex_hash(),
            self.surface.to_hex_hash(),
        )
    }

    /// Generate Hyprland colors.conf.
    pub fn render_hyprland(&self) -> String {
        format!(
            "# Palette: {}\n\
             $background = rgb({})\n\
             $foreground = rgb({})\n\
             $accent = rgb({})\n\
             $secondary = rgb({})\n",
            self.name,
            self.background.to_hex(),
            self.foreground.to_hex(),
            self.accent.to_hex(),
            self.secondary.to_hex(),
        )
    }
}

/// Errors for `AssetError` operations.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssetError {
    EmptyName,
    EmptyPath(String),
    Duplicate(String),
    NotFound(String),
}

impl fmt::Display for AssetError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "asset name must not be empty"),
            Self::EmptyPath(x) => write!(f, "asset path must not be empty: {x}"),
            Self::Duplicate(x) => write!(f, "asset name already registered: {x}"),
            Self::NotFound(x) => write!(f, "asset not registered: {x}"),
        }
    }
}

impl std::error::Error for AssetError {}

/// Registry mapping asset names to resource paths.
#[derive(Debug, Default)]
pub struct AssetRegistry {
    assets: BTreeMap<String, String>,
}

impl AssetRegistry {
    pub fn new() -> Self { Self::default() }

    pub fn register(&mut self, name: impl Into<String>, path: impl Into<String>) -> Result<(), AssetError> {
        let name = name.into();
        let path = path.into();
        if name.trim().is_empty() { return Err(AssetError::EmptyName); }
        if path.trim().is_empty() { return Err(AssetError::EmptyPath(name)); }
        if self.assets.contains_key(&name) { return Err(AssetError::Duplicate(name)); }
        self.assets.insert(name, path);
        Ok(())
    }

    pub fn resolve(&self, name: &str) -> Result<&str, AssetError> {
        self.assets.get(name).map(String::as_str)
            .ok_or_else(|| AssetError::NotFound(name.to_string()))
    }

    pub fn len(&self) -> usize { self.assets.len() }
    pub fn is_empty(&self) -> bool { self.assets.is_empty() }
}

/// Backwards-compatible component handle.
#[derive(Debug, Default)]
pub struct Component {
    name: &'static str,
}

impl Component {
    pub fn new() -> Self {
        Self { name: "SigmaOS Theme Color Engine" }
    }

    pub fn name(&self) -> &'static str {
        self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex() {
        let c1 = RgbColor::parse_hex("#cba6f7").unwrap();
        assert_eq!(c1, RgbColor::new(0xcb, 0xa6, 0xf7));
        assert_eq!(c1.to_hex(), "cba6f7");
        assert_eq!(c1.to_hex_hash(), "#cba6f7");

        let c2 = RgbColor::parse_hex("#fff").unwrap();
        assert_eq!(c2, RgbColor::new(255, 255, 255));
    }

    #[test]
    fn test_contrast_ratio() {
        let black = RgbColor::new(0, 0, 0);
        let white = RgbColor::new(255, 255, 255);
        let ratio = black.contrast_ratio(&white);
        assert!(ratio >= 20.0);
    }

    #[test]
    fn test_palette_generation() {
        let catppuccin = Palette::catppuccin_mocha();
        let css = catppuccin.render_css();
        assert!(css.contains("@define-color primary #cba6f7"));
        let hypr = catppuccin.render_hyprland();
        assert!(hypr.contains("$accent = rgb(cba6f7)"));
    }

    #[test]
    fn test_asset_registry() {
        let mut r = AssetRegistry::new();
        r.register("wallpaper", "/usr/share/wall.png").unwrap();
        assert_eq!(r.resolve("wallpaper").unwrap(), "/usr/share/wall.png");
    }
}
