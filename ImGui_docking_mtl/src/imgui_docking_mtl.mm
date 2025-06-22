//
//  imgui_docking_mtl.m
//  ImGui_docking_mtl
//
//  Created by Timoty Gielkens on 10/07/2024.
//

#include "../include/imgui_docking_mtl.h"
#include <imgui.h>
#include <imgui_impl_glfw.h>
#include <imgui_impl_metal.h>
#include <stdio.h>


void __init_glfw_mtl(void* p_window, void* p_device) {
    
    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls
    //io.ConfigFlags |= ImGuiConfigFlags_ViewportsEnable;
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;

    // Setup Dear ImGui style
    ImGui::StyleColorsDark();
    //ImGui::StyleColorsLight();

    // Setup Platform/renderer backends
    ImGui_ImplGlfw_InitForOther((GLFWwindow*) p_window, true);
    ImGui_ImplMetal_Init((__bridge id <MTLDevice>) p_device);

}
void __terminate_mtl(void) {
    ImGui_ImplMetal_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
}


void __new_frame_mtl(void const* descriptor) {
    ImGuiIO& io = ImGui::GetIO();
    ImGui_ImplMetal_NewFrame((__bridge MTLRenderPassDescriptor*) descriptor);
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
    io.DisplayFramebufferScale = ImVec2(1, 1);
}
void __end_frame_mtl(void) {
    ImGui::EndFrame();
}
void __render_mtl(void const* command_buffer, void const* command_encoder) {
    ImGui::Render();
    ImGui_ImplMetal_RenderDrawData(
            ImGui::GetDrawData(),
            (__bridge id <MTLCommandBuffer>) command_buffer,
            (__bridge id <MTLRenderCommandEncoder>) command_encoder
    );
}






