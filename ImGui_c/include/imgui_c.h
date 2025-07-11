//
// Created by Timoty Gielkens on 13/04/2024.
//

#ifndef IMGUI_BINDINGS_IMGUI_BINDINGS_H
#define IMGUI_BINDINGS_IMGUI_BINDINGS_H


void init_glfw(void*, void*);
void terminate();

void _show_demo_window(int*);
void _show_style_editor();

void _get_mouse_pos(float*, float*);
int _is_window_hovered(int);
int _is_window_focused(int);
void _get_cursor_screen_pos(float* x, float* y);
void _set_cursor_screen_pos(float x, float y);
void _get_cursor_pos(float* x, float* y);
void _set_cursor_pos(float x, float y);

void new_frame(void const*);
void end_frame();
void render(void const*, void const*);

void* create_context();
void set_current_context(void*);
void* get_current_context();

void _set_next_window_size(float, float, int);
void _get_window_size(float*, float*);
void _get_content_region_avail(float* w, float* h);
void _set_next_window_pos(float, float, int, float, float);
void _get_window_pos(float*, float*);
void set_next_window_focus();
void set_window_focus();
void set_next_item_width(float width);
void align_text_to_frame_padding();

void push_item_width(float);
void pop_item_width();

int _begin(const char const*, int*, int);
void end();
int _begin_child(const char const*, float, float, int, int);
void end_child();
void _open_popup(const char const*, int);
int _begin_popup(const char const*, int);
int _begin_popup_modal(const char const*, int*, int);
void end_popup();
int _collapsing_header(const char const*);
int _tree_node(const char const*);
void tree_pop();
int _selectable(const char const*, int, int, float, float);

int _begin_tab_bar(const char const*, int);
void end_tab_bar();
int _begin_tab_item(const char const*, int*, int);
void end_tab_item();
int _tab_item_button(const char const*, int);

void _columns(int, const char const*, int);
void next_column();
void _table_setup_column(const char const*, int, float, unsigned int);
void table_headers_row();
int _begin_table(const char const*, int, int);
void _table_next_row(int, float);
int _table_next_column();
int _table_set_column_index(int);
void end_table();

void _text(const char const*);
void _text_wrapped(const char const*);
int _input_text(const char const*, char*, int, int);
int _input_text_multi_line(const char const*, char*, int, float, float, int, void*, void*);
void _text_colored(float r, float g, float b, float a, const char const* text);

void _image(void const*, float const*, float const*, float const*, float const*, float const*);

int _begin_main_menu_bar();
void end_main_menu_bar();

int _begin_menu_bar();
void end_menu_bar();

int _begin_menu(const char const*, int);
void end_menu();

int _menu_item(const char const*, const char const*, int*, int);

void separator();
void _separator_text(const char const*);
void _same_line(float, float);

int _button(const char const*, float, float);
int _radio_button(const char const*, int*, int);
int _small_button(const char const*);

int _list_box(const char const*, int*, const char* const*, int);
int _combo(const char const*, int*, const char* const*, int, int);

void push_id(int);
void pop_id();

void _push_style_var(int, float, float);
void _pop_style_var(int);
void _get_style_var(int, float*, float*);

void _push_style_color(int, float*);
void pop_style_color(int);

int _is_item_clicked(int);
int _is_item_hovered(int);


int _input_float_2(const char const*, float*, const char const*, int);
int _input_float_3(const char const*, float*, const char const*, int);
int _input_float_4(const char const*, float*, const char const*, int);
int _slider_float_2(const char const* label, float* value, float min, float max, const char const* format, int flags);
int _slider_float_3(const char const* label, float* value, float min, float max, const char const* format, int flags);
int _slider_float_4(const char const* label, float* value, float min, float max, const char const* format, int flags);
int _drag_float_2(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags);
int _drag_float_3(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags);
int _drag_float_4(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags);
int _drag_int_2(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags);
int _drag_int_3(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags);
int _drag_int_4(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags);

int _input_float(const char const*, float*, float, float, const char const*, int);
int _drag_float(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags);
int _drag_int(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags);
int _slider_float(const char const* label, float* value, float min, float max, const char const* format, int flags);
int _input_double(const char const*, double*, double, double, const char const*, int);
int _input_int(const char const*, int*, int, int, int);
int _checkbox(const char const*, int*);

int _color_edit_3(const char const* label, float* col, int flags);
int _color_edit_4(const char const* label, float* col, int flags);
int _color_picker_3(const char const* label, float* col, int flags);
int _color_picker_4(const char const* label, float* col, int flags, const float* ref_col);


float frame_rate();


#endif //IMGUI_BINDINGS_IMGUI_BINDINGS_H
