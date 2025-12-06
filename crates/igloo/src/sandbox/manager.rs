use super::cg;
use super::container::Container;

// dunno what to name here, Pool is not a bad choice but `igloo` should be handling it, not `sandbox`...
pub struct Manager;

#[allow(clippy::new_without_default)]
impl Manager {
    pub fn new() -> Self {
        cg::init().expect("err init cgroup tree");
        Self
    }

    pub fn create_container(&self, id: impl Into<String>) -> Container {
        Container::new(id)
    }
}

impl Drop for Manager {
    fn drop(&mut self) {
        cg::destroy().ok();
    }
}
