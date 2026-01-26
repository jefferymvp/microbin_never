use crate::translation::{get_translation, Translation};
use crate::args::{Args, ARGS};
use actix_web::{get, web, HttpResponse, HttpRequest};
use askama::Template;

#[derive(Template)]
#[template(path = "auth_admin.html")]
struct AuthAdmin<'a> {
    args: &'a Args,
    status: String,
    text: Translation,
}

#[get("/auth_admin")]
pub async fn auth_admin(req: HttpRequest) -> HttpResponse {
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        AuthAdmin {
            args: &ARGS,
            status: String::from(""),
            text,
        }
        .render()
        .unwrap(),
    );
}

#[get("/auth_admin/{status}")]
pub async fn auth_admin_with_status(req: HttpRequest, param: web::Path<String>) -> HttpResponse {
    let status = param.into_inner();
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    return HttpResponse::Ok().content_type("text/html; charset=utf-8").body(
        AuthAdmin {
            args: &ARGS,
            status,
            text,
        }
        .render()
        .unwrap(),
    );
}
