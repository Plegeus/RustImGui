
#include "../include/imgui_cpp_vk.h"
#include "../../ImGui_cpp/include/imgui_cpp.h"
#include "../../ImGui_cpp/include/vulkan_info.h"
#include <imgui.h>
#include <backends/imgui_impl_glfw.h>
#include <backends/imgui_impl_vulkan.h>
#include <stdio.h>
#include <cstdlib>


// from https://github.com/ocornut/imgui/blob/master/examples/example_glfw_vulkan/main.cpp#L259
void check_vk_result(VkResult err);
void check_vk_result(VkResult err) {
    if (err == VK_SUCCESS)
        return;
    fprintf(stderr, "[vulkan] Error: VkResult = %d\n", err);
    if (err < 0)
        abort();
}


void __init_glfw_vk(void* p_window, void* p_info) {

    VulkanInfo* vulkan_info = (VulkanInfo*) p_info;
    GLFWwindow* window = (GLFWwindow*) p_window;

    IMGUI_CHECKVERSION();
    ImGui::CreateContext();
    ImGuiIO& io = ImGui::GetIO();
    io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard; // Enable Keyboard Controls
    io.ConfigFlags |= ImGuiConfigFlags_DockingEnable;

    ImGui::StyleColorsDark();
    ImGui_ImplGlfw_InitForVulkan(window, true);
    
    ImGui_ImplVulkan_InitInfo info = { 0 };
    info.Instance = (VkInstance) vulkan_info->instance;
    info.PhysicalDevice = (VkPhysicalDevice) vulkan_info->physical_device;
    info.Device = (VkDevice) vulkan_info->device;
    info.QueueFamily = (uint32_t) vulkan_info->queue_familty;
    info.Queue = (VkQueue) vulkan_info->queue;
    info.DescriptorPool = VK_NULL_HANDLE;
    //info.RenderPass = (VkRenderPass) vulkan_info->render_pass;
    info.RenderPass = VK_NULL_HANDLE;
    info.MinImageCount = (uint32_t) vulkan_info->min_image_count;
    info.ImageCount = (uint32_t) vulkan_info->actual_image_count;
    info.MSAASamples = VK_SAMPLE_COUNT_1_BIT;
    info.PipelineCache = VK_NULL_HANDLE;
    info.DescriptorPoolSize = 1024;
    info.UseDynamicRendering = false;
    info.Allocator = nullptr;
    info.CheckVkResultFn = check_vk_result;
    ImGui_ImplVulkan_Init(&info);
    

}
void __terminate_vk(void) {
    ImGui_ImplVulkan_Shutdown();
    ImGui_ImplGlfw_Shutdown();
    ImGui::DestroyContext();
}

void __new_frame_vk(void) {    
    ImGui_ImplVulkan_NewFrame();
    ImGui_ImplGlfw_NewFrame();
    ImGui::NewFrame();
}
void __end_frame_vk(void) {
    ImGui::EndFrame();
}
void __render_vk(void* p_command_buffer) {
    ImGui::Render();
    ImGui_ImplVulkan_RenderDrawData(
        ImGui::GetDrawData(), 
        (VkCommandBuffer) p_command_buffer
    );
}



