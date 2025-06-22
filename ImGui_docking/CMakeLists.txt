cmake_minimum_required(VERSION 3.24.3)
project(ImGui_docking)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)

set(IMGUI_DIR ${PROJECT_SOURCE_DIR}/deps/imgui-docking)
if(APPLE)
  set(BACKEND_DIR ${PROJECT_SOURCE_DIR}/../ImGui_docking_mtl)
  set(IMGUI_IMPL_FILE ${IMGUI_DIR}/backends/imgui_impl_metal.mm)
else()
  set(GLFW_DIR E:/Coding/deps/glfw3)
  set(BACKEND_DIR ${PROJECT_SOURCE_DIR}/../ImGui_docking_vk)
  set(IMGUI_IMPL_FILE ${IMGUI_DIR}/backends/imgui_impl_vulkan.cpp)
endif()


add_library(ImGui_docking STATIC
  
  src/imgui_docking.cpp

  ${IMGUI_DIR}/imgui.cpp
  ${IMGUI_DIR}/imgui_draw.cpp
  ${IMGUI_DIR}/imgui_tables.cpp
  ${IMGUI_DIR}/imgui_widgets.cpp
  ${IMGUI_DIR}/imgui_demo.cpp
  ${IMGUI_DIR}/misc/cpp/imgui_stdlib.cpp
  ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
  ${IMGUI_IMPL_FILE}

)


target_include_directories(ImGui_docking PRIVATE ${PROJECT_SOURCE_DIR}/include)
target_include_directories(ImGui_docking PRIVATE ${IMGUI_DIR})
target_include_directories(ImGui_docking PRIVATE ${IMGUI_DIR}/misc/cpp)

target_link_directories(ImGui_docking PRIVATE ${GLFW_DIR}/lib-arm64)
target_link_libraries(ImGui_docking PRIVATE glfw3)
target_include_directories(ImGui_docking PRIVATE ${GLFW_DIR}/include/)

target_include_directories(ImGui_docking PRIVATE ${BACKEND_DIR}/include)  

if(WIN32) 
find_package(Vulkan REQUIRED)
target_link_libraries(ImGui_docking PRIVATE Vulkan::Vulkan)
endif()


