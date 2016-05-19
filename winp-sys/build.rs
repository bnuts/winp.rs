extern crate gcc;

fn main() {
    gcc::Config::new()
        .include("winp.c/src")
        .file("winp.c/src/winp.c")
        .file("winp.c/src/pipes.c")
        .compile("libwinp.a");
}
