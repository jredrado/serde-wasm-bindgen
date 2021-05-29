#![feature(core_intrinsics,lang_items,start,default_alloc_error_handler,custom_test_frameworks)]
#![feature(min_type_alias_impl_trait)]
#![no_std]
#![no_main]

#![cfg_attr(feature = "external_doc", feature(external_doc))]
#![cfg_attr(feature = "external_doc", doc(include = "../README.md"))]
#![cfg_attr(feature = "external_doc", warn(missing_docs))]

use wasm_bindgen::prelude::*;

extern crate alloc;

use alloc::collections::{BTreeMap as HashMap};

mod de;
mod error;
mod ser;

pub use de::Deserializer;
pub use error::Error;
pub use ser::Serializer;

type Result<T> = core::result::Result<T, Error>;

use core::sync::atomic::AtomicPtr;

#[derive(Default)]
struct Cache {
    pub map: core::cell::RefCell<HashMap<&'static str, JsValue>>
}

unsafe impl Sync for Cache {}

use lazy_static::lazy_static;


fn static_str_to_js(s: &'static str) -> JsValue {
 
    //Concurrency?? WASM, no threads, no atomics

   lazy_static! { 
        static ref CACHE: Cache = Cache { map: Default::default() };
   }
    
   CACHE
    .map
    .borrow_mut()
    .entry(s)
    .or_insert_with(|| JsValue::from_str(s))
    .clone()

}


/// Converts [`JsValue`] into a Rust type.
pub fn from_value<T: serde::de::DeserializeOwned>(value: JsValue) -> Result<T> {
    T::deserialize(Deserializer::from(value))
}

/// Converts a Rust value into a [`JsValue`].
pub fn to_value<T: serde::ser::Serialize>(value: &T) -> Result<JsValue> {
    value.serialize(&Serializer::new())
}
