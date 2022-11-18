
#[global_allocator]
static A: std::alloc::System = std::alloc::System;


fn main() {
    let file_path = std::env::args()
        .nth(1)
        .expect("could not be able to find the file");
    
    let file_content = std::fs::read_to_string(file_path)
        .expect("could not be able to read the file");
    println!("{}", file_content);
}
