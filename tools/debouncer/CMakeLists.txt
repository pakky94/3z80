cmake_minimum_required(VERSION 3.13)

include(pico_sdk_import.cmake)

project(debouncer C CXX ASM)
set(CMAKE_C_STANDARD 11)
set(CMAKE_CXX_STANDARD 17)
pico_sdk_init()

add_executable(debouncer
    main.c
)

pico_enable_stdio_usb(debouncer 1)
pico_enable_stdio_uart(debouncer 1)

pico_add_extra_outputs(debouncer)

target_link_libraries(debouncer pico_stdlib)
