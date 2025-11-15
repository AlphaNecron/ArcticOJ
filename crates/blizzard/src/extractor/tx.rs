// use async_trait::async_trait;
// use poem::http::StatusCode;
// use poem::{Error, FromRequest, Request, RequestBody};
// use sea_orm::{ConnectionTrait, DatabaseConnection, DbBackend, DbErr, ExecResult, QueryResult, Statement};
// use std::fmt::{Debug, Display};
//
// // TODO: gonna let it use db conn til I figure out a way to automatically dispose (commit/rollback) tx
//
// pub(crate) struct Extractor<'a>(&'a DatabaseConnection);
//
// impl<'a> FromRequest<'a> for Extractor<'a> {
//     async fn from_request(req: &'a Request, body: &mut RequestBody) -> Result<Self, Error> {
//         // req.data::<DatabaseConnection>().map(|c| c.begin());
//         match req.data::<DatabaseConnection>() {
//             Some(tx) => Ok(Extractor(tx)),
//             None => Err(Error::from_string("error retrieving db conn", StatusCode::INTERNAL_SERVER_ERROR))
//         }
//     }
// }
//
// #[async_trait]
// impl<'a> ConnectionTrait for Extractor<'a> {
//     fn get_database_backend(&self) -> DbBackend {
//         self.0.get_database_backend()
//     }
//
//     async fn execute_raw(&self, stmt: Statement) -> Result<ExecResult, DbErr> {
//         self.0.execute_raw(stmt).await
//     }
//
//     async fn execute_unprepared(&self, sql: &str) -> Result<ExecResult, DbErr> {
//         self.0.execute_unprepared(sql).await
//     }
//
//     async fn query_one_raw(&self, stmt: Statement) -> Result<Option<QueryResult>, DbErr> {
//         self.0.query_one_raw(stmt).await
//     }
//
//     async fn query_all_raw(&self, stmt: Statement) -> Result<Vec<QueryResult>, DbErr> {
//         self.0.query_all_raw(stmt).await
//     }
// }