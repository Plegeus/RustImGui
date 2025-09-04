
cmake_minimum_required(VERSION 3.24.3)
project(ImGui_cpp_vk)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)


add_library(ImGui_cpp_vk STATIC ${PROJECT_SOURCE_DIR}/src/imgui_cpp_vk.cpp)

target_include_directories(ImGui_cpp_vk PRIVATE ${PROJECT_SOURCE_DIR}/../deps/imgui-docking)
find_package(Vulkan REQUIRED)
target_link_libraries(ImGui_cpp_vk PRIVATE Vulkan::Vulkan)



