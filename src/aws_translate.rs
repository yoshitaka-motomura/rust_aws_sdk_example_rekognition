// aws_translate.rs

use aws_config::SdkConfig;
use aws_sdk_translate as translate;
use log::*;

pub async fn get_translate_text(
    config: SdkConfig,
    text: String,
    source_language_code: String,
    target_language_code: String
) -> Result<String, translate::Error> {
    let client = translate::Client::new(&config);
    let result = match client
        .translate_text()
        .set_text(Some(text))
        .set_source_language_code(Some(source_language_code))
        .set_target_language_code(Some(target_language_code))
        .send()
        .await
    {
        Ok(output) => output,
        Err(error) => {
            error!("Error: {:?}", error);
            return Err(error.into());
        }
    };
    Ok(result.translated_text().to_string())
}