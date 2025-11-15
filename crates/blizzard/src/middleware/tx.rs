// use poem::{Endpoint, Error, IntoResponse, Request, Response};
// use sea_orm::{DatabaseTransaction, DbErr, TransactionTrait};
// use std::sync::{Arc, Mutex};
// use tracing::error;
//
// pub(crate) async fn middleware<E: Endpoint>(next: E, mut req: Request) -> Result<Response, Error> {
//     let db = req
//         .data::<&'static sea_orm::DatabaseConnection>()
//         .ok_or_else(|| {
//             Error::from_string(
//                 "err retrieving db conn",
//                 poem::http::StatusCode::INTERNAL_SERVER_ERROR,
//             )
//         })?;
//     let tx = db.begin().await.map_err(|e| {
//         error!(err = e.to_string(), "err beginning tx");
//         Error::from_string(
//             "err beginning tx",
//             poem::http::StatusCode::INTERNAL_SERVER_ERROR,
//         )
//     })?;
//     let actual_tx = Arc::new(Mutex::new(tx));
//     req.extensions_mut().insert(actual_tx);
//     match next.call(req).await {
//         Ok(res) => {
//             let res = res.into_response();
//             if (res.is_success()) {
//                 actual_tx.lock().unwrap().commit().await.map_err(|e| {
//                     error!(err = e.to_string(), "err committing tx");
//                     Error::from_string(
//                         "err committing tx",
//                         poem::http::StatusCode::INTERNAL_SERVER_ERROR,
//                     )
//                 })?;
//             } else {
//                 actual_tx
//                     .lock()
//                     .unwrap()
//                     .rollback()
//                     .await
//                     .map_err(|e| {
//                         error!(err = e.to_string(), "err rolling back tx");
//                         Error::from_string(
//                             "err rolling back tx",
//                             poem::http::StatusCode::INTERNAL_SERVER_ERROR,
//                         )
//                     })?;
//             }
//             Ok(res)
//         }
//         Err(e) => Err(e),
//     }
// }
