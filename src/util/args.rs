use args::{ArgsError, Args};

const PROGRAM_DESC: &'static str = "Run this program";
const PROGRAM_NAME: &'static str = "program";

pub struct ProgramOptions {
    pub create_little_spheres: bool,
}

pub fn parse_args(input: &Vec<String>) -> Result<ProgramOptions, ArgsError> {
    let mut args = Args::new(PROGRAM_NAME, PROGRAM_DESC);
    args.flag("s", "create_little_spheres", "Create little spheres");

    args.parse(input)?;

    Ok(ProgramOptions {
        create_little_spheres: args.value_of("create_little_spheres")?,
    })
}
