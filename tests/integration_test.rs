// Integration test for SigmaOS-theme-color
use sigmaos_theme_color::{Palette, Component};

#[test]
fn test_palette_integration() {
    let mocha = Palette::catppuccin_mocha();
    let mint = Palette::mint_emerald();
    let arch = Palette::arch_minimal();

    assert_eq!(mocha.name, "Catppuccin Mocha");
    assert_eq!(mint.name, "Mint-Y Emerald");
    assert_eq!(arch.name, "Arch Minimal");

    let css = mocha.render_css();
    assert!(css.contains("@define-color background #1e1e2e;"));
}

#[test]
fn test_color_contrast_accessible() {
    let mocha = Palette::catppuccin_mocha();
    let contrast = mocha.background.contrast_ratio(&mocha.foreground);
    assert!(contrast >= 4.5, "Text on background must meet WCAG AA contrast (got {})", contrast);
}

#[test]
fn test_component_handle() {
    let comp = Component::new();
    assert_eq!(comp.name(), "SigmaOS Theme Color Engine");
}
