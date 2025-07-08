//
// Created by Timoty Gielkens on 14/04/2024.
//

#include <imgui_cpp.h>
#include "../include/imgui_c.h"


void init_glfw(void* p_window, void* p_device) {
    __init_glfw(p_window, p_device);
}
void terminate() {
    __terminate();
}

void _show_demo_window(int* open) {
    __show_demo_window(open);
}
void _show_style_editor() {
    __show_style_editor();
}

void _get_mouse_pos(float* x, float* y) {
    __get_mouse_pos(x, y);
}
int _is_window_hovered(int flags) {
    return __is_window_hovered(flags);
}
int _is_window_focused(int flags) {
    return __is_window_focused(flags);
}
void _get_cursor_screen_pos(float* x, float* y) {
    __get_cursor_screen_pos(x, y);
}
void _set_cursor_screen_pos(float x, float y) {
    __set_cursor_screen_pos(x, y);
}
void _get_cursor_pos(float* x, float* y) {
    __get_cursor_pos(x, y);
}
void _set_cursor_pos(float x, float y) {
    __set_cursor_pos(x, y);
}

void new_frame(void const* descriptor) {
    __new_frame(descriptor);
}
void end_frame() {
    __end_frame();
}
void render(void const* command_buffer, void const* command_encoder) {
    __render(command_buffer, command_encoder);
}

void* create_context() {
    return __create_context();
}
void set_current_context(void* p_context) {
    __set_current_context(p_context);
}
void* get_current_context() {
    return __get_current_context();
}

void _set_next_window_size(float w, float h, int cond) {
    __set_next_window_size(w, h, cond);
}
void _get_window_size(float* w, float* h) {
    __get_window_size(w, h);
}
void _get_content_region_avail(float* w, float* h) {
    __get_content_region_avail(w, h);
}
void _set_next_window_pos(float x, float y, int cond, float pivot_x, float pivot_y) {
    __set_next_window_pos(x, y, cond, pivot_x, pivot_y);
}
void _get_window_pos(float* x, float* y) {
    __get_window_pos(x, y);
}
void set_next_window_focus() {
    __set_next_window_focus();
}
void set_window_focus() {
    __set_window_focus();
}
void set_next_item_width(float width) {
    __set_next_item_width(width);
}
void align_text_to_frame_padding() {
    __align_text_to_frame_padding();
}

void push_item_width(float width) {
    __push_item_width(width);
}
void pop_item_width() {
    __pop_item_width();
}

int _begin(const char const* title, int* open, int flags) {
    return __begin(title, open, flags);
}
void end() {
    __end();
}
int _begin_child(const char const* id, float x, float y, int child_flags, int window_flags) {
    return __begin_child(id, x, y, child_flags, window_flags);
}
void end_child() {
    __end_child();
}
void _open_popup(const char const* id, int flags) {
    __open_popup(id, flags);
}
int _begin_popup(const char const* id, int flags) {
    return __begin_popup(id, flags);
}
int _begin_popup_modal(const char const* name, int* open, int flags) {
    return __begin_popup_modal(name, open, flags);
}
void end_popup() {
    __end_popup();
}
int _collapsing_header(const char const* label) {
    return __collapsing_header(label);
}
int _tree_node(const char const* label) {
    return __tree_node(label);
}
void tree_pop() {
    __tree_pop();
}
int _selectable(const char const* label, int selected, int flags, float w, float h) {
    return __selectable(label, selected, flags, w, h);
}

int _begin_tab_bar(const char const* label, int flags) {
    return __begin_tab_bar(label, flags);
}
void end_tab_bar() {
    __end_tab_bar();
}
int _begin_tab_item(const char const* label, int* open, int flags) {
    return __begin_tab_item(label, open, flags);
}
void end_tab_item() {
    __end_tab_item();
}
int _tab_item_button(const char const* label, int flags) {
    return __tab_item_button(label, flags);
}

void _columns(int count, const char const* id, int border) {
    __columns(count, id, border);
}
void next_column() {
    __next_column();
}
void _table_setup_column(const char const* label, int flags, float width_or_weight, unsigned int user_id) {
    __table_setup_column(label, flags, width_or_weight, user_id);
}
void table_headers_row() {
    __table_headers_row();
}
int _begin_table(const char const* id, int columns, int flags) {
    return __begin_table(id, columns, flags);
}
void _table_next_row(int flags, float min_row_height) {
    __table_next_row(flags, min_row_height);
}
int _table_next_column() {
    return __table_next_column();
}
int _table_set_column_index(int column) {
    return __table_set_column_index(column);
}
void end_table() {
    __end_table();
}

