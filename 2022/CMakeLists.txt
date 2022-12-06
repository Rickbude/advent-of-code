cmake_minimum_required(VERSION 3.17)
project(aoc_2022)

#Add all subdirectories that satisfy the pattern puzzle\d+
file(GLOB sources_list LIST_DIRECTORIES true puzzle*)
foreach(dir ${sources_list})
    IF(IS_DIRECTORY ${dir})
        add_subdirectory(${dir})
    ELSE()
        CONTINUE()
    ENDIF()
endforeach()