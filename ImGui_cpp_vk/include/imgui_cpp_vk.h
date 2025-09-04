
#ifndef imgui_docking_vk_h
#define imgui_docking_vk_h

void __init_glfw_vk(void* p_window, void* p_info);
void __terminate_vk(void);

void __new_frame_vk(void);
void __end_frame_vk(void);
void __render_vk(void* p_command_buffer);

#endif

