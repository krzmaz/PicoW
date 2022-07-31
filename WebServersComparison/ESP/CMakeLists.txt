
cmake_minimum_required(VERSION 3.16)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

set(ENV{IDF_TARGET} esp32)
set(CMAKE_BUILD_TYPE Release)
include(${CMAKE_SOURCE_DIR}/cmake/get_esp_idf.cmake)
getEspIdf()

if (NOT DEFINED ENV{IDF_PATH})
    message(ERROR
        " Make sure to source the generated env.sh file"
    )
else()
    message(STATUS "Include IDF project.cmake from ${esp-idf_SOURCE_DIR}")
    include($ENV{IDF_PATH}/tools/cmake/project.cmake)
endif()



project(esp_webserver)