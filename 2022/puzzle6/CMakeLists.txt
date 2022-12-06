add_executable(puzzle6 main.cpp)
target_include_directories(puzzle6 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)