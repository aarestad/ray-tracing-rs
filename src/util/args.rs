use args::{Args, ArgsError};

const PROGRAM_DESC: &str = "Trace some rays";

pub struct ProgramOptions {
    pub create_little_spheres: bool,
}

pub fn parse_args(input: &[String]) -> Result<ProgramOptions, ArgsError> {
    let program_name: String = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let mut args = Args::new(program_name.as_str(), PROGRAM_DESC);
    args.flag("s", "create_little_spheres", "Create little spheres");

    args.parse(input)?;

    Ok(ProgramOptions {
        create_little_spheres: args.value_of("create_little_spheres")?,
    })
}
