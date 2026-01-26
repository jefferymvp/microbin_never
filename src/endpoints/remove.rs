use actix_multipart::Multipart;
use actix_web::{get, post, web, Error, HttpResponse, HttpRequest};

use crate::args::ARGS;
use crate::endpoints::errors::ErrorTemplate;
use crate::pasta::PastaFile;
use crate::util::animalnumbers::to_u64;
use crate::util::auth;
use crate::util::db::delete;
use crate::util::hashids::to_u64 as hashid_to_u64;
use crate::util::misc::remove_expired;
use crate::translation::get_translation;
use crate::AppState;
use askama::Template;
use std::fs;

#[get("/remove/{id}")]
pub async fn remove(req: HttpRequest, data: web::Data<AppState>, id: web::Path<String>) -> HttpResponse {
    let mut pastas = data.pastas.lock().unwrap();

    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id.into_inner()).unwrap_or(0)
    };

    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            // if it's encrypted or read-only, it needs password to be deleted
            // OR if it is not editable (public immutable), it needs admin password to be deleted
            if pasta.encrypt_server || pasta.readonly || !pasta.editable {
                return HttpResponse::Found()
                    .append_header((
                        "Location",
                        format!("{}/auth_remove_private/{}", ARGS.public_path_as_str(), pasta.id_as_animals()),
                    ))
                    .finish();
            }

            // remove the file itself
            if let Some(PastaFile { name, .. }) = &pasta.file {
                if fs::remove_file(format!(
                    "{}/attachments/{}/{}",
                    ARGS.data_dir,
                    pasta.id_as_animals(),
                    name
                ))
                .is_err()
                {
                    log::error!("Failed to delete file {}!", name)
                }

                // and remove the containing directory
                if fs::remove_dir(format!(
                    "{}/attachments/{}/",
                    ARGS.data_dir,
                    pasta.id_as_animals()
                ))
                .is_err()
                {
                    log::error!("Failed to delete directory {}!", name)
                }
            }

            // remove it from in-memory pasta list
            pastas.remove(i);

            delete(Some(&pastas), Some(id));

            return HttpResponse::Found()
                .append_header(("Location", format!("{}/list", ARGS.public_path_as_str())))
                .finish();
        }
    }

    remove_expired(&mut pastas);
    
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(ErrorTemplate { args: &ARGS, text }.render().unwrap())
}

#[post("/remove/{id}")]
pub async fn post_remove(
    req: HttpRequest,
    data: web::Data<AppState>,
    id: web::Path<String>,
    payload: Multipart,
) -> Result<HttpResponse, Error> {
    let id = if ARGS.hash_ids {
        hashid_to_u64(&id).unwrap_or(0)
    } else {
        to_u64(&id.into_inner()).unwrap_or(0)
    };

    let mut pastas = data.pastas.lock().unwrap();

    remove_expired(&mut pastas);

    let password = auth::password_from_multipart(payload).await?;
    
    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);

    for (i, pasta) in pastas.iter().enumerate() {
        if pasta.id == id {
            if pastas[i].readonly || pastas[i].encrypt_server || !pastas[i].editable {
                if password != *"" {
                    let mut is_confirmed = false;

                    // Check if user typed the correct confirmation word
                    if password.trim() == text.remove_confirm_word {
                        is_confirmed = true;
                    }

                    if is_confirmed {
                        // remove the file itself
                        if let Some(PastaFile { name, .. }) = &pasta.file {
                            if fs::remove_file(format!(
                                "{}/attachments/{}/{}",
                                ARGS.data_dir,
                                pasta.id_as_animals(),
                                name
                            ))
                            .is_err()
                            {
                                log::error!("Failed to delete file {}!", name)
                            }

                            // and remove the containing directory
                            if fs::remove_dir(format!(
                                "{}/attachments/{}/",
                                ARGS.data_dir,
                                pasta.id_as_animals()
                            ))
                            .is_err()
                            {
                                log::error!("Failed to delete directory {}!", name)
                            }
                        }

                        // remove it from in-memory pasta list
                        pastas.remove(i);

                        delete(Some(&pastas), Some(id));

                        return Ok(HttpResponse::Found()
                            .append_header((
                                "Location",
                                format!("{}/list", ARGS.public_path_as_str()),
                            ))
                            .finish());
                    } else {
                        return Ok(HttpResponse::Found()
                            .append_header((
                                "Location",
                                format!("{}/auth_remove_private/{}/incorrect", ARGS.public_path_as_str(), pasta.id_as_animals()),
                            ))
                            .finish());
                    }
                } else {
                    return Ok(HttpResponse::Found()
                        .append_header((
                            "Location",
                            format!("{}/auth_remove_private/{}/incorrect", ARGS.public_path_as_str(), pasta.id_as_animals()),
                        ))
                        .finish());
                }
            }

            return Ok(HttpResponse::Found()
                .append_header((
                    "Location",
                    format!(
                        "{}/upload/{}",
                        ARGS.public_path_as_str(),
                        pastas[i].id_as_animals()
                    ),
                ))
                .finish());
        }
    }

    let lang = req.cookie("lang").map(|c| c.value().to_string()).unwrap_or_else(|| "zh".to_string());
    let text = get_translation(&lang);
    
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(ErrorTemplate { args: &ARGS, text }.render().unwrap()))
}
