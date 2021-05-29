use wasm_bindgen::prelude::*;

use alloc::string::String;
use crate::alloc::string::ToString;

/// A newtype that represents Serde errors as JavaScript exceptions.
#[derive(Debug)]
pub struct Error(JsValue);


impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        #[wasm_bindgen]
        extern "C" {
            #[wasm_bindgen(js_name = String)]
            pub fn to_string(value: &JsValue) -> String;
        }

        to_string(&self.0).fmt(f)
    }
}

//impl std::error::Error for Error {}


impl Error {
    /// Creates a JavaScript `Error` with a given message.
    pub fn new<T: core::fmt::Display>(msg: T) -> Self {
        Error(js_sys::Error::new(&msg.to_string()).into())
    }
}

impl serde::ser::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error::new(msg)
    }
}

impl serde::de::Error for Error {
    fn custom<T: core::fmt::Display>(msg: T) -> Self {
        Error::new(msg)
    }
}

/// This conversion is needed for `?` to just work when using wasm-bindgen
/// imports that return JavaScript exceptions as `Result<T, JsValue>`.
impl From<JsValue> for Error {
    fn from(error: JsValue) -> Error {
        Error(error)
    }
}

// This conversion is needed for `?` to just work in wasm-bindgen exports
// that return `Result<T, JsValue>` to throw JavaScript exceptions.
impl From<Error> for JsValue {
    fn from(error: Error) -> JsValue {
        error.0
    }
}
