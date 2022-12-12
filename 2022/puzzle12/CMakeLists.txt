add_executable(puzzle12 main.cpp)
target_include_directories(puzzle12 PRIVATE ../include)
target_link_libraries(puzzle12 Eigen3::Eigen)

#copy input file to build directory
configure_file(input.txt input.txt)