set(PROGRAM_NAME hello_pico)

add_executable(${PROGRAM_NAME}
        main.cpp
        )

target_link_libraries(${PROGRAM_NAME}
        pico_stdlib              # for core functionality
        pico_cyw43_arch_none     # we need Wifi to access the GPIO, but we don't need anything else
        )

pico_enable_stdio_usb(${PROGRAM_NAME} TRUE)
pico_enable_stdio_uart(${PROGRAM_NAME} FALSE)
suppress_tinyusb_warnings()

# create map/bin/hex file etc.
pico_add_extra_outputs(${PROGRAM_NAME})