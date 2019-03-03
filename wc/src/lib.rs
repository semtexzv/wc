#![feature(box_syntax)]

use console_error_panic_hook::set_once as set_panic_hook;
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
    fn new() -> Self;
    fn element_name() -> &'static str;
}



#[wasm_bindgen(module = "../../wc/js/custom_elem.js")]
extern {
    type CustomElement;

    #[wasm_bindgen(static_method_of = CustomElement)]
    pub fn meta() -> js_sys::Object;

    #[wasm_bindgen(static_method_of = CustomElement)]
    fn define(n : &str, v : js_sys::Object);


    #[wasm_bindgen(static_method_of = CustomElement)]
    fn x(o: JsValue) -> js_sys::Object;
}



#[wasm_bindgen(extends = CustomElement)]
struct TestElem;

#[wasm_bindgen]
impl TestElem{
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        TestElem
    }
}

impl Component for TestElem {
    fn new() -> Self {
        Self
    }

    fn element_name() -> &'static str {
        "custom-element"
    }
}

fn define() -> Result<(), JsValue> {
    let window = window().unwrap();
    let elem = CustomElement::meta();
    let cc = TestElem::new();
    let j : JsValue = cc.into();
    Ok(CustomElement::define(TestElem::element_name(),CustomElement::x(j)))
}

// Called by our JS entry point to run the example.
#[wasm_bindgen]
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();

    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    console_web::log!("Defining");
    define().expect("aaa");
    console_web::log!("Defined");
    let p: web_sys::Node = document.create_element("custom-element")?.into();
    p.set_text_content(Some("Hello from Rust, WebAsseambly, and Webpack!"));

    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;


    Ok(())
}