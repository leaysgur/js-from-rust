use napi::bindgen_prelude::{Function, Result};
use napi_derive::napi;

/// A simple function that demonstrates calling Rust from JS.
/// This is the most basic usecase of `napi-rs`.
#[napi]
pub fn hello(name: String) -> String {
    format!("🦀 < `Hello, {name}!`")
}

// ---

/// A simple function that calls a JS callback from Rust.
/// This demonstrates how to call a JS function directly.
#[napi]
pub fn hello_with_callback(callback: Function<String, String>) -> Result<String> {
    let result = callback.call("🦀 < `Hello, {{template}}!`".to_string())?;
    println!("{result}");
    Ok(result)
}
