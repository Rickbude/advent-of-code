add_executable(puzzle16 main.cpp)
target_include_directories(puzzle16 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)