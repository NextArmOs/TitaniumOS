fn main() {
    cc::Build::new()
        .cpp(true)
        .file("cpp_src/core.cpp")
        .compile("os_core");
}
