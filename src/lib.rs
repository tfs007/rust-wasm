use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::JsFuture;
use web_sys::Request;
use web_sys::RequestInit;
use web_sys::RequestMode;
use web_sys::Response;

#[wasm_bindgen]
pub async fn fetch_nft_prices() -> Result<JsValue, JsValue> {
    // OpenSea API URL for fetching NFT prices (example endpoint)
    let url = "https://api.opensea.io/api/v1/assets";

    // Configure the request
    let mut opts = RequestInit::new();
    opts.method("GET");
    opts.mode(RequestMode::Cors);

    let request = Request::new_with_str_and_init(url, &opts)?;

    // Fetch the data
    let window = web_sys::window().expect("no global `window` exists");
    let response_value = JsFuture::from(window.fetch_with_request(&request)).await?;
    let response: Response = response_value.dyn_into().unwrap();

    // Parse the response
    let json = JsFuture::from(response.json()?).await?;
    Ok(json)
}
