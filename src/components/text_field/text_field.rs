//! Material Design Text Fields
use std::sync::Arc;

use egui::emath::TSTransform;
use egui::os::OperatingSystem;
use egui::output::OutputEvent;
use egui::text::{CCursor, CCursorRange, LayoutJob};
use egui::text_selection::text_cursor_state::cursor_rect;
use egui::text_selection::visuals::paint_text_selection;
use egui::{
    Align2, Context, CornerRadius, CursorIcon, Event, EventFilter, FontId, Galley, Id, ImeEvent,
    Key, KeyboardShortcut, Margin, Modifiers, NumExt, Rect, Sense, Stroke, TextBuffer, Ui, Widget,
    WidgetText, pos2, response, text_selection, vec2,
};
use material_colors::scheme::Scheme;

use super::MaterialTextFieldStyle;
use super::{output::TextEditOutput, state::TextEditState};

/// Material Design Text Field
pub struct MaterialTextField<'t> {
    pub text: &'t mut String,
    pub label: Option<String>,
    pub style: MaterialTextFieldStyle,
    pub disable: bool,
    pub error: bool,
    pub multiline: bool,
    pub password: bool,
}

impl MaterialTextField<'_> {
    pub fn load_state(ctx: &Context, id: Id) -> Option<TextEditState> {
        TextEditState::load(ctx, id)
    }

    pub fn store_state(ctx: &Context, id: Id, state: TextEditState) {
        state.store(ctx, id);
    }
}

impl<'t> MaterialTextField<'t> {
    pub fn singleline(text: &'t mut String, scheme: &Scheme) -> Self {
        Self {
            text,
            label: None,
            style: MaterialTextFieldStyle::normal(scheme),
            disable: false,
            error: false,
            multiline: false,
            password: false,
        }
    }

    pub fn multiline(text: &'t mut String, scheme: &Scheme) -> Self {
        Self {
            text,
            label: None,
            style: MaterialTextFieldStyle::normal(scheme),
            disable: false,
            error: false,
            multiline: true,
            password: false,
        }
    }

    pub fn with_disable(self, disable: bool) -> Self {
        Self { disable, ..self }
    }

    pub fn with_width(self, width: f32) -> Self {
        Self {
            style: MaterialTextFieldStyle {
                width: Some(width),
                ..self.style
            },
            ..self
        }
    }

    pub fn with_height(self, height: f32) -> Self {
        Self {
            style: MaterialTextFieldStyle {
                height: Some(height),
                ..self.style
            },
            ..self
        }
    }

    pub fn with_label(self, label: String) -> Self {
        Self {
            label: Some(label),
            ..self
        }
    }
}

impl Widget for MaterialTextField<'_> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        self.show(ui).response
    }
}

