// Entry point for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main::run().await;
    Ok(())
}
use paper_blade::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    let poly = Polyhedron::cube();
    println!("hey there!\n{:?}", poly,);
}
