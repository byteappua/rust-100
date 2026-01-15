fn main() {
    // Compile the C library
    cc::Build::new()
        .file("src/mylib.c")
        .compile("mylib");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=src/mylib.c");
}
