#[macro_use]
extern crate rocket;
use rocket::serde::json::Json;

mod globals;
mod serializers;
mod utils;

use globals::LANGS;
use serializers::{LanguageResponse, RootResponse, TranslateRequest, TranslateResponse};
use utils::translate_helper;

#[get("/")]
fn root() -> Json<RootResponse<'static>> {
    let response: RootResponse = RootResponse {
        version: "0.1.0",
        status: "Ok",
    };
    Json(response)
}

#[get("/languages")]
fn languages() -> Json<LanguageResponse<'static>> {
    let response: LanguageResponse = LanguageResponse { languages: LANGS };
    Json(response)
}

#[post("/translate", data = "<body>")]
async fn translate(body: Json<TranslateRequest<'_>>) -> Json<TranslateResponse<'_>> {
    let res = translate_helper(
        body.text.to_string(),
        body.source_lang.to_string(),
        body.target_lang.to_string(),
    )
    .await;
    match res {
        Ok(s) => {
            let response: TranslateResponse = TranslateResponse {
                success: true,
                source_language: body.source_lang,
                target_language: body.target_lang,
                original_text: body.text,
                translated_text: s,
            };
            return Json(response);
        }
        _ => {
            let response: TranslateResponse = TranslateResponse {
                success: false,
                source_language: body.source_lang,
                target_language: body.target_lang,
                original_text: body.text,
                translated_text: "Unexpected error".to_string(),
            };
            return Json(response);
        }
    }
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, languages, translate])
}
