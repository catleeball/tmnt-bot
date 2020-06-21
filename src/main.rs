mod arg_parse;
// use mimalloc::MiMalloc;

// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    let args = arg_parse::get_args();
    println!("Hello, world!");
}
