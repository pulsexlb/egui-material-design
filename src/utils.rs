use egui::Color32;
use material_colors::color::Argb;

/// Convert Argb(from material-color) to Color32(from egui)
pub(crate) fn argb_to_color32(argb: Argb) -> Color32 {
    Color32::from_rgba_unmultiplied(argb.red, argb.green, argb.blue, argb.alpha)
}
