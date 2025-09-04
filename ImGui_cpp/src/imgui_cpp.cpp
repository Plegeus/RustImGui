//
// Created by Timoty Gielkens on 08/07/2024.
//

#include <imgui_cpp.h>
#include <imgui.h>
#ifdef _WIN32
#include <imgui_cpp_vk.h>
#include <vulkan/vulkan.h>
#endif
#ifdef __APPLE__ 
#include <imgui_cpp_mtl.h>
#endif
#include <stdio.h>


void __init_cocoa(void* pWindow, void* pDevice) {
    __init_cocoa_mtl(pWindow, pDevice);
}
void __init_glfw(void* p_window, void* p_data) {
#ifdef __APPLE__
    //__init_glfw_mtl();
#endif
#ifdef _WIN32
    __init_glfw_vk(p_window, p_data);
#endif
}
void __terminate() {
#ifdef __APPLE__
    __terminate_mtl();
#endif
#ifdef _WIN32
    __terminate_vk();
#endif
}

void __new_frame(void* descriptor, void* pView) {
#ifdef __APPLE__
    __new_frame_mtl(descriptor, pView);
#endif
#ifdef _WIN32
    __new_frame_vk();
#endif
}
void __end_frame() {
#ifdef __APPLE__
    __end_frame_mtl();
#endif
#ifdef _WIN32
    __end_frame_vk();
#endif
}
void __render(void* pCommandBuffer, void* pCommandEncoder) {
#ifdef __APPLE__
    __render_mtl(pCommandBuffer, pCommandEncoder);
#endif
#ifdef _WIN32
    __render_vk(p_command_buffer);
#endif
}


void* __create_context() {
    IMGUI_CHECKVERSION();
    return ImGui::CreateContext();
}
void __set_current_context(void* p_context) {
    ImGui::SetCurrentContext((ImGuiContext*) p_context);
}
void* __get_current_context() {
    return (void*) ImGui::GetCurrentContext();
}

void get_allocator_functions(void* allocf, void* free, void* user_data) {
    //ImGui::GetAllocatorFunctions(allocf, free, user_data);
}
void set_allocator_functions(void* alloc, void* free, void* user_data) {

}

void __show_demo_window(int* open) {
    if (open == NULL) {
        ImGui::ShowDemoWindow();
    }
    else {
        bool b = *open == 1;
        ImGui::ShowDemoWindow(&b);
        *open = b ? 1 : 0;
    }
}
void __show_style_editor() {
    ImGui::ShowStyleEditor();
}

void __get_mouse_pos(float* x, float* y) {
    auto pos = ImGui::GetMousePos();
    *x = pos.x;
    *y = pos.y;
}
int __is_window_hovered(int flags) {
    return ImGui::IsWindowHovered(flags) ? 1 : 0;
}
int __is_window_focused(int flags) {
    return ImGui::IsWindowFocused(flags) ? 1 : 0;
}
void __get_cursor_screen_pos(float* x, float* y) {
    auto pos = ImGui::GetCursorScreenPos();
    *x = pos.x;
    *y = pos.y;
}
void __set_cursor_screen_pos(float x, float y) {
    ImVec2 pos(x, y);
    ImGui::SetCursorScreenPos(pos);
}
void __get_cursor_pos(float* x, float* y) {
    auto pos = ImGui::GetCursorPos();
    *x = pos.x;
    *y = pos.y;
}
void __set_cursor_pos(float x, float y) {
    ImVec2 pos(x, y);
    ImGui::SetCursorPos(pos);
}


void __set_next_window_size(float w, float h, int cond) {
    ImVec2 size(w, h);
    ImGui::SetNextWindowSize(size, cond);
}
void __get_window_size(float* w, float* h) {
    auto size = ImGui::GetWindowSize();
    *w = size.x;
    *h = size.y;
}
void __get_content_region_avail(float* w, float* h) {
    auto avail = ImGui::GetContentRegionAvail();
    *w = avail.x;
    *h = avail.y;
}
void __set_next_window_pos(float x, float y, int cond, float pivot_x, float pivot_y) {
    ImVec2 pos(x, y);
    ImVec2 pivot(pivot_x, pivot_y);
    ImGui::SetNextWindowPos(pos, cond, pivot);
}
void __get_window_pos(float* x, float* y) {
    auto pos = ImGui::GetWindowPos();
    *x = pos.x;
    *y = pos.y;
}
void __set_next_window_focus() {
    ImGui::SetNextWindowFocus();
}
void __set_window_focus() {
    ImGui::SetWindowFocus();
}
void __set_next_item_width(float width) {
    ImGui::SetNextItemWidth(width);
}
void __align_text_to_frame_padding() {
    ImGui::AlignTextToFramePadding();
}

