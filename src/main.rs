use std::time;

use gtranslate::{
    Translator,
    TranslateOptions
};

#[tokio::main]
async fn main() {
    let translator = Translator::new();
    let opts = TranslateOptions::new()
        .set_source_lang("nl")
        .set_target_lang("tr")
        .query("hallo ik ga vandaag hardlopen");
    
    let translated = translator.translate(time::Duration::from_secs(2), opts).await.unwrap();
    println!("translated: {translated}")
}
