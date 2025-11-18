use crate::executor::error::SelfTestErr;

mod error;
mod gcc;
mod python;

#[derive(Debug)]
pub(crate) struct Desc {
    pub(crate) id: String,
}

#[async_trait::async_trait]
pub(crate) trait Executor: 'static {
    fn desc(&self) -> Desc;
    fn argv0(&self) -> Option<String>;
    async fn self_test(&self) -> Result<String, SelfTestErr>;
    fn boxed(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

pub(crate) fn executors() -> Vec<Box<dyn Executor + 'static>> {
    vec![gcc::new(), python::python3(), python::pypy3()]
}