void __push_item_width(float width) {
    ImGui::PushItemWidth(width);
}
void __pop_item_width() {
    ImGui::PopItemWidth();
}

int __begin(const char const* title, int* open, int flags) {
    if (open == NULL) {
        return ImGui::Begin(title, nullptr, flags) ? 1 : 0;
    }
    else {
        bool b = *open == 1;
        bool b2 = ImGui::Begin(title, &b, flags);
        *open = b ? 1 : 0;
        return b2 ? 1 : 0;
    }
}
void __end() {
    ImGui::End();
}
int __begin_child(const char const* id, float x, float y, int child_flags, int window_flags) {
    ImVec2 size(x, y);
    return ImGui::BeginChild(id, size, child_flags, window_flags) ? 1 : 0;
}
void __end_child() {
    ImGui::EndChild();
}
void __open_popup(const char const* id, int flags) {
    ImGui::OpenPopup(id, flags);
}
int __begin_popup(const char const* id, int flags) {
    return ImGui::BeginPopup(id, flags) ? 1 : 0;
}
int __begin_popup_modal(const char const* name, int* open, int flags) {
    if (open == NULL) {
        return ImGui::BeginPopupModal(name, NULL, flags) ? 1 : 0;
    }
    else {
        bool b = *open != 0;
        bool b2 = ImGui::BeginPopupModal(name, &b, flags);
        *open = b ? 1 : 0;
        return b2 ? 1 : 0;
    }
}
void __end_popup() {
    ImGui::EndPopup();
}
int __collapsing_header(const char const* label) {
    return ImGui::CollapsingHeader(label) ? 1 : 0;
}
int __tree_node(const char const* label) {
    return ImGui::TreeNode(label) ? 1 : 0;
}
void __tree_pop() {
    ImGui::TreePop();
}
int __selectable(const char const* label, int selected, int flags, float w, float h) {
    ImVec2 size(w, h);
    return ImGui::Selectable(label, selected != 0, flags, size) ? 1 : 0;
}
int __selectable_ptr(const char const* label, int* selected, int flags, float w, float h) {
    ImVec2 size(w, h);
    bool b = *selected != 0;
    int res = ImGui::Selectable(label, &b, flags, size) ? 1 : 0;
    *selected = b ? 1 : 0;
    return res;
}

int __begin_tab_bar(const char const* id, int flags) {
    return ImGui::BeginTabBar(id, flags) ? 1 : 0;
}
void __end_tab_bar() {
    ImGui::EndTabBar();
}
int __begin_tab_item(const char const* label, int* open, int flags) {
    if (open == nullptr) {
        return ImGui::BeginTabItem(label, nullptr, flags) ? 1 : 0;
    } else {
        bool b = *open != 0;
        int result = ImGui::BeginTabItem(label, &b, flags) ? 1 : 0;
        *open = b ? 1 : 0;
        return result;
    }
}
void __end_tab_item() {
    ImGui::EndTabItem();
}
int __tab_item_button(const char const* label, int flags) {
    return ImGui::TabItemButton(label, flags) ? 1 : 0;
}

void __columns(int count, const char const* id, int border) {
    ImGui::Columns(count, id, border != 0);
}
void __next_column() {
    ImGui::NextColumn();
}
void __table_setup_column(const char const* label, int flags, float width_or_weight, unsigned int user_id) {
    ImGui::TableSetupColumn(label, flags, width_or_weight, user_id);
}
void __table_headers_row() {
    ImGui::TableHeadersRow();
}
int __begin_table(const char const* id, int columns, int flags) {
    return ImGui::BeginTable(id, columns, flags) ? 1 : 0;
}
void __table_next_row(int flags, float min_row_height) {
    ImGui::TableNextRow(flags, min_row_height);
}
int __table_next_column() {
    return ImGui::TableNextColumn() ? 1 : 0;
}
int __table_set_column_index(int column) {
    return ImGui::TableSetColumnIndex(column) ? 1 : 0;
}
void __end_table() {
    ImGui::EndTable();
}

