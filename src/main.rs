mod arg_parse;

#[cfg(feature = "mimalloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mimalloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    #[cfg(all(feature = "cli_args", feature = "toml_cfg"))]
    {
        // Gen logo with config file and cli args; cli args take precidence.
        let args = arg_parse::get_args();
    }
    #[cfg(all(feature = "cli_args", not(feature = "toml_cfg")))]
    {
        let args = arg_parse::get_args();
        // Gen logo using only cli arg specifications.
    }
    #[cfg(all(feature = "toml_cfg", not(feature = "cli_args")))]
    { 
        // Gen logo using only config specifications.
    }
    println!("Hello, world!");
}
