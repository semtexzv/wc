#![feature(prelude_import)]
#![no_std]
#![feature(box_syntax)]
#[prelude_import]
use ::std::prelude::v1::*;
#[macro_use]
extern crate std as std;

use console_error_panic_hook::set_once as set_panic_hook;
use web_sys::{window, Window, CustomElementRegistry};
use js_sys::Function;

use wasm_bindgen::{prelude::*, JsValue, closure::Closure, JsCast};

trait Component {
    fn element_name()
    -> &'static str;
}

fn define<C: Component>() -> Result<(), JsValue> {
    let window = window().unwrap();
    let elem = CustomElement::meta();
    Ok(CustomElement::def(C::element_name(), elem))

}

#[allow(bad_style)]
#[doc = ""]
#[repr(transparent)]
#[allow(clippy::all)]
struct CustomElement {
    obj: ::wasm_bindgen::JsValue,
}
#[allow(bad_style)]
#[allow(clippy::all)]
const __wbg_generated_const_CustomElement: () =
    {
        use wasm_bindgen::convert::{IntoWasmAbi, FromWasmAbi, Stack};
        use wasm_bindgen::convert::{OptionIntoWasmAbi, OptionFromWasmAbi};
        use wasm_bindgen::convert::RefFromWasmAbi;
        use wasm_bindgen::describe::WasmDescribe;
        use wasm_bindgen::{JsValue, JsCast};
        use wasm_bindgen::__rt::core;
        impl WasmDescribe for CustomElement {
            fn describe() { JsValue::describe(); }
        }
        impl IntoWasmAbi for CustomElement {
            type
            Abi
            =
            <JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self, extra: &mut Stack) -> Self::Abi {
                self.obj.into_abi(extra)
            }
        }
        impl OptionIntoWasmAbi for CustomElement {
            #[inline]
            fn none() -> Self::Abi { 0 }
        }
        impl <'a> OptionIntoWasmAbi for &'a CustomElement {
            #[inline]
            fn none() -> Self::Abi { 0 }
        }
        impl FromWasmAbi for CustomElement {
            type
            Abi
            =
            <JsValue as FromWasmAbi>::Abi;
            #[inline]
            unsafe fn from_abi(js: Self::Abi, extra: &mut Stack) -> Self {
                CustomElement{obj: JsValue::from_abi(js, extra),}
            }
        }
        impl OptionFromWasmAbi for CustomElement {
            #[inline]
            fn is_none(abi: &Self::Abi) -> bool { *abi == 0 }
        }
        impl <'a> IntoWasmAbi for &'a CustomElement {
            type
            Abi
            =
            <&'a JsValue as IntoWasmAbi>::Abi;
            #[inline]
            fn into_abi(self, extra: &mut Stack) -> Self::Abi {
                (&self.obj).into_abi(extra)
            }
        }
        impl RefFromWasmAbi for CustomElement {
            type
            Abi
            =
            <JsValue as RefFromWasmAbi>::Abi;
            type
            Anchor
            =
            core::mem::ManuallyDrop<CustomElement>;
            #[inline]
            unsafe fn ref_from_abi(js: Self::Abi, extra: &mut Stack)
             -> Self::Anchor {
                let tmp =
                    <JsValue as RefFromWasmAbi>::ref_from_abi(js, extra);
                core::mem::ManuallyDrop::new(CustomElement{obj:
                                                               core::mem::ManuallyDrop::into_inner(tmp),})
            }
        }
        impl From<JsValue> for CustomElement {
            #[inline]
            fn from(obj: JsValue) -> CustomElement { CustomElement{obj,} }
        }
        impl AsRef<JsValue> for CustomElement {
            #[inline]
            fn as_ref(&self) -> &JsValue { &self.obj }
        }
        impl From<CustomElement> for JsValue {
            #[inline]
            fn from(obj: CustomElement) -> JsValue { obj.obj }
        }
        impl JsCast for CustomElement {
            #[cfg(not(all(target_arch = "wasm32",
                          not(target_os = "emscripten"))))]
            fn instanceof(val: &JsValue) -> bool {
                drop(val);








                // Called by our JS entry point to run the example.




                {
                    ::std::rt::begin_panic("cannot check instanceof on non-wasm targets",
                                           &("wc/src/lib.rs", 26u32, 1u32))
                };
            }
            #[inline]
            fn unchecked_from_js(val: JsValue) -> Self {
                CustomElement{obj: val,}
            }
            #[inline]
            fn unchecked_from_js_ref(val: &JsValue) -> &Self {
                unsafe { &*(val as *const JsValue as *const CustomElement) }
            }
        }
        ()
    };
