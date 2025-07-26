use axum::{
    http::StatusCode,
    response::{
        IntoResponse,
        Response,
    },
};

pub struct WebserverError(#[allow(dead_code)] pub anyhow::Error);

impl IntoResponse for WebserverError {
    fn into_response(self) -> Response {
        (StatusCode::INTERNAL_SERVER_ERROR).into_response()
    }
}

impl<E> From<E> for WebserverError
where
    E: Into<anyhow::Error>,
{
    fn from(err: E) -> Self {
        Self(err.into())
    }
}
