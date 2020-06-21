mod arg_parse;
mod json_cfg;
use failure::Error;

// ===== mimalloc, anecdotally faster, requires cmake and gcc =====
// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;
// ===== End mimalloc boilerplate =====

/// Main method, before quick_main! composition.
fn main() -> Result<(), Error> {
    let args = arg_parse::get_args();
    let cfg = json_cfg::get_config("/Users/int/.config/tmnt.json")?;
    println!("{:?}", cfg);
    Ok(())
}
