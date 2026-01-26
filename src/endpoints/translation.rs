use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use actix_web::cookie::Cookie;

#[get("/set_lang/{lang}")]
pub async fn set_lang(req: HttpRequest, path: web::Path<String>) -> impl Responder {
    let lang = path.into_inner();
    
    // Validate lang
    let lang_val = match lang.as_str() {
        "zh" => "zh",
        _ => "en",
    };
    
    // Get referer or default to "/"
    let redirect_to = req
        .headers()
        .get("referer")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("/");

    HttpResponse::Found()
        .append_header(("Location", redirect_to))
        .cookie(
            Cookie::build("lang", lang_val)
                .path("/")
                .finish(),
        )
        .finish()
}