void __text(const char const* text) {
    ImGui::Text("%s", text);
}
void __text_wrapped(const char const* text) {
    ImGui::TextWrapped(text);
}
int __input_text(const char const* label, char* buf, int buf_size, int flags) {
    return ImGui::InputText(label, buf, buf_size, flags) ? 1 : 0;
}
int __input_text_multi_line(const char const* label, char* buf, int buf_size, float size_x, float size_y, int flags, void* callback, void* user_data) {
    ImVec2 size(size_x, size_y);
    return ImGui::InputTextMultiline(
        label,
        buf,
        buf_size,
        size,
        flags
    ) ? 1 : 0;
}
void __text_colored(float r, float g, float b, float a, const char const* text) {
    ImVec4 color(r, g, b, a);
    ImGui::TextColored(color, "%s", text);
}

void __image(void const* texture_id, float const* img_size, float const* uv0, float const* uv1, float const* tint_col, float const* border_col) {
    ImVec2 img_size2(img_size[0], img_size[1]);
    ImVec2 uv02(uv0[0], uv0[1]);
    ImVec2 uv12(uv1[0], uv1[1]);
    ImVec4 tint_col2(tint_col[0], tint_col[1], tint_col[2], tint_col[3]);
    ImVec4 border_col2(border_col[0], border_col[1], border_col[2], border_col[3]);
    ImGui::Image((ImTextureID) texture_id, img_size2, uv02, uv12, tint_col2, border_col2);
}

int __begin_main_menu_bar() {
    return ImGui::BeginMainMenuBar() ? 1 : 0;
}
void __end_main_menu_bar() {
    ImGui::EndMainMenuBar();
}

int __begin_menu_bar() {
    return ImGui::BeginMenuBar();
}
void __end_menu_bar() {
    ImGui::EndMenuBar();
}

int __begin_menu(const char const* label, int enabled) {
    return ImGui::BeginMenu(label, enabled != 0) ? 1 : 0;
}
void __end_menu() {
    ImGui::EndMenu();
}

int __menu_item(const char const* label, const char const* shortcut, int* selected, int enabled) {
    bool result;
    if (selected == nullptr) {
        result = ImGui::MenuItem(label, shortcut, false, enabled != 0);
    } else {
        bool b_selected = *selected != 0;
        result = ImGui::MenuItem(label, shortcut, &b_selected, enabled != 0);
        *selected = b_selected ? 1 : 0;
    }
    return result ? 1 : 0;
}

void __separator() {
    ImGui::Separator();
}
void __separator_text(const char const* text) {
    ImGui::SeparatorText(text);
}
void __same_line(float offset, float spacing) {
    ImGui::SameLine(offset, spacing);
}

int __button(const char const* label, float w, float h) {
    ImVec2 size(w, h);
    return ImGui::Button(label, size);
}
int __radio_button(const char const* label, int* value_or_active, int v_button) {
    if (v_button == -1)
        return ImGui::RadioButton(label, *value_or_active != 0) ? 1 : 0;
    else
        return ImGui::RadioButton(label, value_or_active, v_button);
}
int __small_button(const char const* label) {
    return ImGui::SmallButton(label) ? 1 : 0;
}

int __list_box(const char const* label, int* current, const char* const* items, int count) {
    return ImGui::ListBox(label, current, items, count) == 1;
}
int __combo(const char const* label, int* current, const char* const* items, int count, int max_height) {
    return ImGui::Combo(label, current, items, count, max_height) ? 1 : 0;
}

void __push_id(int id) {
    ImGui::PushID(id);
}
void __pop_id() {
    ImGui::PopID();
}

void __push_style_var(int idx, float x, float y) {
    switch (idx) {
        case ImGuiStyleVar_WindowPadding:
        case ImGuiStyleVar_WindowMinSize:
        case ImGuiStyleVar_WindowTitleAlign:
        case ImGuiStyleVar_FramePadding:
        case ImGuiStyleVar_ItemSpacing:
        case ImGuiStyleVar_ItemInnerSpacing:
        case ImGuiStyleVar_CellPadding:
        case ImGuiStyleVar_ButtonTextAlign:
        case ImGuiStyleVar_SelectableTextAlign:
        case ImGuiStyleVar_SeparatorTextAlign:
        case ImGuiStyleVar_SeparatorTextPadding:
            ImGui::PushStyleVar(idx, ImVec2(x, y));
            break;
        default:
            ImGui::PushStyleVar(idx, x);
            break;
    }
}
void __pop_style_var(int count) {
    ImGui::PopStyleVar(count);
}
void __get_style_var(int idx, float* x, float* y) {
    auto style= ImGui::GetStyle();
    switch (idx) {
        case ImGuiStyleVar_ItemSpacing:
            *x = style.ItemSpacing.x;
            *y = style.ItemSpacing.y;
            break;
        case ImGuiStyleVar_WindowPadding:
            *x = style.WindowPadding.x;
            *y = style.WindowPadding.y;
            break;
        default:
            printf("__get_style_var %d not implemented in ImGui_docking", idx);
            break;
    }
}

