// use std::ops::{Deref, DerefMut};
//
// use poem::{IntoResponse, Request, RequestBody, Response, Result};
//
// use poem_openapi::{impl_apirequest_for_payload, payload::{Json, ParsePayload, Payload}, registry::{MetaResponses, MetaSchemaRef, Registry}, types::{ParseFromJSON, ToJSON, Type}, ApiResponse};
//
// #[derive(Debug, Clone, Eq, PartialEq, Default)]
// pub(crate) struct InlineJson<T>(pub(crate) T);
//
// impl<T> Deref for InlineJson<T> {
//     type Target = T;
//
//     fn deref(&self) -> &Self::Target {
//         &self.0
//     }
// }
//
// impl<T> DerefMut for InlineJson<T> {
//     fn deref_mut(&mut self) -> &mut Self::Target {
//         &mut self.0
//     }
// }
//
// impl<T: Type> Payload for InlineJson<T> {
//     const CONTENT_TYPE: &'static str = Json::<T>::CONTENT_TYPE;
//
//     fn check_content_type(content_type: &str) -> bool {
//         Json::<T>::check_content_type(content_type)
//     }
//
//     fn schema_ref() -> MetaSchemaRef {
//         MetaSchemaRef::Inline()
//     }
//
//     fn register(_registry: &mut Registry) {}
// }
//
// impl<T: ParseFromJSON> ParsePayload for InlineJson<T> {
//     const IS_REQUIRED: bool = T::IS_REQUIRED;
//
//     async fn from_request(request: &Request, body: &mut RequestBody) -> Result<Self> {
//         let j = Json::<T>::from_request(request, body).await?;
//         Ok(Self(j.0))
//     }
// }
//
// impl<T: ToJSON> IntoResponse for InlineJson<T> {
//     fn into_response(self) -> Response {
//         poem::web::Json(self.0.to_json()).into_response()
//     }
// }
//
// impl<T: ToJSON> ApiResponse for InlineJson<T> {
//     fn meta() -> MetaResponses {
//         Json::<T>::meta()
//     }
//
//     fn register(_registry: &mut Registry) {}
// }
//
// impl_apirequest_for_payload!(InlineJson<T>, T: ParseFromJSON);
