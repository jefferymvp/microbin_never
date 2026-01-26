use actix_web::{Error, HttpResponse, HttpRequest};
use askama::Template;

use crate::args::{Args, ARGS};
use crate::translation::{get_translation, Translation};

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate<'a> {
    pub args: &'a Args,
    pub text: Translation,
}

pub async fn not_found(req: HttpRequest) -> Result<HttpResponse, Error> {
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(ErrorTemplate { args: &ARGS, text }.render().unwrap()))
}
