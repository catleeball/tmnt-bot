mod arg_parse;
mod json_cfg;
use failure::Error;

// ===== mimalloc requires cmake and gcc =====
// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

fn main() -> Result<(), Error> {
    let args = arg_parse::get_args();
    let cfg = json_cfg::get_config("/Users/int/.config/tmnt.json")?;
    println!("{:?}", cfg);
    println!("logo {} musr {}", cfg.logo_generator, cfg.mastodon_username);
    Ok(())
}
