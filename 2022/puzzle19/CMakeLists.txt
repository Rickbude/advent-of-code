add_executable(puzzle19 main.cpp)
target_link_libraries(puzzle19 Eigen3::Eigen)
target_link_libraries(puzzle19 OpenMP::OpenMP_CXX)
target_include_directories(puzzle19 PRIVATE ../include)

#copy input file to build directory
configure_file(input.txt input.txt)