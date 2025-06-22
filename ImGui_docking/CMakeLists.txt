cmake_minimum_required(VERSION 3.24.3)
project(ImGui_docking)

set(CMAKE_CXX_STANDARD 23)
set(LIBRARY_OUTPUT_PATH ${PROJECT_SOURCE_DIR}/target)

set(IMGUI_DIR ${PROJECT_SOURCE_DIR}/deps/imgui-docking)
set(GLFW_DIR /Users/timo/Documents/coding/glfw-3.3.9.bin.MACOS)


add_library(ImGui_docking STATIC
  
  src/imgui_docking.cpp

  ${IMGUI_DIR}/imgui.cpp
  ${IMGUI_DIR}/imgui_draw.cpp
  ${IMGUI_DIR}/imgui_tables.cpp
  ${IMGUI_DIR}/imgui_widgets.cpp
  ${IMGUI_DIR}/imgui_demo.cpp
  ${IMGUI_DIR}/misc/cpp/imgui_stdlib.cpp
  ${IMGUI_DIR}/backends/imgui_impl_glfw.cpp
  ${IMGUI_DIR}/backends/imgui_impl_metal.mm

)


target_include_directories(ImGui_docking PRIVATE ${PROJECT_SOURCE_DIR}/include)
target_include_directories(ImGui_docking PRIVATE ${IMGUI_DIR})
target_include_directories(ImGui_docking PRIVATE ${IMGUI_DIR}/misc/cpp)

target_link_directories(ImGui_docking PRIVATE ${GLFW_DIR}/lib-arm64)
target_link_libraries(ImGui_docking PRIVATE glfw3)
target_include_directories(ImGui_docking PRIVATE ${GLFW_DIR}/include/)

target_include_directories(ImGui_docking PRIVATE /Users/timo/Documents/coding/black-death-studios/the-cosmos-game-engine/deps/imgui_bindings/ImGui_docking_mtl/include)


