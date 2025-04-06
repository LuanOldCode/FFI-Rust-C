fn main() {
    cc::Build::new().file("c_code/hello.c").compile("hello");
}
