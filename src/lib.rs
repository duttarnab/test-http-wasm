mod utils;


#[cfg(not(target_arch = "wasm32"))]
mod blocking {
    use reqwest::blocking::Client;
    use std::error::Error;

    pub fn fetch_data(url: &str) -> Result<String, Box<dyn Error>> {
        let client = Client::new();
        let response = client.get(url).send()?.text()?;
        Ok(response)
    }
}

#[cfg(target_arch = "wasm32")]
mod async_mod {
    
    use reqwest::Client;
    use wasm_bindgen::prelude::*;
    use wasm_bindgen_futures::spawn_local;

    #[wasm_bindgen]
    pub fn fetch_data(url: String
        //, callback: js_sys::Function
    ) {
        spawn_local(async move {
            let client = Client::new();
            let result = match client.get(&url).send().await {
                Ok(response) => match response.text().await {
                    Ok(text) => Ok(text),
                    Err(err) => Err(err.to_string()),
                },
                Err(err) => Err(err.to_string()),
            };
            alert(&result.unwrap())

            // Pass the result back to JavaScript
            //let _ = callback.call1(&JsValue::NULL, &JsValue::from(result));
        });
    }

    #[wasm_bindgen]
extern {
    fn alert(s: &str);
}
}

// Re-export the appropriate module based on the target architecture
#[cfg(not(target_arch = "wasm32"))]
pub use blocking::fetch_data;

#[cfg(target_arch = "wasm32")]
pub use async_mod::fetch_data;
