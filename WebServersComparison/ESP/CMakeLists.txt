
cmake_minimum_required(VERSION 3.16)
set(CMAKE_EXPORT_COMPILE_COMMANDS TRUE)

set(ENV{IDF_TARGET} esp32)
include(cmake/get_esp_idf.cmake)
getEspIdf()

IF (NOT DEFINED ENV{IDF_PATH})
message(FATAL_ERROR 
        " \nIDF_PATH env variable not found. Please export needed env variables by running:\n"
        " export IDF_TOOLS_PATH=$ENV{IDF_TOOLS_PATH}; . ${esp-idf_SOURCE_DIR}/export.sh $ENV{IDF_TARGET}")
ENDIF()

message(STATUS "Include IDF project.cmake from ${esp-idf_SOURCE_DIR}")
include($ENV{IDF_PATH}/tools/cmake/project.cmake)

project(esp_webserver)