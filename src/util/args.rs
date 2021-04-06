use args::{Args, ArgsError};

const PROGRAM_DESC: &str = "Trace some rays";
const PROGRAM_NAME: &str = "ray-tracer";

pub struct ProgramOptions {
    pub create_little_spheres: bool,
}

pub fn parse_args(input: &[String]) -> Result<ProgramOptions, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("s", "create_little_spheres", "Create little spheres");

    args.parse(input)?;

    Ok(ProgramOptions {
        create_little_spheres: args.value_of("create_little_spheres")?,
    })
}
