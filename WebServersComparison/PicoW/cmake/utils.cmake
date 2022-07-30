# see https://github.com/raspberrypi/pico-sdk/issues/780
function(suppress_tinyusb_warnings)
        set_source_files_properties(
                ${PICO_TINYUSB_PATH}/src/portable/raspberrypi/rp2040/rp2040_usb.c
                PROPERTIES
                COMPILE_FLAGS "-Wno-stringop-overflow -Wno-array-bounds")
endfunction()