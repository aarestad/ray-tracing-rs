use args::{Args, ArgsError};
use getopts::Occur;

pub struct ProgramOptions {
    pub use_bvh: bool,
    pub world_choice: u8,
    pub samples_per_pixel: u32,
    pub interactive: bool,
    pub help: bool,
    pub help_str: String,
}

pub fn parse_args(input: &[String]) -> Result<ProgramOptions, ArgsError> {
    let program_name: String = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let mut args = Args::new(program_name.as_str(), env!("CARGO_PKG_DESCRIPTION"));
    args.flag(
        "f",
        "flat",
        "For world 0, use linear HittableVec instead of BVH (default is BVH)",
    );
    args.flag("h", "help", "Print this help message");
    args.flag(
        "i",
        "interactive",
        "Open an interactive window: LMB orbit, RMB roll, wheel zoom; progressive render updates live",
    );
    args.option(
        "w",
        "world_choice",
        "Select world (0–9; 9 = Utah teapots OBJ)",
        "[0-9]",
        Occur::Optional,
        Some(String::from("0")),
    );
    args.option(
        "s",
        "samples_per_pixel",
        "Samples per pixel",
        "N",
        Occur::Optional,
        Some(String::from("100")),
    );

    args.parse(input)?;

    let samples_per_pixel: u32 = args.value_of("samples_per_pixel")?;

    Ok(ProgramOptions {
        use_bvh: !args.value_of("flat")?,
        world_choice: args.value_of("world_choice")?,
        samples_per_pixel,
        interactive: args.value_of("interactive")?,
        help: args.value_of("help")?,
        help_str: args.full_usage(),
    })
}
