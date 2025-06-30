
#ifndef VULKAN_INFO_H
#define VULKAN_INFO_H

#ifdef __cplusplus
extern "C" {
#endif


typedef struct vulkan_info {
    void* instance;
    void* physical_device;
    void* device;
    int queue_familty;
    void* queue;
    void* render_pass;
    int min_image_count;
    int actual_image_count;
} VulkanInfo;



#ifdef __cplusplus
}
#endif


#endif


