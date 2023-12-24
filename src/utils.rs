use rust_bert::pipelines::common::{ModelResource, ModelType};
use rust_bert::pipelines::translation::{Language, TranslationConfig, TranslationModel};
use rust_bert::resources::RemoteResource;
use rust_bert::t5::{T5ConfigResources, T5ModelResources, T5VocabResources};
use rust_bert::RustBertError;
use tch::Device;

fn initialize() -> Result<TranslationModel, RustBertError> {
    let model_resource = RemoteResource::from_pretrained(T5ModelResources::T5_BASE);
    let config_resource = RemoteResource::from_pretrained(T5ConfigResources::T5_BASE);
    let vocab_resource = RemoteResource::from_pretrained(T5VocabResources::T5_BASE);

    let source_languages = [
        Language::English,
        Language::ChineseMandarin,
        Language::Spanish,
        Language::Hindi,
        Language::Arabic,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
        Language::Indonesian,
        Language::German,
        Language::French,
        Language::Italian,
        Language::Turkish,
        Language::Vietnamese,
        Language::Telugu,
        Language::Marathi,
        Language::Tamil,
        Language::Gujarati,
        Language::Polish,
        Language::Ukrainian,
        Language::Malayalam,
        Language::Kannada,
        Language::Oriya,
        Language::Burmese,
        Language::Panjabi,
        Language::Chinese,
        Language::Maithili,
        Language::Tagalog,
    ];
    let target_languages = [
        Language::English,
        Language::ChineseMandarin,
        Language::Spanish,
        Language::Hindi,
        Language::Arabic,
        Language::Bengali,
        Language::Portuguese,
        Language::Russian,
        Language::Urdu,
        Language::Indonesian,
        Language::German,
        Language::French,
        Language::Italian,
        Language::Turkish,
        Language::Vietnamese,
        Language::Telugu,
        Language::Marathi,
        Language::Tamil,
        Language::Gujarati,
        Language::Polish,
        Language::Ukrainian,
        Language::Malayalam,
        Language::Kannada,
        Language::Oriya,
        Language::Burmese,
        Language::Panjabi,
        Language::Chinese,
        Language::Maithili,
        Language::Tagalog,
    ];

    let translation_config = TranslationConfig::new(
        ModelType::T5,
        ModelResource::Torch(Box::new(model_resource)),
        config_resource,
        vocab_resource,
        None,
        source_languages,
        target_languages,
        Device::cuda_if_available(),
    );
    TranslationModel::new(translation_config)
}

fn get_language(lang: String) -> Language {
    match lang.as_str() {
        "English" => Language::English,
        "ChineseMandarin" => Language::ChineseMandarin,
        "Spanish" => Language::Spanish,
        "Hindi" => Language::Hindi,
        "Arabic" => Language::Arabic,
        "Bengali" => Language::Bengali,
        "Portuguese" => Language::Portuguese,
        "Russian" => Language::Russian,
        "Indonesian" => Language::Indonesian,
        "German" => Language::German,
        "French" => Language::French,
        "Italian" => Language::Italian,
        "Turkish" => Language::Turkish,
        "Vietnamese" => Language::Vietnamese,
        "Telugu" => Language::Telugu,
        "Marathi" => Language::Marathi,
        "Tamil" => Language::Tamil,
        "Urdu" => Language::Urdu,
        "Gujarati" => Language::Gujarati,
        "Polish" => Language::Polish,
        "Ukrainian" => Language::Ukrainian,
        "Malayalam" => Language::Malayalam,
        "Kannada" => Language::Kannada,
        "Oriya" => Language::Oriya,
        "Burmese" => Language::Burmese,
        "Panjabi" => Language::Panjabi,
        "Chinese" => Language::Chinese,
        "Maithili" => Language::Maithili,
        "Tagalog" => Language::Tagalog,
        _ => Language::English,
    }
}

pub async fn translate_helper(
    text: String,
    src: String,
    target: String,
) -> Result<String, RustBertError> {
    let model = initialize()?;
    let result = model.translate(&[text], get_language(src), get_language(target))?;
    let Some(translated_text) = result.first() else {todo!()};
    return Ok(translated_text.to_string());
}