void _text(const char const* text) {
    __text(text);
}
void _text_wrapped(const char const* text) {
    __text_wrapped(text);
}
int _input_text(const char const* label, char* buf, int buf_size, int flags) {
    return __input_text(label, buf, buf_size, flags);
}
int _input_text_multi_line(const char const* label, char* buf, int buf_size, float size_x, float size_y, int flags, void* callback, void* user_data) {
    return __input_text_multi_line(label, buf, buf_size, size_x, size_y, flags, callback, user_data);
}
void _text_colored(float r, float g, float b, float a, const char const* text) {
    __text_colored(r, g, b, a, text);
}

void _image(void const* texture_id, float const* img_size, float const* uv0, float const* uv1, float const* tint_col, float const* border_col) {
    __image(texture_id, img_size, uv0, uv1, tint_col, border_col);
}

int _begin_main_menu_bar() {
    return __begin_main_menu_bar();
}
void end_main_menu_bar() {
    __end_main_menu_bar();
}

int _begin_menu_bar() {
    return __begin_menu_bar();
}
void end_menu_bar() {
    __end_menu_bar();
}

int _begin_menu(const char const* label, int enabled) {
    return __begin_menu(label, enabled);
}
void end_menu() {
    __end_menu();
}

int _menu_item(const char const* label, const char const* shortcut, int* selected, int enabled) {
    return __menu_item(label, shortcut, selected, enabled);
}

void separator() {
    __separator();
}
void _separator_text(const char const* text) {
    __separator_text(text);
}
void _same_line(float offset, float spacing) {
    __same_line(offset, spacing);
}

int _button(const char const* label, float w, float h) {
    return __button(label, w, h);
}
int _radio_button(const char const* label, int* value_or_active, int v_button) {
    return __radio_button(label, value_or_active, v_button);
}
int _small_button(const char const* label) {
    return __small_button(label);
}

int _list_box(const char const* label, int* current, const char* const* items, int count) {
    return __list_box(label, current, items, count);
}
int _combo(const char const* label, int* current, const char* const* items, int count, int max_height) {
    return __combo(label, current, items, count, max_height);
}

void push_id(int id) {
    __push_id(id);
}
void pop_id() {
    __pop_id();
}

void _push_style_var(int idx, float x, float y) {
    __push_style_var(idx, x, y);
}
void _pop_style_var(int count) {
    __pop_style_var(count);
}
void _get_style_var(int idx, float* x, float* y) {
    __get_style_var(idx, x, y);
}

void _push_style_color(int idx, float* color) {
    __push_style_color(idx, color);
}
void pop_style_color(int count) {
    __pop_style_color(count);
}

int _is_item_clicked(int mouse_button) {
    return __is_item_clicked(mouse_button);
}
int _is_item_hovered(int flags) {
    return __is_item_hovered(flags);
}


int _input_float_2(const char const* label, float* value, const char const* format, int flags) {
    return __input_float_2(label, value, format, flags);
}
int _input_float_3(const char const* label, float* value, const char const* format, int flags) {
    return __input_float_3(label, value, format, flags);
}
int _input_float_4(const char const* label, float* value, const char const* format, int flags) {
    return __input_float_4(label, value, format, flags);
}
int _slider_float_2(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return __slider_float_2(label, value, min, max, format, flags);
}
int _slider_float_3(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return __slider_float_3(label, value, min, max, format, flags);
}
int _slider_float_4(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return __slider_float_4(label, value, min, max, format, flags);
}
int _drag_float_2(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return __drag_float_2(label, value, speed, min, max, format, flags);
}
int _drag_float_3(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return __drag_float_3(label, value, speed, min, max, format, flags);
}
int _drag_float_4(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return __drag_float_4(label, value, speed, min, max, format, flags);
}

int _input_float(const char const* label, float* value, float step, float step_fast, const char const* format, int flags) {
    return __input_float(label, value, step, step_fast, format, flags);
}
int _drag_float(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return __drag_float(label, value, speed, min, max, format, flags);
}
int _slider_float(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return __slider_float(label, value, min, max, format, flags);
}
int _input_double(const char const* label, double* value, double step, double step_fast, const char const* format, int flags) {
    return __input_double(label, value, step, step_fast, format, flags);
}
int _input_int(const char const* label, int* value, int step, int step_fast, int flags) {
    return __input_int(label ,value, step, step_fast, flags);
}
int _checkbox(const char const* label, int* boo) {
    return __checkbox(label, boo);
}

int _color_edit_3(const char const* label, float* col, int flags) {
    return __color_edit_3(label, col, flags);
}
int _color_edit_4(const char const* label, float* col, int flags) {
    return __color_edit_4(label, col, flags);
}
int _color_picker_3(const char const* label, float* col, int flags) {
    return __color_picker_3(label, col, flags);
}
int _color_picker_4(const char const* label, float* col, int flags, const float* ref_col) {
    return __color_picker_4(label, col, flags, ref_col);
}


float frame_rate() {
    return __frame_rate();
}