impl MaterialTextField<'_> {
    fn show(self, ui: &mut egui::Ui) -> TextEditOutput {
        self.show_content(ui)
    }

    fn show_content(self, ui: &mut egui::Ui) -> TextEditOutput {
        let Self {
            text,
            label,
            style,
            disable,
            error,
            multiline,
            password,
        } = self;

        // 对齐,是否换行方式
        let (_align, clip_text) = if multiline {
            (Align2::LEFT_TOP, false)
        } else {
            (Align2::LEFT_CENTER, true)
        };

        // 确定当前状态的颜色
        let (container_color, outline_color, outline_width, label_color, input_color) = if disable {
            (
                style
                    .disabled_container_color
                    .linear_multiply(style.disabled_container_opacity),
                style.disabled_outline_color,
                style.disabled_outline_width,
                style
                    .disabled_label_font_color
                    .linear_multiply(style.disabled_label_font_opacity),
                style
                    .disabled_input_font_color
                    .linear_multiply(style.disabled_input_font_opacity),
            )
        } else if error {
            (
                style.error_container_color,
                style.error_outline_color,
                style.error_outline_width,
                style.error_label_font_color,
                style.error_input_font_color,
            )
        } else {
            (
                style.container_color,
                style.outline_color,
                style.outline_width,
                style.label_font_color,
                style.input_font_color,
            )
        };

        let font_id = FontId::monospace(style.input_font_size);
        let row_height = ui.fonts(|f| f.row_height(&font_id));

        // 计算大小
        let desired_width = style.width.unwrap_or_else(|| ui.available_width());
        let desired_height = style.height.unwrap_or_else(|| {
            if multiline {
                style.input_font_line_height * 4.0 + 16.0 // 默认4行高度
            } else {
                style.input_font_line_height + 16.0 // 单行高度
            }
        });

        // 排列
        let mut layouter = move |ui: &Ui, text: &dyn TextBuffer, wrap_width: f32| {
            let text = mask_if_password(password, text.as_str());
            let layout_job = if multiline {
                LayoutJob::simple(text, font_id.clone(), input_color, wrap_width)
            } else {
                LayoutJob::simple_singleline(text, font_id.clone(), input_color)
            };
            ui.fonts(|f| f.layout_job(layout_job))
        };

        // galley
        let mut galley = layouter(ui, text, desired_width);

        // id
        let (id, rect) = ui.allocate_space(vec2(desired_width, desired_height));
        // 状态
        let mut state = TextEditState::load(ui.ctx(), id).unwrap_or_default();
        let allow_drag_to_select =
            ui.input(|i| !i.has_touch_screen()) || ui.memory(|mem| mem.has_focus(id));
        let sense = if !disable {
            if allow_drag_to_select {
                Sense::click_and_drag()
            } else {
                Sense::click()
            }
        } else {
            Sense::hover()
        };

        // 交互响应
        let mut response = ui.interact(rect, id, sense);
        response.intrinsic_size = Some(vec2(desired_width, desired_height));
        response.flags -= response::Flags::FAKE_PRIMARY_CLICKED;
        // 计算内边距
        let padding = vec2(16.0, 12.0);
        let inner_rect = rect.shrink2(padding);

        // 原始文本
        let prev_text = text.as_str().to_owned();

        // 绘制器
        let painter = ui.painter_at(rect.expand(1.0));
        // 交互逻辑
        if !disable {
            if let Some(pointer_pos) = ui.ctx().pointer_interact_pos() {
                // 如果悬停且文本可变，设置可变文本标志
                if response.hovered() && text.is_mutable() {
                    ui.output_mut(|o| o.mutable_text_under_cursor = true);
                }
                // 计算指针位置对应的光标位置
                let cursor_at_pointer =
                    galley.cursor_from_pos(pointer_pos - rect.min + state.text_offset);
                // 如果启用了光标预览且指针在移动，显示光标预览
                if ui.visuals().text_cursor.preview
                    && response.hovered()
                    && ui.input(|i| i.pointer.is_moving())
                {
                    // text cursor preview:
                    let cursor_rect = TSTransform::from_translation(rect.min.to_vec2())
                        * cursor_rect(&galley, &cursor_at_pointer, row_height);
                    text_selection::visuals::paint_cursor_end(&painter, ui.visuals(), cursor_rect);
                }
                // 检查是否正在拖动
                let is_being_dragged = ui.ctx().is_being_dragged(response.id);
                // 处理光标交互
                let did_interact = state.cursor.pointer_interaction(
                    ui,
                    &response,
                    cursor_at_pointer,
                    &galley,
                    is_being_dragged,
                );
                // 如果有交互或点击，请求焦点并记录交互时间
                if did_interact || response.clicked() {
                    ui.memory_mut(|mem| mem.request_focus(response.id));
                    state.last_interaction_time = ui.ctx().input(|i| i.time);
                }
            }
        }

        // 如果可交互且悬停，设置文本光标图标
        if !disable && response.hovered() {
            ui.ctx().set_cursor_icon(CursorIcon::Text);
        }

        let mut cursor_range = None;
        let prev_cursor_range = state.cursor.range(&galley);
        // 处理有焦点时的输入事件
        if !disable && ui.memory(|mem| mem.has_focus(id)) {
            let event_filter = EventFilter {
                // moving the cursor is really important
                horizontal_arrows: true,
                vertical_arrows: true,
                tab: false, // tab is used to change focus, not to insert a tab character
                ..Default::default()
            };
            let cursor_at_end = true;
            // 设置焦点锁定过滤器
            ui.memory_mut(|mem| mem.set_focus_lock_filter(id, event_filter));
            // 确定默认光标范围：在末尾或默认位置
            let default_cursor_range = if cursor_at_end {
                CCursorRange::one(galley.end())
            } else {
                CCursorRange::default()
            };
            // 处理键盘输入事件，返回是否更改了文本和新的光标范围
            let (changed, new_cursor_range) = events(
                ui,
                &mut state,
                text,
                &mut galley,
                &mut layouter,
                id,
                desired_width,
                multiline,
                password,
                default_cursor_range,
                usize::MAX,
                event_filter,
                None,
            );
            // 如果文本更改，标记响应为已更改
            if changed {
                response.mark_changed();
            }
            // 保存新的光标范围
            cursor_range = Some(new_cursor_range);
        }

        // 计算文本排版位置：根据对齐方式在矩形内定位
        let mut galley_pos = if multiline {
            // 多行文本从顶部开始
            pos2(inner_rect.min.x, inner_rect.min.y)
        } else {
            // 单行文本垂直居中
            pos2(inner_rect.min.x, inner_rect.center().y - row_height / 2.0)
        };
        // 计算对齐偏移量
        let align_offset = rect.left_top() - galley_pos;

        // 处理单行文本的视觉裁剪（当文本比输入框宽时）
        if clip_text && align_offset.x == 0.0 {
            // 获取光标位置
            let cursor_pos = match (cursor_range, ui.memory(|mem| mem.has_focus(id))) {
                (Some(cursor_range), true) => galley.pos_from_cursor(cursor_range.primary).min.x,
                _ => 0.0,
            };

            // 计算滚动偏移量
            let mut offset_x = state.text_offset.x;
            let visible_range = offset_x..=offset_x + desired_width;

            // 如果光标不在可见范围内，调整偏移量
            if !visible_range.contains(&cursor_pos) {
                if cursor_pos < *visible_range.start() {
                    offset_x = cursor_pos;
                } else {
                    offset_x = cursor_pos - desired_width;
                }
            }

            // 限制偏移量在合理范围内
            offset_x = offset_x
                .at_most(galley.size().x - desired_width)
                .at_least(0.0);

            // 更新状态中的偏移量
            state.text_offset = vec2(offset_x, align_offset.y);

            // 应用偏移量到排版位置
            galley_pos -= vec2(offset_x, 0.0);
        } else {
            state.text_offset = align_offset;
        }

        // 检查选择范围是否改变
        let selection_changed = if let (Some(cursor_range), Some(prev_cursor_range)) =
            (cursor_range, prev_cursor_range)
        {
            prev_cursor_range != cursor_range
        } else {
            false
        };

        // 矩形可见时进行绘制
        if ui.is_rect_visible(rect) {
            // 绘制容器背景
            painter.rect_filled(
                rect,
                CornerRadius::same(style.container_rounding as u8),
                container_color,
            );
            // 绘制边框
            let has_focus = response.has_focus();
            let (final_outline_color, final_outline_width) = if has_focus && !disable {
                (style.focused_outline_color, style.focused_outline_width)
            } else {
                (outline_color, outline_width)
            };
            painter.rect_stroke(
                rect,
                CornerRadius::same(style.container_rounding as u8),
                Stroke::new(final_outline_width, final_outline_color),
                egui::StrokeKind::Middle,
            );

            // 绘制标签（当文本为空且没有焦点时）
            if let Some(ref label_text) = label {
                if text.is_empty() && !has_focus {
                    let label_font =
                        egui::FontId::new(style.label_font_size, egui::FontFamily::Monospace);
                    let label_galley = if multiline {
                        WidgetText::from(label_text).into_galley(
                            ui,
                            Some(egui::TextWrapMode::Wrap),
                            desired_width,
                            label_font,
                        )
                    } else {
                        WidgetText::from(label_text).into_galley(
                            ui,
                            Some(egui::TextWrapMode::Extend),
                            f32::INFINITY,
                            label_font,
                        )
                    };
                    let label_pos = pos2(
                        inner_rect.min.x,
                        inner_rect.center().y - label_galley.size().y / 2.0,
                    );
                    painter.galley(label_pos, label_galley, label_color);
                }
            }
            // 如果有焦点且有选择范围，绘制选择高亮
            if has_focus {
                if let Some(cursor_range) = state.cursor.range(&galley) {
                    // Add text selection rectangles to the galley:
                    paint_text_selection(&mut galley, ui.visuals(), &cursor_range, None);
                }
            }
            // 如果不裁剪文本且编辑导致尺寸变化，分配额外空间
            if !clip_text {
                let extra_size = galley.size() - rect.size();
                if extra_size.x > 0.0 || extra_size.y > 0.0 {
                    ui.allocate_rect(Rect::from_min_size(rect.max, extra_size), Sense::hover());
                }
            }
            // 显示用户输入文本
            if !text.is_empty() {
                if has_focus && !disable {
                    painter.galley(galley_pos, galley.clone(), style.focused_input_font_color);
                } else {
                    painter.galley(galley_pos, galley.clone(), input_color);
                };
            }

            // 如果有焦点，绘制光标和处理相关逻辑
            if has_focus {
                if let Some(cursor_range) = state.cursor.range(&galley) {
                    // 计算主光标矩形
                    let primary_cursor_rect =
                        cursor_rect(&galley, &cursor_range.primary, row_height)
                            .translate(galley_pos.to_vec2());

                    // 如果文本更改或选择更改，滚动到光标位置
                    if response.changed() || selection_changed {
                        ui.scroll_to_rect(primary_cursor_rect + Margin::symmetric(0, 0), None);
                    }

                    // 如果文本可变且可交互，绘制闪烁光标
                    if text.is_mutable() && !disable {
                        let now = ui.ctx().input(|i| i.time);
                        if response.changed() || selection_changed {
                            state.last_interaction_time = now;
                        }

                        // 只有当egui视口有焦点时才显示（和闪烁）光标
                        let viewport_has_focus = ui.ctx().input(|i| i.focused);
                        if viewport_has_focus {
                            text_selection::visuals::paint_text_cursor(
                                ui,
                                &painter,
                                primary_cursor_rect,
                                now - state.last_interaction_time,
                            );
                        }

                        // 设置IME输出（屏幕坐标）
                        let to_global = ui
                            .ctx()
                            .layer_transform_to_global(ui.layer_id())
                            .unwrap_or_default();
                        ui.ctx().output_mut(|o| {
                            o.ime = Some(egui::output::IMEOutput {
                                rect: to_global * rect,
                                cursor_rect: to_global * primary_cursor_rect,
                            });
                        });
                    }
                }
            }
        }

        // 确保当文本输入区域获得或失去焦点时IME行为正确
        if state.ime_enabled && (response.gained_focus() || response.lost_focus()) {
            state.ime_enabled = false;
            if let Some(mut ccursor_range) = state.cursor.char_range() {
                ccursor_range.secondary.index = ccursor_range.primary.index;
                state.cursor.set_char_range(Some(ccursor_range));
            }
            ui.input_mut(|i| i.events.retain(|e| !matches!(e, Event::Ime(_))));
        }

        // 保存状态
        state.clone().store(ui.ctx(), id);

        // 如果文本更改，添加小部件信息
        if response.changed() {
            response.widget_info(|| {
                egui::WidgetInfo::text_edit(
                    ui.is_enabled(),
                    mask_if_password(password, prev_text.as_str()),
                    mask_if_password(password, text.as_str()),
                    "",
                )
            });
        } else if selection_changed {
            // 如果选择更改，输出选择更改事件
            let cursor_range = cursor_range.unwrap();
            let char_range = cursor_range.primary.index..=cursor_range.secondary.index;
            let info = egui::WidgetInfo::text_selection_changed(
                ui.is_enabled(),
                char_range,
                mask_if_password(password, text.as_str()),
            );
            response.output_event(OutputEvent::TextSelectionChanged(info));
        } else {
            // 否则，添加静态小部件信息
            response.widget_info(|| {
                egui::WidgetInfo::text_edit(
                    ui.is_enabled(),
                    mask_if_password(password, prev_text.as_str()),
                    mask_if_password(password, text.as_str()),
                    "",
                )
            });
        }

        TextEditOutput {
            response,
            galley,
            galley_pos,
            text_clip_rect: rect,
            state,
            cursor_range,
        }
    }
}

