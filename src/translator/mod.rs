use std::time;
use reqwest::Method;

pub const API_URL: &'static str = "https://translate.google.com/translate_a/t";
const DST_TARGET: &'static str = "t";

/// Configuration for the translations
/// 
/// #### Example
/// ```
/// use std::time;
/// use gtranslate::translator::{
///     Translator,
///     TranslateOptions
/// };
///
/// #[tokio::main]
/// async fn main() {
///    let translator = Translator::new();
///    let opts = TranslateOptions::new()
///         .set_source_lang("nl")
///         .set_target_lang("tr")
///         .query("hallo ik ga vandaag hardlopen");
///    
///    let translated = translator.translate(time::Duration::from_secs(2), opts).await.unwrap();
///    println!("translated: {translated}")
///  }
/// ```
pub struct TranslateOptions<'a> {
    pub client: &'a str, // Client, should be 'p'
    pub source_lang: &'a str, // Source language
    pub target_lang: &'a str, // Target language
    pub dst_target: &'a str,  // Destination target, should be 't'
    pub query: &'a str // Query (text to translate from source lang to target lang)
}

impl Default for TranslateOptions<'_> {
    fn default() -> Self {
        Self { client: "p", source_lang: Default::default(), target_lang: Default::default(), dst_target: &DST_TARGET, query: Default::default() }
    }
}

impl TranslateOptions<'_> {
    /// Creates options using defaults, field ``client`` must be set to ``p``
    pub fn new() -> Self {
        TranslateOptions::default()
    }

    /// Sets the ``client`` field
    pub fn set_client(mut self, client: &'static str) -> Self {
        self.client = client;
        self
    }

    /// Sets the ``source_lang`` field
    pub fn set_source_lang(mut self, source_lang: &'static str) -> Self {
        self.source_lang = source_lang;
        self
    }

    /// Sets the ``target_lang`` field
    pub fn set_target_lang(mut self, target_lang: &'static str) -> Self {
        self.target_lang = target_lang;
        self
    }

    /// Sets the ``dst_target`` field
    pub fn set_dst_target(mut self, dst_target: &'static str) -> Self {
        self.dst_target = dst_target;
        self
    }

    /// Sets the ``query`` field
    pub fn query(mut self, query: &'static str) -> Self {
        self.query = query;
        self
    }
}

/// [Translator] instance
/// 
/// Which holds the definetive bindings for translation
pub struct Translator {
    client: reqwest::Client
}

impl Translator {
    /// Builds a new [Translator] using the default [reqwest::Client::new]
    pub fn new() -> Self {
        Translator { client: reqwest::Client::new() }
    }

    /// Builds a [Translator] using the given ``client``
    pub fn with_client(client: reqwest::Client) -> Self {
        Translator { client }
    }

    /// Translates using the given preferences ``opts`` and ``timeout``
    /// 
    /// ### Errors
    /// On failure it returns a boxed std error: [std::error::Error]
    pub async fn translate(&self, timeout: time::Duration, opts: TranslateOptions<'_>) -> Result<String, Box<dyn std::error::Error>> {
        let req = reqwest::Client::request(&self.client, Method::GET, format!("{API_URL}?client={}&sl={}&tl={}&dt={}&q={}",  
            opts.client,
            opts.source_lang,
            opts.target_lang,
            opts.dst_target,
            opts.query
        ))
            .header("Referer", "https://translate.google.com/")
            .header("User-Agent", "Mozilla/5.0 (Linux; Android 6.0; Nexus 5 Build/MRA58N) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/129.0.0.0 Mobile Safari/537.36")
            .timeout(timeout)
            .build()?;

        let res = self.client.execute(req).await?;
        if !res.status().is_success() {
            return Err(Box::new(errors::TranslatorErrors::InvalidResponse))
        }

        let body = res.text().await.unwrap();
        let parsed_body: serde_json::Value = serde_json::from_str(&body)?;
        match parsed_body.get(0) {
            Some(translated) => Ok(translated.to_string().replace("\"", "")),
            None => Err(Box::new(errors::TranslatorErrors::FailedParsing)),
        }

        
       
    }
}

pub mod errors {
    #[derive(thiserror::Error, Debug)]
    pub enum TranslatorErrors {
        #[error("got invalid response")]
        InvalidResponse,

        #[error("failed parsing")]
        FailedParsing
    }
}

mod tests {
    #[tokio::test]
    async fn test_translation() {
        use std::time;
        use crate::{
            Translator,
            TranslateOptions
        };
        
        let translator = Translator::new();
        let opts = TranslateOptions::new()
            .set_client("p")
            .set_source_lang("nl")
            .set_target_lang("tr")
            .query("In computer science, a pointer is an object in many programming languages that stores a memory address. This can be that of another value located in computer memory, or in some cases, that of memory-mapped computer hardware. A pointer references a location in memory, and obtaining the value stored at that location is known as dereferencing the pointer. As an analogy, a page number in a book's index could be considered a pointer to the corresponding page; dereferencing such a pointer would be done by flipping to the page with the given page number and reading the text found on that page. The actual format and content of a pointer variable is dependent on the underlying computer architecture.");
        
        let translated = translator.translate(time::Duration::from_secs(2), opts).await.unwrap();
        println!("translated: {translated}")
    }
}