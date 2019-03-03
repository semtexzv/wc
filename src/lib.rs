#![feature(box_syntax, unboxed_closures)]

use web_sys::{
    window, Window, CustomElementRegistry,
};
use js_sys::Function;
use wasm_bindgen::{
    prelude::*,
    JsValue, closure::Closure,
    JsCast,
};

trait Component {
    fn element_name() -> &'static str;
}


fn define<C: Component>() -> Result<(), JsValue> {
    let window = window().unwrap();

    let registry = window.custom_elements();
    let constructor = Closure::wrap((box move || {}) as Box<FnMut()>);
    Ok(registry.define(C::element_name(), constructor.as_ref().unchecked_ref())?)
}


// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    //set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");

    let p: web_sys::Node = document.create_element("p")?.into();
    p.set_text_content(Some("Hello from Rust, WebAssembly, and Webpack!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;


    Ok(())
}