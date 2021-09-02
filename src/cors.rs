use actix_web::middleware;

pub fn cors() -> middleware::DefaultHeaders {
  middleware::DefaultHeaders::new()
    .header("Access-Control-Allow-Origin", "*")
    .header(
      "Access-Control-Allow-Methods",
      "GET, POST, PATCH, PUT, DELETE, OPTIONS",
    )
    .header(
      "Access-Control-Allow-Headers",
      "Content-Type, Access-Control-Allow-Headers, token",
    )
    .header("vary", "Origin")
    .header("vary", "Access-Control-Request-Method")
    .header("vary", "Access-Control-Request-Headers")
    .header("Access-Control-Max-Age", 600)
    .header(
      "strict-transport-security",
      "max-age=31536000; includeSubDomains; preload",
    )
}
