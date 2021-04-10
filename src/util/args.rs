use args::{Args, ArgsError};

pub struct ProgramOptions {
    pub create_little_spheres: bool,
    pub use_bvh: bool,
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

    args.parse(input)?;

    Ok(ProgramOptions {
        create_little_spheres: args.value_of("create_little_spheres")?,
        use_bvh: args.value_of("use_bvh")?,
    })
}
