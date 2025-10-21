use std::sync::mpsc;

use napi::{
    Status,
    bindgen_prelude::FnArgs,
    threadsafe_function::{ThreadsafeFunction, ThreadsafeFunctionCallMode},
};
use napi_derive::napi;

/// A simple function that demonstrates calling Rust from JS.
/// This is the most basic usecase of `napi-rs`.
#[napi]
pub fn hello(name: String) -> String {
    format!("🦀 < `Hello, {name}!`")
}

// ---

pub type JsCallback = ThreadsafeFunction<
    // The input type passed from Rust to JS
    FnArgs<(String,)>,
    // The return type from JS to Rust
    String,
    // The arguments for the JS function (usually same as the first args)
    FnArgs<(String,)>,
    // The error type (default: Status)
    Status,
    // Whether errors are handled by the callback (true) or are fatal (false)
    false,
>;

/// A simple function that calls a JS callback from Rust.
/// This demonstrates how to use `ThreadsafeFunction` to call back into JS.
#[napi]
pub async fn hello_with_callback(js_hello_cb: JsCallback) -> napi::Result<()> {
    let (tx, rx) = mpsc::channel();

    js_hello_cb.call_with_return_value(
        FnArgs::from(("🦀 < `Hello, {{template}}!`".to_string(),)),
        ThreadsafeFunctionCallMode::Blocking,
        move |result, _| {
            tx.send(result).ok();
            Ok(())
        },
    );

    let result = rx.recv().unwrap()?;
    println!("{result}");

    Ok(())
}
