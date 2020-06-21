mod arg_parse;
mod json_cfg;

// ===== Boilerplate for error-chain crate. =====
#[macro_use]
extern crate error_chain;
mod errors { error_chain! { } }
pub use errors::*;
quick_main!(run);
// ===== End error-chain boilerplate. =====

// ===== mimalloc, anecdotally faster, requires cmake and gcc =====
// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;
// ===== End mimalloc boilerplate =====

/// Main method, before quick_main! composition.
fn run() {
    let _args = arg_parse::get_args();

    println!("Hello, world!");
}
