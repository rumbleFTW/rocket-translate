extern crate rocket;
use rocket::serde::{Deserialize, Serialize};

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct TranslateRequest<'r> {
    pub source_lang: &'r str,
    pub target_lang: &'r str,
    pub text: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct RootResponse<'r> {
    pub version: &'r str,
    pub status: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct LanguageResponse<'r> {
    pub languages: [&'r str; 30],
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub struct TranslateResponse<'r> {
    pub success: bool,
    pub source_language: &'r str,
    pub target_language: &'r str,
    pub original_text: &'r str,
    pub translated_text: String,
}