void __push_style_color(int idx, float* cols) {
    ImVec4 color(cols[0], cols[1], cols[2], cols[3]);
    ImGui::PushStyleColor(idx, color);
}
void __pop_style_color(int count) {
    ImGui::PopStyleColor(count);
}

int __is_item_clicked(int mouse_button) {
    return ImGui::IsItemClicked(mouse_button) ? 1 : 0;
}
int __is_item_hovered(int flags) {
    return ImGui::IsItemHovered(flags) ? 1 : 0;
}


int __input_float_2(const char const* label, float* value, const char const* format, int flags) {
    return ImGui::InputFloat2(label, value, format, flags) ? 1 : 0;
}
int __input_float_3(const char const* label, float* value, const char const* format, int flags) {
    return ImGui::InputFloat3(label, value, format, flags) ? 1 : 0;
}
int __input_float_4(const char const* label, float* value, const char const* format, int flags) {
    return ImGui::InputFloat4(label, value, format, flags) ? 1 : 0;
}
int __slider_float_2(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return ImGui::SliderFloat2(label, value, min, max, format, flags) ? 1 : 0;
}
int __slider_float_3(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return ImGui::SliderFloat3(label, value, min, max, format, flags) ? 1 : 0;
}
int __slider_float_4(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return ImGui::SliderFloat4(label, value, min, max, format, flags) ? 1 : 0;
}
int __drag_float_2(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return ImGui::DragFloat2(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_float_3(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return ImGui::DragFloat3(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_float_4(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return ImGui::DragFloat4(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_int_2(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags) {
    return ImGui::DragInt2(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_int_3(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags) {
    return ImGui::DragInt3(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_int_4(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags) {
    return ImGui::DragInt4(label, value, speed, min, max, format, flags) ? 1 : 0;
}

int __input_float(const char const* label, float* value, float step, float step_fast, const char const* format, int flags) {
    return ImGui::InputFloat(label, value, step, step_fast, format, flags) ? 1 : 0;
}
int __drag_float(const char const* label, float* value, float speed, float min, float max, const char const* format, int flags) {
    return ImGui::DragFloat(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __drag_int(const char const* label, int* value, float speed, int min, int max, const char const* format, int flags) {
    return ImGui::DragInt(label, value, speed, min, max, format, flags) ? 1 : 0;
}
int __slider_float(const char const* label, float* value, float min, float max, const char const* format, int flags) {
    return ImGui::SliderFloat(label, value, min, max, format, flags) ? 1 : 0;
}
int __input_double(const char const* label, double* value, double step, double step_fast, const char const* format, int flags) {
    return ImGui::InputDouble(label, value, step, step_fast, format, flags) ? 1 : 0;
}
int __input_int(const char const* label, int* value, int step, int step_fast, int flags) {
    return ImGui::InputInt(label, value, step, step_fast, flags) ? 1 : 0;
}
int __checkbox(const char const* label, int* boo) {
    bool b = *boo != 0;
    int result = ImGui::Checkbox(label, &b) ? 1 : 0;
    *boo = b ? 1 : 0;
    return result;
}

int __color_edit_3(const char const* label, float* col, int flags) {
    return ImGui::ColorEdit3(label, col, flags) ? 1 : 0;
}
int __color_edit_4(const char const* label, float* col, int flags) {
    return ImGui::ColorEdit4(label, col, flags) ? 1 : 0;
}
int __color_picker_3(const char const* label, float* col, int flags) {
    return ImGui::ColorPicker3(label, col, flags) ? 1 : 0;
}
int __color_picker_4(const char const* label, float* col, int flags, const float* ref_col) {
    return ImGui::ColorPicker4(label, col, flags, ref_col) ? 1 : 0;
}


float __frame_rate() {
    return ImGui::GetIO().Framerate;
}
