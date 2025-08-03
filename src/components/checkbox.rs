//! Material Design checkbox

use egui::{Color32, CornerRadius, Rect, Sense, Stroke, Vec2, Widget, pos2, text::LayoutJob, vec2};
use material_colors::scheme::Scheme;

use crate::utils::argb_to_color32;

/// Style for Material Design checkbox
///
/// See: [material design document](https://m3.material.io/components/checkbox/specs#fd29f662-6e61-4c1f-9b97-1145c3b33075)
pub struct MaterialCheckboxStyle {
    pub container_size: f32,
    pub container_rounding: f32,
    pub unselected_outline_color: Color32,
    pub unselected_outline_width: f32,
    pub selected_container_color: Color32,
    pub selected_outline_width: f32,
    pub unselected_error_outline_color: Color32,
    pub selected_error_container_color: Color32,
    pub icon_size: f32,
    pub icon_selected_color: Color32,
    pub icon_selected_error_color: Color32,
    pub layer_size: f32,
    pub layer_rounding: f32,
    pub disabled_unselected_outline_color: Color32,
    pub disabled_unselected_outline_width: f32,
    pub disabled_container_opacity: f32,
    pub disabled_selected_container_color: Color32,
    pub disabled_selected_container_opacity: f32,
    pub disabled_selected_outline_width: f32,
    pub disabled_selected_icon_color: Color32,
    pub hovered_unselected_outline_color: Color32,
    pub hovered_unselected_outline_width: f32,
    pub hovered_selected_container_color: Color32,
    pub hovered_selected_outline_width: f32,
    pub hovered_unselected_error_outline_color: Color32,
    pub hovered_selected_error_container_color: Color32,
    pub hovered_selected_layer_color: Color32,
    pub hovered_selected_layer_opacity: f32,
    pub hovered_unselected_layer_color: Color32,
    pub hovered_unselected_layer_opacity: f32,
    pub hovered_error_layer_color: Color32,
    pub hovered_error_layer_opacity: f32,
    pub hovered_selected_icon_color: Color32,
    pub hovered_error_icon_color: Color32,
    pub pressed_unselected_outline_color: Color32,
    pub pressed_unselected_outline_width: f32,
    pub pressed_selected_container_color: Color32,
    pub pressed_selected_outline_width: f32,
    pub pressed_unselected_error_outline_color: Color32,
    pub pressed_selected_error_container_color: Color32,
    pub pressed_unselected_layer_color: Color32,
    pub pressed_unselected_layer_opacity: f32,
    pub pressed_selected_layer_color: Color32,
    pub pressed_selected_layer_opacity: f32,
    pub pressed_error_layer_color: Color32,
    pub pressed_error_layer_opacity: f32,
    pub pressed_selected_icon_color: Color32,
    pub pressed_error_icon_color: Color32,
}

impl MaterialCheckboxStyle {
    fn normal(scheme: &Scheme) -> Self {
        Self {
            container_size: 18.0,
            container_rounding: 2.0,
            unselected_outline_color: argb_to_color32(scheme.on_surface_variant),
            unselected_outline_width: 2.0,
            selected_container_color: argb_to_color32(scheme.primary),
            selected_outline_width: 0.0,
            unselected_error_outline_color: argb_to_color32(scheme.error),
            selected_error_container_color: argb_to_color32(scheme.error),
            icon_size: 18.0,
            icon_selected_color: argb_to_color32(scheme.on_primary),
            icon_selected_error_color: argb_to_color32(scheme.on_error),
            layer_size: 40.0,
            layer_rounding: 20.0,
            disabled_unselected_outline_color: argb_to_color32(scheme.on_surface),
            disabled_unselected_outline_width: 2.0,
            disabled_container_opacity: 0.38,
            disabled_selected_container_color: argb_to_color32(scheme.on_surface),
            disabled_selected_container_opacity: 0.38,
            disabled_selected_outline_width: 0.0,
            disabled_selected_icon_color: argb_to_color32(scheme.on_surface),
            hovered_unselected_outline_color: argb_to_color32(scheme.on_surface),
            hovered_unselected_outline_width: 2.0,
            hovered_selected_container_color: argb_to_color32(scheme.primary),
            hovered_selected_outline_width: 0.0,
            hovered_unselected_error_outline_color: argb_to_color32(scheme.error),
            hovered_selected_error_container_color: argb_to_color32(scheme.error),
            hovered_selected_layer_color: argb_to_color32(scheme.primary),
            hovered_selected_layer_opacity: 0.08,
            hovered_unselected_layer_color: argb_to_color32(scheme.on_surface),
            hovered_unselected_layer_opacity: 0.08,
            hovered_error_layer_color: argb_to_color32(scheme.error),
            hovered_error_layer_opacity: 0.08,
            hovered_selected_icon_color: argb_to_color32(scheme.on_primary),
            hovered_error_icon_color: argb_to_color32(scheme.on_error),
            pressed_unselected_outline_color: argb_to_color32(scheme.on_surface),
            pressed_unselected_outline_width: 2.0,
            pressed_selected_container_color: argb_to_color32(scheme.primary),
            pressed_selected_outline_width: 0.0,
            pressed_unselected_error_outline_color: argb_to_color32(scheme.error),
            pressed_selected_error_container_color: argb_to_color32(scheme.error),
            pressed_unselected_layer_color: argb_to_color32(scheme.primary),
            pressed_unselected_layer_opacity: 0.1,
            pressed_selected_layer_color: argb_to_color32(scheme.on_surface),
            pressed_selected_layer_opacity: 0.1,
            pressed_error_layer_color: argb_to_color32(scheme.error),
            pressed_error_layer_opacity: 0.1,
            pressed_selected_icon_color: argb_to_color32(scheme.on_primary),
            pressed_error_icon_color: argb_to_color32(scheme.on_error),
        }
    }
}

