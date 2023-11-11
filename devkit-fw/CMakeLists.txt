cmake_minimum_required(VERSION 3.13)

include(pico_sdk_import.cmake)

project(devkit-fw C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()

add_executable(devkit-fw
    main.c io.c
)

pico_enable_stdio_usb(devkit-fw 1)
pico_enable_stdio_uart(devkit-fw 1)

pico_add_extra_outputs(devkit-fw)

target_link_libraries(devkit-fw pico_stdlib)