#[allow(clippy::all)]
impl core::ops::Deref for CustomElement {
    type
    Target
    =
    JsValue;
    #[inline]
    fn deref(&self) -> &JsValue { self.as_ref() }
}
impl CustomElement {
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    #[doc = ""]
    #[allow(clippy::all)]
    pub fn meta() -> js_sys::Object {
        {
            ::std::rt::begin_panic("cannot call wasm-bindgen imported functions on non-wasm targets",
                                   &("wc/src/lib.rs", 26u32, 1u32))
        };
    }
}
impl CustomElement {
    #[allow(bad_style, unused_variables)]
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    #[doc = ""]
    #[allow(clippy::all)]
    fn def(n: &str, v: js_sys::Object) {
        {
            ::std::rt::begin_panic("cannot call wasm-bindgen imported functions on non-wasm targets",
                                   &("wc/src/lib.rs", 26u32, 1u32))
        };
    }
}
struct TestElem;
#[allow(clippy::all)]
impl ::wasm_bindgen::describe::WasmDescribe for TestElem {
    fn describe() {
        use wasm_bindgen::__wbindgen_if_not_std;
        use wasm_bindgen::describe::*;
        inform(RUST_STRUCT);
        inform(8u32);
        inform(84u32);
        inform(101u32);
        inform(115u32);
        inform(116u32);
        inform(69u32);
        inform(108u32);
        inform(101u32);
        inform(109u32);
    }
}
#[allow(clippy::all)]
impl ::wasm_bindgen::convert::IntoWasmAbi for TestElem {
    type
    Abi
    =
    u32;
    fn into_abi(self, _extra: &mut ::wasm_bindgen::convert::Stack) -> u32 {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::WasmRefCell;
        Box::into_raw(Box::new(WasmRefCell::new(self))) as u32
    }
}
#[allow(clippy::all)]
impl ::wasm_bindgen::convert::FromWasmAbi for TestElem {
    type
    Abi
    =
    u32;
    unsafe fn from_abi(js: u32, _extra: &mut ::wasm_bindgen::convert::Stack)
     -> Self {
        use wasm_bindgen::__rt::std::boxed::Box;
        use wasm_bindgen::__rt::{assert_not_null, WasmRefCell};
        let ptr = js as *mut WasmRefCell<TestElem>;
        assert_not_null(ptr);
        let js = Box::from_raw(ptr);
        (*js).borrow_mut();
        js.into_inner()
    }
}
#[allow(clippy::all)]
impl ::wasm_bindgen::__rt::core::convert::From<TestElem> for
 ::wasm_bindgen::JsValue {
    #[cfg(not(all(target_arch = "wasm32", not(target_os = "emscripten"))))]
    fn from(_value: TestElem) -> Self {
        {
            ::std::rt::begin_panic("cannot convert to JsValue outside of the wasm target",
                                   &("wc/src/lib.rs", 39u32, 1u32))
        }
    }
}
#[allow(clippy::all)]
impl ::wasm_bindgen::convert::RefFromWasmAbi for TestElem {
    type
    Abi
    =
    u32;
    type
    Anchor
    =
    ::wasm_bindgen::__rt::Ref<'static, TestElem>;
    unsafe fn ref_from_abi(js: Self::Abi,
                           _extra: &mut ::wasm_bindgen::convert::Stack)
     -> Self::Anchor {
        let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<TestElem>;
        ::wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow()
    }
}
#[allow(clippy::all)]
impl ::wasm_bindgen::convert::RefMutFromWasmAbi for TestElem {
    type
    Abi
    =
    u32;
    type
    Anchor
    =
    ::wasm_bindgen::__rt::RefMut<'static, TestElem>;
    unsafe fn ref_mut_from_abi(js: Self::Abi,
                               _extra: &mut ::wasm_bindgen::convert::Stack)
     -> Self::Anchor {
        let js = js as *mut ::wasm_bindgen::__rt::WasmRefCell<TestElem>;
        ::wasm_bindgen::__rt::assert_not_null(js);
        (*js).borrow_mut()
    }
}
impl TestElem {
    pub fn new() -> Self { TestElem }
}
impl Component for TestElem {
    fn element_name() -> &'static str { "custom-element" }
}
pub fn run() -> Result<(), JsValue> {
    set_panic_hook();
    let window = web_sys::window().expect("should have a Window");
    let document = window.document().expect("should have a Document");
    let mut arr = ::console_web::_js_sys::Array::new();
    arr.push(&"Defining".into());
    ::console_web::_web_sys::console::log(&arr);
    define::<TestElem>().expect("aaa");
    let mut arr = ::console_web::_js_sys::Array::new();
    arr.push(&"Defined".into());
    ::console_web::_web_sys::console::log(&arr);
    let p: web_sys::Node = document.create_element("custom-element")?.into();
    p.set_text_content(Some("Hello from Rust, WebAsseambly, and Webpack!"));
    let body = document.body().expect("should have a body");
    let body: &web_sys::Node = body.as_ref();
    body.append_child(&p)?;
    Ok(())
}
