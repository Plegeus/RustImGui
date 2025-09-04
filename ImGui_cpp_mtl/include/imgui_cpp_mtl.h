//
//  imgui_docking_mtl.h
//  ImGui_docking_mtl
//
//  Created by Timoty Gielkens on 10/07/2024.
//

#ifndef imgui_docking_mtl_h
#define imgui_docking_mtl_h

void __init_glfw_mtl(void);
void __init_cocoa_mtl(void* p_window, void* p_device);
void __terminate_mtl(void);

void __new_frame_mtl(void const* descriptor, void* pView);
void __end_frame_mtl(void);
void __render_mtl(void const* command_buffer, void const* command_encoder);


#endif /* imgui_docking_mtl_h */
