add_executable(puzzle13 main.cpp)
target_include_directories(puzzle13 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)