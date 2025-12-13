mod compiled;
mod error;
mod gcc;
mod go;
mod prelude;
mod python;

// executor-specific stuff
// #[async_trait::async_trait]
// pub(crate) trait _Executor: 'static + Send + Sync {
//     async fn self_test(&self, env: &'static BoxedEnv) -> Result<String, SelfTestErr>;
//     // async fn judge(&self);
//     async fn get_version(&self, exec_path: impl Into<String>);
// }
