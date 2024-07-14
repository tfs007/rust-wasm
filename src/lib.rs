use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::{Request, RequestInit, RequestMode, RequestCredentials, Headers, Response};
use dotenv_codegen::dotenv;

#[wasm_bindgen]
pub async fn fetch_nft_prices() -> Result<JsValue, JsValue> {
    // Load environment variables from .env file
    dotenv().ok();

    // Retrieve API key from environment variables
    let api_key = dotenv!("OPENSEA_API_KEY");

    // OpenSea API URL for fetching NFT prices 
    let url = "https://api.opensea.io/api/v1/assets";

    // Configure the request
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);
    opts.credentials(RequestCredentials::Include); // Adjust if needed

    // Create headers and add the API key
    let headers = Headers::new()?;
    headers.append("Authorization", &format!("Bearer {}", api_key))?;
    opts.headers(&headers);

    let request = Request::new_with_str_and_init(url, &opts)?;

    // Fetch the data
    let window = web_sys::window().expect("no global `window` exists");
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();

    // Parse the response
    let json = JsFuture::from(response.json()?).await?;
    Ok(json)
}
