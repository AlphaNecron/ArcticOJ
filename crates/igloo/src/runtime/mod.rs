use crate::runtime::error::SelfTestErr;

mod error;
mod gcc;
mod python;

#[derive(Debug)]
pub(crate) struct RtDesc {
    pub(crate) id: String,
}

#[async_trait::async_trait]
pub(crate) trait Runtime: 'static {
    fn desc(&self) -> RtDesc;
    fn argv0(&self) -> Option<String>;
    async fn self_test(&self) -> Result<String, SelfTestErr>;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub(crate) fn runtimes() -> Vec<Box<dyn Runtime + 'static>> {
    vec![gcc::new(), python::python3(), python::pypy3()]
}
