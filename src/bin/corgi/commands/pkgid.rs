use crate::command_prelude::*;

use corgi::ops;
use corgi::util::print_available_packages;

pub fn cli() -> Command {
    subcommand("pkgid")
        .about("Print a fully qualified package specification")
        .arg_quiet()
        .arg(Arg::new("spec"))
        .arg_package("Argument to get the package ID specifier for")
        .arg_manifest_path()
        .after_help("Run `corgi help pkgid` for more detailed information.\n")
}

pub fn exec(config: &mut Config, args: &ArgMatches) -> CliResult {
    let ws = args.workspace(config)?;
    if args.is_present_with_zero_values("package") {
        print_available_packages(&ws)?
    }
    let spec = args
        .get_one::<String>("spec")
        .or_else(|| args.get_one::<String>("package"))
        .map(String::as_str);
    let spec = ops::pkgid(&ws, spec)?;
    corgi::drop_println!(config, "{}", spec);
    Ok(())
}