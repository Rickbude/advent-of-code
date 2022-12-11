add_executable(puzzle11 main.cpp)
target_include_directories(puzzle11 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)