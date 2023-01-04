extern "C" {
    fn hello_world();
}

fn main() {
    println!("Hello, world!");
    unsafe {
        hello_world();
    }
}