fn mask_if_password(is_password: bool, text: &str) -> String {
    fn mask_password(text: &str) -> String {
        std::iter::repeat(egui::epaint::text::PASSWORD_REPLACEMENT_CHAR)
            .take(text.chars().count())
            .collect::<String>()
    }

    if is_password {
        mask_password(text)
    } else {
        text.to_owned()
    }
}

// ----------------------------------------------------------------------------

#[expect(clippy::too_many_arguments)]
fn events(
    ui: &egui::Ui,
    state: &mut TextEditState,
    text: &mut dyn TextBuffer,
    galley: &mut Arc<Galley>,
    layouter: &mut dyn FnMut(&Ui, &dyn TextBuffer, f32) -> Arc<Galley>,
    id: Id,
    wrap_width: f32,
    multiline: bool,
    password: bool,
    default_cursor_range: CCursorRange,
    char_limit: usize,
    event_filter: EventFilter,
    return_key: Option<KeyboardShortcut>,
) -> (bool, CCursorRange) {
    let os = ui.ctx().os();

    let mut cursor_range = state.cursor.range(galley).unwrap_or(default_cursor_range);

    // We feed state to the undoer both before and after handling input
    // so that the undoer creates automatic saves even when there are no events for a while.
    state.undoer.lock().feed_state(
        ui.input(|i| i.time),
        &(cursor_range, text.as_str().to_owned()),
    );

    let copy_if_not_password = |ui: &Ui, text: String| {
        if !password {
            ui.ctx().copy_text(text);
        }
    };

    let mut any_change = false;

    let mut events = ui.input(|i| i.filtered_events(&event_filter));

    if state.ime_enabled {
        remove_ime_incompatible_events(&mut events);
        // Process IME events first:
        events.sort_by_key(|e| !matches!(e, Event::Ime(_)));
    }

    for event in &events {
        let did_mutate_text = match event {
            // First handle events that only changes the selection cursor, not the text:
            event if cursor_range.on_event(os, event, galley, id) => None,

            Event::Copy => {
                if !cursor_range.is_empty() {
                    copy_if_not_password(ui, cursor_range.slice_str(text.as_str()).to_owned());
                }
                None
            }
            Event::Cut => {
                if cursor_range.is_empty() {
                    None
                } else {
                    copy_if_not_password(ui, cursor_range.slice_str(text.as_str()).to_owned());
                    Some(CCursorRange::one(text.delete_selected(&cursor_range)))
                }
            }
            Event::Paste(text_to_insert) => {
                if !text_to_insert.is_empty() {
                    let mut ccursor = text.delete_selected(&cursor_range);

                    text.insert_text_at(&mut ccursor, text_to_insert, char_limit);

                    Some(CCursorRange::one(ccursor))
                } else {
                    None
                }
            }
            Event::Text(text_to_insert) => {
                // Newlines are handled by `Key::Enter`.
                if !text_to_insert.is_empty() && text_to_insert != "\n" && text_to_insert != "\r" {
                    let mut ccursor = text.delete_selected(&cursor_range);

                    text.insert_text_at(&mut ccursor, text_to_insert, char_limit);

                    Some(CCursorRange::one(ccursor))
                } else {
                    None
                }
            }
            Event::Key {
                key: Key::Tab,
                pressed: true,
                modifiers,
                ..
            } if multiline => {
                let mut ccursor = text.delete_selected(&cursor_range);
                if modifiers.shift {
                    // TODO(emilk): support removing indentation over a selection?
                    text.decrease_indentation(&mut ccursor);
                } else {
                    text.insert_text_at(&mut ccursor, "\t", char_limit);
                }
                Some(CCursorRange::one(ccursor))
            }
            Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if return_key.is_some_and(|return_key| {
                *key == return_key.logical_key && modifiers.matches_logically(return_key.modifiers)
            }) =>
            {
                if multiline {
                    let mut ccursor = text.delete_selected(&cursor_range);
                    text.insert_text_at(&mut ccursor, "\n", char_limit);
                    // TODO(emilk): if code editor, auto-indent by same leading tabs, + one if the lines end on an opening bracket
                    Some(CCursorRange::one(ccursor))
                } else {
                    ui.memory_mut(|mem| mem.surrender_focus(id)); // End input with enter
                    break;
                }
            }

            Event::Key {
                key,
                pressed: true,
                modifiers,
                ..
            } if (modifiers.matches_logically(Modifiers::COMMAND) && *key == Key::Y)
                || (modifiers.matches_logically(Modifiers::SHIFT | Modifiers::COMMAND)
                    && *key == Key::Z) =>
            {
                if let Some((redo_ccursor_range, redo_txt)) = state
                    .undoer
                    .lock()
                    .redo(&(cursor_range, text.as_str().to_owned()))
                {
                    text.replace_with(redo_txt);
                    Some(*redo_ccursor_range)
                } else {
                    None
                }
            }

            Event::Key {
                key: Key::Z,
                pressed: true,
                modifiers,
                ..
            } if modifiers.matches_logically(Modifiers::COMMAND) => {
                if let Some((undo_ccursor_range, undo_txt)) = state
                    .undoer
                    .lock()
                    .undo(&(cursor_range, text.as_str().to_owned()))
                {
                    text.replace_with(undo_txt);
                    Some(*undo_ccursor_range)
                } else {
                    None
                }
            }

            Event::Key {
                modifiers,
                key,
                pressed: true,
                ..
            } => check_for_mutating_key_press(os, &cursor_range, text, galley, modifiers, *key),

            Event::Ime(ime_event) => match ime_event {
                ImeEvent::Enabled => {
                    state.ime_enabled = true;
                    state.ime_cursor_range = cursor_range;
                    None
                }
                ImeEvent::Preedit(text_mark) => {
                    if text_mark == "\n" || text_mark == "\r" {
                        None
                    } else {
                        // Empty prediction can be produced when user press backspace
                        // or escape during IME, so we clear current text.
                        let mut ccursor = text.delete_selected(&cursor_range);
                        let start_cursor = ccursor;
                        if !text_mark.is_empty() {
                            text.insert_text_at(&mut ccursor, text_mark, char_limit);
                        }
                        state.ime_cursor_range = cursor_range;
                        Some(CCursorRange::two(start_cursor, ccursor))
                    }
                }
                ImeEvent::Commit(prediction) => {
                    if prediction == "\n" || prediction == "\r" {
                        None
                    } else {
                        state.ime_enabled = false;

                        if !prediction.is_empty()
                            && cursor_range.secondary.index
                                == state.ime_cursor_range.secondary.index
                        {
                            let mut ccursor = text.delete_selected(&cursor_range);
                            text.insert_text_at(&mut ccursor, prediction, char_limit);
                            Some(CCursorRange::one(ccursor))
                        } else {
                            let ccursor = cursor_range.primary;
                            Some(CCursorRange::one(ccursor))
                        }
                    }
                }
                ImeEvent::Disabled => {
                    state.ime_enabled = false;
                    None
                }
            },

            _ => None,
        };

        if let Some(new_ccursor_range) = did_mutate_text {
            any_change = true;

            // Layout again to avoid frame delay, and to keep `text` and `galley` in sync.
            *galley = layouter(ui, text, wrap_width);

            // Set cursor_range using new galley:
            cursor_range = new_ccursor_range;
        }
    }

    state.cursor.set_char_range(Some(cursor_range));

    state.undoer.lock().feed_state(
        ui.input(|i| i.time),
        &(cursor_range, text.as_str().to_owned()),
    );

    (any_change, cursor_range)
}

