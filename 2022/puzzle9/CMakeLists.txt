add_executable(puzzle9 main.cpp)
target_include_directories(puzzle9 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)