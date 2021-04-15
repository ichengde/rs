fn json_error_handler(err: error::JsonPayloadError, _req: &HttpRequest) -> error::Error {
    let detail = err.to_string();
    let response = match &err {
        error::JsonPayloadError::ContentType => HttpResponse::UnsupportedMediaType()
            .content_type("text/plain")
            .body(detail),
        _ => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(detail),
    };
    error::InternalError::from_response(err, response).into()
}
