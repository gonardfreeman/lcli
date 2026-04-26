pub static VERSION: &str = env!("CARGO_PKG_VERSION");
pub const HELP_TEXT: &str = "
Usage: lcli [options] <inputs>...
       lcli --help
       lcli (-v|--version)

Inputs:

Options:
    -h, --help      Print help
    -v, --version   Print version
";
