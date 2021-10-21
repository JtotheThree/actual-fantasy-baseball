use crate::config::CONFIG;
use crate::AppSchema;

use common::decode_token;
use actix_web::{HttpRequest, web::Data};
use async_graphql_actix_web::{Request, Response};

pub async fn index(
    schema: Data<AppSchema>,
    http_req: HttpRequest,
    gql_request: Request,
) -> Response {
    let mut request = gql_request.into_inner();

    if let Some(token) = get_header_token(http_req) {
        if let Ok(token) = decode_token(&token, &CONFIG.session.secret) {
            request = request.data(token);
        }
    }

    schema.execute(request).await.into()
}

pub fn get_header_token(http_req: HttpRequest) -> Option<String> {
    http_req
        .headers()
        .get("Authorization")
        .and_then(|header_value| {
            header_value.to_str().ok().map(|s| {
                let jwt_start_index = "Bearer ".len();
                let jwt = s[jwt_start_index..s.len()].to_string();

                jwt
            })
        })
}