//! Material Design Text Fields Style
use egui::Color32;
use material_colors::scheme::Scheme;

use crate::utils::argb_to_color32;

/// Material Text Field style
///
/// see alse: [material doc](https://m3.material.io/components/text-fields/specs#f967d3f6-0139-43f7-8336-510022684fd1)
pub struct MaterialTextFieldStyle {
    pub container_color: Color32,
    pub container_rounding: f32,
    pub outline_color: Color32,
    pub outline_width: f32,
    pub label_font_color: Color32,
    pub label_font_size: f32,
    pub label_font_line_height: f32,
    pub input_font_color: Color32,
    pub input_font_size: f32,
    pub input_font_line_height: f32,
    pub disabled_container_color: Color32,
    pub disabled_container_opacity: f32,
    pub disabled_label_font_color: Color32,
    pub disabled_label_font_opacity: f32,
    pub disabled_input_font_color: Color32,
    pub disabled_input_font_opacity: f32,
    pub disabled_outline_color: Color32,
    pub disabled_outline_width: f32,
    pub focused_label_font_color: Color32,
    pub focused_input_font_color: Color32,
    pub focused_outline_color: Color32,
    pub focused_outline_width: f32,
    pub error_container_color: Color32,
    pub error_label_font_color: Color32,
    pub error_input_font_color: Color32,
    pub error_outline_color: Color32,
    pub error_outline_width: f32,
    pub width: Option<f32>,
    pub height: Option<f32>,
}

impl MaterialTextFieldStyle {
    pub fn normal(scheme: &Scheme) -> Self {
        Self {
            container_color: argb_to_color32(scheme.surface_container_highest),
            container_rounding: 8.0,
            outline_color: argb_to_color32(scheme.on_surface_variant),
            outline_width: 2.0,
            label_font_color: argb_to_color32(scheme.on_surface_variant),
            label_font_size: 16.0,
            label_font_line_height: 15.0,
            input_font_color: argb_to_color32(scheme.on_surface),
            input_font_size: 16.0,
            input_font_line_height: 15.0,
            disabled_container_color: argb_to_color32(scheme.on_surface),
            disabled_container_opacity: 0.04,
            disabled_label_font_color: argb_to_color32(scheme.on_surface_variant),
            disabled_label_font_opacity: 0.38,
            disabled_input_font_color: argb_to_color32(scheme.on_surface),
            disabled_input_font_opacity: 0.38,
            disabled_outline_color: argb_to_color32(scheme.on_surface_variant),
            disabled_outline_width: 2.0,
            focused_label_font_color: argb_to_color32(scheme.on_surface_variant),
            focused_input_font_color: argb_to_color32(scheme.on_surface),
            focused_outline_color: argb_to_color32(scheme.primary),
            focused_outline_width: 2.0,
            error_container_color: argb_to_color32(scheme.error_container),
            error_label_font_color: argb_to_color32(scheme.error),
            error_input_font_color: argb_to_color32(scheme.on_error_container),
            error_outline_color: argb_to_color32(scheme.error),
            error_outline_width: 2.0,
            width: None,
            height: None,
        }
    }
}
