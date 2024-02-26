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
    let poly = Poly {
        edges: vec![
            vec![1, 2, 7], // 0
            vec![0, 3, 6], // 1
            vec![0, 3, 5], // 2
            vec![1, 2, 4], // 3
            vec![3, 5, 6], // 4
            vec![2, 4, 7], // 5
            vec![1, 4, 7], // 6
            vec![0, 5, 6], // 7
        ],
        faces: vec![],
    };
    println!("hey there! {:?}", poly);
}