/// Material Design Checkbox
pub struct MaterialCheckbox<'a> {
    checked: &'a mut bool,
    icon: String,
    disable: bool,
    error: bool,
    style: MaterialCheckboxStyle,
}

impl<'a> MaterialCheckbox<'a> {
    pub fn new(checked: &'a mut bool, scheme: &Scheme) -> Self {
        Self {
            checked,
            icon: String::from("✔"),
            disable: false,
            error: false,
            style: MaterialCheckboxStyle::normal(scheme),
        }
    }

    pub fn with_disable(self, disable: bool) -> Self {
        Self { disable, ..self }
    }
}

impl Widget for MaterialCheckbox<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let Self {
            checked,
            icon,
            disable,
            error,
            style,
        } = self;

        let sense = if disable {
            Sense::hover()
        } else {
            Sense::click()
        };
        let layer_size = vec2(style.layer_size, style.layer_size);
        let (layer_rect, response) = ui.allocate_exact_size(layer_size, sense);

        let is_pressed = response.is_pointer_button_down_on();
        let is_hovering = response.hovered();

        // 状态切换
        if !disable && response.clicked() {
            *checked = !*checked;
        }

        let center = layer_rect.center();
        // 容器矩形（实际 checkbox 框）
        let container_rect =
            Rect::from_center_size(center, vec2(style.container_size, style.container_size));
        let rounding = CornerRadius::from(style.container_rounding);
        let layer_rounding = CornerRadius::from(style.layer_rounding);
        // 计算颜色
        let (container_fill, outline_stroke, icon_color, layer_color, layer_opacity) = {
            let mut fill = Color32::TRANSPARENT;
            let mut stroke = Stroke::NONE;
            let mut icon = Color32::TRANSPARENT;
            let mut layer = Color32::TRANSPARENT;
            let mut opacity = 0.0;

            if disable {
                // 禁用状态
                if *checked {
                    fill = style
                        .disabled_selected_container_color
                        .linear_multiply(style.disabled_selected_container_opacity);
                    stroke = Stroke::new(style.disabled_selected_outline_width, fill);
                    icon = style.disabled_selected_icon_color;
                } else {
                    stroke = Stroke::new(
                        style.disabled_unselected_outline_width,
                        style.disabled_unselected_outline_color,
                    );
                }
            } else {
                // 正常启用状态
                if *checked {
                    fill = if error {
                        style.selected_error_container_color
                    } else {
                        style.selected_container_color
                    };
                    stroke = Stroke::new(style.selected_outline_width, fill);
                    icon = if error {
                        style.icon_selected_error_color
                    } else {
                        style.icon_selected_color
                    };
                } else {
                    stroke = Stroke::new(
                        style.unselected_outline_width,
                        if error {
                            style.unselected_error_outline_color
                        } else {
                            style.unselected_outline_color
                        },
                    );
                }

                // 交互层颜色（hover / pressed）
                if is_pressed {
                    layer = if *checked {
                        style.pressed_selected_layer_color
                    } else {
                        style.pressed_unselected_layer_color
                    };
                    opacity = style
                        .pressed_selected_layer_opacity
                        .max(style.pressed_unselected_layer_opacity);
                    if !*checked {
                        stroke.color = style.pressed_unselected_outline_color;
                        stroke.width = style.pressed_unselected_outline_width;
                    } else {
                        stroke.width = style.pressed_selected_outline_width;
                    }
                    icon = if *checked {
                        style.pressed_selected_icon_color
                    } else if error {
                        style.pressed_error_icon_color
                    } else {
                        icon
                    };
                } else if is_hovering {
                    layer = if *checked {
                        style.hovered_selected_layer_color
                    } else {
                        style.hovered_unselected_layer_color
                    };
                    opacity = if *checked {
                        style.hovered_selected_layer_opacity
                    } else {
                        style.hovered_unselected_layer_opacity
                    };
                    if !*checked {
                        stroke.color = style.hovered_unselected_outline_color;
                        stroke.width = style.hovered_unselected_outline_width;
                    } else {
                        stroke.width = style.hovered_selected_outline_width;
                    }
                    icon = if *checked {
                        style.hovered_selected_icon_color
                    } else if error {
                        style.hovered_error_icon_color
                    } else {
                        icon
                    };
                }
            }

            (fill, stroke, icon, layer, opacity)
        };

        // 绘制
        if ui.is_rect_visible(layer_rect) {
            let painter = ui.painter();

            // 绘制 state layer（悬停/按下反馈）
            if layer_opacity > 0.0 && layer_color != Color32::TRANSPARENT {
                let layer_color = layer_color.linear_multiply(layer_opacity);
                painter.rect_filled(layer_rect, layer_rounding, layer_color);
            }

            // 绘制容器（边框或填充）
            painter.rect(
                container_rect,
                rounding,
                container_fill,
                outline_stroke,
                egui::StrokeKind::Inside,
            );

            // 绘制选中图标
            if *checked {
                let font_id = egui::FontId::monospace(style.icon_size); // 使用等宽字体避免偏移
                let layout_job = LayoutJob::simple(icon, font_id, icon_color, 0.0);
                // let layout_job = egui::TextFormat::simple(font_id, icon_color)(
                //     ui.ctx(),
                //     icon,
                //     Vec2::splat(f32::INFINITY),
                // );
                let galley = ui.fonts(|f| f.layout_job(layout_job));

                let icon_pos = pos2(
                    container_rect.center().x - galley.size().x / 2.0,
                    container_rect.center().y - galley.size().y / 2.0,
                );

                painter.galley(icon_pos, galley, icon_color);
            }
        }

        response
    }
}
