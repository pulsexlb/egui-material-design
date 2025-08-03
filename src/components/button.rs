//! Material Desigin Buttons

use egui::{Color32, Response, Sense, Ui, Widget};
use material_colors::scheme::Scheme;

use crate::utils::argb_to_color32;

/// Material Button Style
///
/// See: [material design document](https://m3.material.io/components/buttons/specs#08e8cb37-ac4e-49b8-82af-c77421c834ee)
pub struct MaterialButtonStyle {
    pub container_color: Color32,
    pub shadow_color: Color32,
    pub label_color: Color32,
    pub icon_color: Color32,
    pub container_height: f32,
    pub font_size: f32,
    pub line_height: f32,
    pub icon_size: f32,
    pub rounding: f32,
    pub pressed_rounding: f32,
    pub padding: f32,
    pub leading_space: f32,
    pub between_icon_label_space: f32,
    pub displayed_container_color: Color32,
    pub displayed_container_opacity: f32,
    pub displayed_label_color: Color32,
    pub displayed_label_opacity: f32,
    pub displayed_icon_color: Color32,
    pub displayed_icon_opacity: f32,
    pub hovered_container_layer_color: Color32,
    pub hovered_container_layer_opacity: f32,
    pub hovered_label_color: Color32,
    pub hovered_icon_color: Color32,
    pub pressed_container_layer_color: Color32,
    pub pressed_container_layer_opacity: f32,
    pub pressed_label_color: Color32,
    pub pressed_icon_color: Color32,
}

impl MaterialButtonStyle {
    /// default color for button
    pub fn normal(scheme: &Scheme) -> Self {
        Self {
            container_color: argb_to_color32(scheme.primary),
            shadow_color: argb_to_color32(scheme.shadow),
            label_color: argb_to_color32(scheme.on_primary),
            icon_color: argb_to_color32(scheme.on_primary),
            container_height: 40.0,
            font_size: 14.0,
            line_height: 20.0,
            icon_size: 20.0,
            rounding: 20.0,
            pressed_rounding: 8.0,
            padding: 15.0,
            leading_space: 24.0,
            between_icon_label_space: 8.0,
            displayed_container_color: argb_to_color32(scheme.on_surface),
            displayed_container_opacity: 0.1,
            displayed_label_color: argb_to_color32(scheme.on_surface),
            displayed_label_opacity: 0.38,
            displayed_icon_color: argb_to_color32(scheme.on_surface),
            displayed_icon_opacity: 0.38,
            hovered_container_layer_color: argb_to_color32(scheme.on_primary),
            hovered_container_layer_opacity: 0.08,
            hovered_label_color: argb_to_color32(scheme.on_primary),
            hovered_icon_color: argb_to_color32(scheme.on_primary),
            pressed_container_layer_color: argb_to_color32(scheme.on_primary),
            pressed_container_layer_opacity: 0.1,
            pressed_label_color: argb_to_color32(scheme.on_primary),
            pressed_icon_color: argb_to_color32(scheme.on_primary),
        }
    }
}

/// Material Design Button
pub struct MaterialButton {
    pub text: String,
    pub icon: Option<u32>, // todo: add icon support
    pub style: MaterialButtonStyle,
    pub display: bool,
}

impl MaterialButton {
    pub fn new(text: String, scheme: &Scheme) -> Self {
        Self {
            text,
            icon: None,
            style: MaterialButtonStyle::normal(scheme),
            display: false,
        }
    }
}

impl Widget for MaterialButton {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            text,
            icon: _icon,
            style,
            display,
        } = self;

        let text_size = ui.fonts(|f| {
            f.layout_no_wrap(
                text.clone(),
                egui::FontId::monospace(style.font_size),
                style.label_color,
            )
            .size()
        });

        // 创建一个“按钮区域”
        let button_width = text_size.x + 2.0 * style.padding;
        let button_height = text_size.y + 2.0 * style.padding;
        let desired_size = egui::vec2(button_width, button_height);

        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());

        // 获取当前交互状态
        let is_pressed = response.is_pointer_button_down_on();
        let is_hovering = response.hovered();

        // 计算当前背景色
        let bg_color = if is_pressed {
            style.container_color.lerp_to_gamma(
                style.pressed_container_layer_color,
                style.pressed_container_layer_opacity,
            )
        } else if is_hovering {
            style.container_color.lerp_to_gamma(
                style.hovered_container_layer_color,
                style.hovered_container_layer_opacity,
            )
        } else if display {
            style
                .displayed_container_color
                .linear_multiply(style.displayed_container_opacity)
        } else {
            style.container_color
        };

        // 默认圆角
        let rounding = if is_pressed {
            style.pressed_rounding
        } else {
            style.rounding
        };

        // 绘制按钮外观
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();

            // 绘制带圆角的背景
            painter.rect_filled(rect, rounding, bg_color);

            // 绘制文字（居中）
            let text_color = style.label_color;

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                text,
                egui::FontId::monospace(style.font_size),
                text_color,
            );
        }

        response
    }
}
