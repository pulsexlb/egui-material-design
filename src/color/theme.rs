use crate::utils::argb_to_color32;
use egui::Visuals;
pub use material_colors::{
    color::Argb,
    scheme::Scheme,
    theme::{Theme, ThemeBuilder},
};

/// Color mode for Material Design
#[derive(Clone, Copy)]
pub enum ColorMode {
    Light,
    Dark,
}

/// Material Color Theme
///
/// # Example
/// ```rust
/// // create a theme from argb code
/// let theme = MaterialTheme::from_argb(0xffaae5a4, ColorMode::Light);
/// // get scheme from theme
/// let scheme = theme.get();
/// ````
pub struct MaterialTheme(Theme, ColorMode);

impl MaterialTheme {
    /// Build through argb base colors
    pub fn from_argb(argb: u32, color_mode: ColorMode) -> Self {
        Self(
            ThemeBuilder::with_source(Argb::from_u32(argb)).build(),
            color_mode,
        )
    }

    /// Get light theme color schemes
    pub fn get_light_scheme(&self) -> &Scheme {
        &self.0.schemes.light
    }

    /// Get dark theme color schemes
    pub fn get_dark_scheme(&self) -> &Scheme {
        &self.0.schemes.dark
    }

    // Get the schemes in the current color mode
    pub fn get(&self) -> &Scheme {
        match self.1 {
            ColorMode::Light => self.get_light_scheme(),
            ColorMode::Dark => self.get_dark_scheme(),
        }
    }

    /// Get Color Mode
    pub fn get_dark_mode(&self) -> bool {
        match self.1 {
            ColorMode::Dark => true,
            ColorMode::Light => false,
        }
    }

    /// Set Color Mode
    pub fn set_dark_mode(&mut self, dark: bool) {
        match dark {
            true => {
                self.1 = ColorMode::Dark;
            }
            false => {
                self.1 = ColorMode::Light;
            }
        }
    }

    /// Apply it to your egui application
    pub fn apply(&self, ctx: &egui::Context) {
        let mut visuals = match self.1 {
            ColorMode::Light => Visuals::light(),
            ColorMode::Dark => Visuals::dark(),
        };
        let scheme = self.get();

        visuals.window_fill = argb_to_color32(scheme.surface);
        visuals.faint_bg_color = argb_to_color32(scheme.surface_container);
        visuals.extreme_bg_color = argb_to_color32(scheme.surface_variant);
        visuals.code_bg_color = argb_to_color32(scheme.surface_dim);
        visuals.panel_fill = argb_to_color32(scheme.surface_container_high);
        visuals.warn_fg_color = argb_to_color32(scheme.error_container);
        visuals.error_fg_color = argb_to_color32(scheme.error);

        visuals.override_text_color = Some(argb_to_color32(scheme.on_surface));
        visuals.hyperlink_color = argb_to_color32(scheme.primary);

        ctx.set_visuals(visuals);
    }
}
