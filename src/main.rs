mod json_cfg;
use failure::Error;

// ===== mimalloc requires cmake and gcc =====
// use mimalloc::MiMalloc;
// #[global_allocator]
// static GLOBAL: MiMalloc = MiMalloc;

/// Entry point to software.
/// 
/// # Arguments
/// None.
/// 
/// # Examples
/// ```
/// result = main();
/// assert_eq!(result, Ok(()));
/// ```
fn main() -> Result<(), Error> {
    let cfg = json_cfg::get_config("/Users/int/.config/tmnt.json")?;
    println!("{:?}", cfg);
    println!("logo {} musr {}", cfg.logo_generator, cfg.mastodon_username);
    Ok(())
}
