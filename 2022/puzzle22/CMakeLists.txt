add_executable(puzzle22 main.cpp)
target_include_directories(puzzle22 PRIVATE ../include)
target_link_libraries(puzzle22 Eigen3::Eigen)

#copy input file to build directory
configure_file(input.txt input.txt)