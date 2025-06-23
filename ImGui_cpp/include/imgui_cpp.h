//
// Created by Timoty Gielkens on 08/07/2024.
//

#ifndef IMGUI_DOCKING_IMGUI_DOCKING_H
#define IMGUI_DOCKING_IMGUI_DOCKING_H

#ifdef __cplusplus
extern "C" {
#endif

// platform specific...
void __init_glfw(void*, void*);
void __terminate();

void __new_frame(void const*);
void __end_frame();
void __render(void const*, void const*);

// cross platform...
void* __create_context();
void* __get_current_context();
void __set_current_context(void* p_context);

void get_allocator_functions(void*, void*, void*);
void set_allocator_functions(void*, void*, void*);

void __show_demo_window(int*);
void __show_style_editor();

void __get_mouse_pos(float*, float*);
int __is_window_hovered(int);
int __is_window_focused(int);
void __get_cursor_screen_pos(float* x, float* y);
void __set_cursor_screen_pos(float x, float y);
void __get_cursor_pos(float* x, float* y);
void __set_cursor_pos(float x, float y);

void __set_next_window_size(float, float, int);
void __get_window_size(float*, float*);
void __get_content_region_avail(float*, float*);
void __set_next_window_pos(float, float, int, float, float);
void __get_window_pos(float*, float*);
void __set_next_window_focus();
void __set_window_focus();
void __set_next_item_width(float);
void __align_text_to_frame_padding();

void __push_item_width(float);
void __pop_item_width();

int __begin(const char const*, int*, int);
void __end();
int __begin_child(const char const*, float, float, int, int);
void __end_child();
void __open_popup(const char const*, int);
int __begin_popup(const char const*, int);
int __begin_popup_modal(const char const*, int*, int);
void __end_popup();
int __collapsing_header(const char const*);
int __tree_node(const char const*);
void __tree_pop();
int __selectable(const char const*, int, int, float, float);

int __begin_tab_bar(const char const*, int);
void __end_tab_bar();
int __begin_tab_item(const char const*, int*, int);
void __end_tab_item();
int __tab_item_button(const char const*, int);

void __columns(int, const char const*, int);
void __next_column();
void __table_setup_column(const char const*, int, float, unsigned int);
void __table_headers_row();
int __begin_table(const char const*, int, int);
void __table_next_row(int, float);
int __table_next_column();
int __table_set_column_index(int);
void __end_table();

void __text(const char const*);
void __text_wrapped(const char const*);
int __input_text(const char const*, char*, int, int);
int __input_text_multi_line(const char const*, char*, int, float, float, int, void*, void*);
void __text_colored(float r, float g, float b, float a, const char const* text);

void __image(void*, float*, float*, float*, float*, float*);

int __begin_main_menu_bar();
void __end_main_menu_bar();

int __begin_menu_bar();
void __end_menu_bar();

int __begin_menu(const char const*, int);
void __end_menu();

int __menu_item(const char const*, const char const*, int*, int);

void __separator();
void __separator_text(const char const*);
void __same_line(float, float);

int __button(const char const*, float, float);
int __radio_button(const char const*, int*, int);
int __small_button(const char const*);

int __list_box(const char const*, int*, const char* const*, int);
int __combo(const char const*, int*, const char* const*, int, int);

void __push_id(int);
void __pop_id();

void __push_style_var(int, float, float);
void __pop_style_var(int);
void __get_style_var(int, float*, float*);

void __push_style_color(int, float*);
void __pop_style_color(int);

int __is_item_clicked(int);
int __is_item_hovered(int);


int __input_float_2(const char const*, float*, const char const*, int);
int __input_float_3(const char const*, float*, const char const*, int);
int __input_float_4(const char const*, float*, const char const*, int);

int __input_float(const char const*, float*, float, float, const char const*, int);
int __input_double(const char const*, double*, double, double, const char const*, int);
int __input_int(const char const*, int*, int, int, int);
int __checkbox(const char const*, int*);

float __frame_rate();


#ifdef __cplusplus
}
#endif

#endif //IMGUI_DOCKING_IMGUI_DOCKING_H
