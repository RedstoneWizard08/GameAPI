use actix_web::{HttpRequest, HttpResponse};

use crate::util::{print_path, build_response};

pub async fn error(req: HttpRequest) -> HttpResponse {
    let path = req.uri().path();

    print_path(req.clone(), "404");

    let mut resp = String::new();
    resp.push_str("{ ");
    resp.push_str(
        format!(
            "\"code\": \"404\", \"error\": \"Route not found: {}\"",
            path
        )
        .as_str(),
    );
    resp.push_str(" }");

    return build_response(resp, false);
}
