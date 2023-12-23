#[macro_use]
extern crate rocket;
use rocket::serde::{json::Json, Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct TranslateRequest<'r> {
    source_lang: &'r str,
    target_lang: &'r str,
    text: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct RootResponse<'r> {
    version: &'r str,
    status: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct LanguageResponse<'r> {
    languages: [&'r str; 10],
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct TranslateResponse<'r> {
    success: bool,
    source_language: &'r str,
    target_language: &'r str,
    original_text: &'r str,
    translated_text: &'r str,
}

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
    let response: LanguageResponse = LanguageResponse {
        languages: ["fr", "en", "hi", "bn", "ka", "de", "af", "aa", "ab", "sq"],
    };
    Json(response)
}

#[post("/translate", data = "<body>")]
fn translate(body: Json<TranslateRequest>) -> Json<TranslateResponse> {
    let response: TranslateResponse = TranslateResponse {
        success: true,
        source_language: body.source_lang,
        target_language: body.target_lang,
        original_text: body.text,
        translated_text: "<placeholder>",
    };
    Json(response)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![root, languages, translate])
}
