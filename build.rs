fn main() {
    // Set CMAKE_POLICY_VERSION_MINIMUM to 3.5 to fix compatibility issues
    // with older CMakeLists.txt files that specify cmake_minimum_required < 3.5
    std::env::set_var("CMAKE_POLICY_VERSION_MINIMUM", "3.5");
}