// ----------------------------------------------------------------------------

fn remove_ime_incompatible_events(events: &mut Vec<Event>) {
    // Remove key events which cause problems while 'IME' is being used.
    // See https://github.com/emilk/egui/pull/4509
    events.retain(|event| {
        !matches!(
            event,
            Event::Key { repeat: true, .. }
                | Event::Key {
                    key: Key::Backspace
                        | Key::ArrowUp
                        | Key::ArrowDown
                        | Key::ArrowLeft
                        | Key::ArrowRight,
                    ..
                }
        )
    });
}

// ----------------------------------------------------------------------------

/// Returns `Some(new_cursor)` if we did mutate `text`.
fn check_for_mutating_key_press(
    os: OperatingSystem,
    cursor_range: &CCursorRange,
    text: &mut dyn TextBuffer,
    galley: &Galley,
    modifiers: &Modifiers,
    key: Key,
) -> Option<CCursorRange> {
    match key {
        Key::Backspace => {
            let ccursor = if modifiers.mac_cmd {
                text.delete_paragraph_before_cursor(galley, cursor_range)
            } else if let Some(cursor) = cursor_range.single() {
                if modifiers.alt || modifiers.ctrl {
                    // alt on mac, ctrl on windows
                    text.delete_previous_word(cursor)
                } else {
                    text.delete_previous_char(cursor)
                }
            } else {
                text.delete_selected(cursor_range)
            };
            Some(CCursorRange::one(ccursor))
        }

        Key::Delete if !modifiers.shift || os != OperatingSystem::Windows => {
            let ccursor = if modifiers.mac_cmd {
                text.delete_paragraph_after_cursor(galley, cursor_range)
            } else if let Some(cursor) = cursor_range.single() {
                if modifiers.alt || modifiers.ctrl {
                    // alt on mac, ctrl on windows
                    text.delete_next_word(cursor)
                } else {
                    text.delete_next_char(cursor)
                }
            } else {
                text.delete_selected(cursor_range)
            };
            let ccursor = CCursor {
                prefer_next_row: true,
                ..ccursor
            };
            Some(CCursorRange::one(ccursor))
        }

        Key::H if modifiers.ctrl => {
            let ccursor = text.delete_previous_char(cursor_range.primary);
            Some(CCursorRange::one(ccursor))
        }

        Key::K if modifiers.ctrl => {
            let ccursor = text.delete_paragraph_after_cursor(galley, cursor_range);
            Some(CCursorRange::one(ccursor))
        }

        Key::U if modifiers.ctrl => {
            let ccursor = text.delete_paragraph_before_cursor(galley, cursor_range);
            Some(CCursorRange::one(ccursor))
        }

        Key::W if modifiers.ctrl => {
            let ccursor = if let Some(cursor) = cursor_range.single() {
                text.delete_previous_word(cursor)
            } else {
                text.delete_selected(cursor_range)
            };
            Some(CCursorRange::one(ccursor))
        }

        _ => None,
    }
}
