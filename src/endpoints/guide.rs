use crate::translation::{get_translation, Translation};
use crate::args::{Args, ARGS};
use actix_web::{get, HttpResponse, HttpRequest};
use askama::Template;

#[derive(Template)]
#[template(path = "guide.html")]
struct Guide<'a> {
    args: &'a Args,
    text: Translation,
    lang: String,
}

#[get("/guide")]
pub async fn guide(req: HttpRequest) -> HttpResponse {
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(Guide { args: &ARGS, text, lang }.render().unwrap())
}
