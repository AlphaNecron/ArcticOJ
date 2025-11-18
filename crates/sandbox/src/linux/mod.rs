use std::error::Error;

mod env;
mod prelude;
mod syscall;

pub(crate) fn init() -> Result<(), impl Error> {
    // let hier = Box::new(hierarchies::V2::new());
    // let cg = CgroupBuilder::new("igloo.slice").set_specified_controllers(vec![
    //     "cpu".to_string(),
    //     "cpuset".to_string()
    // ]).build(hier)?;
    // SLICE.set(cg).unwrap();
    // Ok(())
    Ok::<(), std::io::Error>(())
}
