use actix_web::{HttpRequest, HttpResponse, HttpResponseBuilder};
use reqwest::StatusCode;

pub fn get_status(status: &str) -> &'static str {
    if status == "404" {
        return "\x1b[1;31m404 NOT FOUND\x1b[0m";
    } else if status == "400" {
        return "\x1b[1;31m400 BAD REQUEST\x1b[0m";
    } else if status == "200" {
        return "\x1b[1;34m200 OK\x1b[0m";
    } else if status == "302" {
        return "\x1b[1;32m302 FOUND\x1b[0m";
    } else {
        return "\x1b[1;35m202 ACCEPTED\x1b[0m";
    }
}

pub fn print_path(request: HttpRequest, status: &str) {
    let protocol = "\x1b[34m[\x1b[36mGET\x1b[34m]";
    let path = request.path();
    println!("{} \x1b[32m{path} \x1b[35mHTTP/1.1 {}", protocol, get_status(status));
}

pub fn build_response(data: String, is_redirect: bool) -> HttpResponse {
    let mut code = StatusCode::OK;
    
    if is_redirect {
        code = StatusCode::FOUND;
    }

    let mut resp = HttpResponseBuilder::new(code);

    if is_redirect {
        resp.insert_header(("Location", data.clone()));
    }
    
    return resp.body(data);
}
