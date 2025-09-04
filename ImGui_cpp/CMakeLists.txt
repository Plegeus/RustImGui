cmake_minimum_required(VERSION 3.24.3)
project(ImGui_cpp)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)

set(DEPS_DIR ${PROJECT_SOURCE_DIR}/../deps)
set(IMGUI_DIR ${DEPS_DIR}/imgui-docking)
if(APPLE)
  set(GLFW_DIR ${DEPS_DIR}/glfw-3.4.bin.MACOS)
  set(BACKEND_DIR ${PROJECT_SOURCE_DIR}/../ImGui_cpp_mtl)
  set(IMGUI_IMPL_FILE ${IMGUI_DIR}/backends/imgui_impl_metal.mm)
else()
  set(GLFW_DIR ${DEPS_DIR}/glfw-3.4.bin.WIN64)
  set(BACKEND_DIR ${PROJECT_SOURCE_DIR}/../ImGui_cpp_vk)
  set(IMGUI_IMPL_FILE ${IMGUI_DIR}/backends/imgui_impl_vulkan.cpp)
endif()


add_library(
  ImGui_cpp 
  STATIC
  src/imgui_cpp.cpp
  ${IMGUI_DIR}/imgui.cpp
  ${IMGUI_DIR}/imgui_draw.cpp
  ${IMGUI_DIR}/imgui_tables.cpp
  ${IMGUI_DIR}/imgui_widgets.cpp
  ${IMGUI_DIR}/imgui_demo.cpp
  ${IMGUI_DIR}/misc/cpp/imgui_stdlib.cpp
  ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
  ${IMGUI_DIR}/backends/imgui_impl_osx.mm
  ${IMGUI_IMPL_FILE}
)


target_include_directories(ImGui_cpp PRIVATE ${PROJECT_SOURCE_DIR}/include)
target_include_directories(ImGui_cpp PRIVATE ${IMGUI_DIR})
target_include_directories(ImGui_cpp PRIVATE ${IMGUI_DIR}/misc/cpp)

#target_link_directories(ImGui_cpp PRIVATE ${GLFW_DIR}/lib-arm64)
#target_link_libraries(ImGui_cpp PRIVATE glfw3)
target_include_directories(ImGui_cpp PRIVATE ${GLFW_DIR}/include/)

target_include_directories(ImGui_cpp PRIVATE ${BACKEND_DIR}/include)  

if(WIN32) 
find_package(Vulkan REQUIRED)
target_link_libraries(ImGui_cpp PRIVATE Vulkan::Vulkan)
endif()


