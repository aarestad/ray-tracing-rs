use args::{Args, ArgsError};
use getopts::Occur;

pub struct ProgramOptions {
    pub create_little_spheres: bool,
    pub use_bvh: bool,
    pub world_choice: u8,
}

pub fn parse_args(input: &[String]) -> Result<ProgramOptions, ArgsError> {
    let program_name: String = format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let mut args = Args::new(program_name.as_str(), env!("CARGO_PKG_DESCRIPTION"));
    args.flag("s", "create_little_spheres", "Create little spheres");
    args.flag(
        "b",
        "use_bvh",
        "Use BoundedVolumeHierarchy instead of HittableVec",
    );
    args.option("w", "world_choice", "Select world (int from 1-8", "[1-8]", Occur::Optional, Option::Some(String::from("0")));

    args.parse(input)?;

    Ok(ProgramOptions {
        create_little_spheres: args.value_of("create_little_spheres")?,
        use_bvh: args.value_of("use_bvh")?,
        world_choice: args.value_of("world_choice")?,
    })
}
