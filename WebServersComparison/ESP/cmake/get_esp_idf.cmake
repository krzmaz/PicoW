# credit for the inspiration for ESP-IDF toolchain setup: @patricia-gallardo see:
# https://github.com/patricia-gallardo/hello_esp/blob/main/CMakeLists.txt
function(getEspIdf)
    message(STATUS "FetchContent_Declare esp-idf")
    include(FetchContent)
    FetchContent_Declare(
        esp-idf
        URL https://github.com/espressif/esp-idf/releases/download/v4.4.1/esp-idf-v4.4.1.zip
    )

    message(STATUS "FetchContent_GetProperties esp-idf")
    FetchContent_GetProperties(esp-idf)
    if (NOT esp-idf_POPULATED)
        message(STATUS "FetchContent_Populate esp-idf, this may take a while")
        FetchContent_Populate(esp-idf)
        set(ENV{IDF_TOOLS_PATH} ${esp-idf_BINARY_DIR})
        set(esp-idf_SOURCE_DIR  "${esp-idf_SOURCE_DIR}" CACHE INTERNAL "esp-idf_SOURCE_DIR")
        file(GLOB IDF_TOOLS_PATH_FILES $ENV{IDF_TOOLS_PATH}/* )
        if (NOT IDF_TOOLS_PATH_FILES)
            message(STATUS "Installing in: " $ENV{IDF_TOOLS_PATH})
            execute_process(COMMAND "./install.sh" $ENV{IDF_TARGET} WORKING_DIRECTORY ${esp-idf_SOURCE_DIR})
        endif()
    endif ()
    file(WRITE env.sh "# This script should be sourced, not executed.\n")
    file(APPEND env.sh "export IDF_TOOLS_PATH=$ENV{IDF_TOOLS_PATH}\n")
    file(APPEND env.sh ". ${esp-idf_SOURCE_DIR}/export.sh $ENV{IDF_TARGET}\n")
    

endfunction(getEspIdf)
