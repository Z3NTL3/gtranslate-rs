# gtranslate-rs
Rust bindings for Google Translations

#### Quick start
Write the following in your Rust project:

> ``cargo add gtranslate``

#### Features
- No API key requirement
- Supports timeout
- Supports the use of a custom [reqwest::Client](https://docs.rs/reqwest/latest/reqwest/struct.Client.html)


#### Example
```rust
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

```