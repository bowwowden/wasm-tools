//! A WebAssembly encoder.
//!
//! The main builder is the [`Module`]. You can build a section with a
//! section-specific builder, like [`TypeSection`] or [`ImportSection`], and
//! then add it to the module with [`Module::section`]. When you are finished
//! building the module, call either [`Module::as_slice`] or [`Module::finish`]
//! to get the encoded bytes. The former gives a shared reference to the
//! underlying bytes as a slice, while the latter gives you ownership of them as
//! a vector.
//!
//! # Example
//!
//! If we wanted to build this module:
//!
//! ```wasm
//! (module
//!   (type (func (param i32 i32) (result i32)))
//!   (func (type 0)
//!     local.get 0
//!     local.get 1
//!     i32.add)
//!   (export "f" (func 0)))
//! ```
//!
//! then we would do this:
//!
//! ```
//! use wasm_encoder::{
//!     CodeSection, Export, ExportSection, Function, FunctionSection, Instruction,
//!     Module, TypeSection, ValType,
//! };
//!
//! let mut module = Module::new();
//!
//! // Encode the type section.
//! let mut types = TypeSection::new();
//! let params = vec![ValType::I32, ValType::I32];
//! let results = vec![ValType::I32];
//! types.function(params, results);
//! module.section(&types);
//!
//! // Encode the function section.
//! let mut functions = FunctionSection::new();
//! let type_index = 0;
//! functions.function(type_index);
//! module.section(&functions);
//!
//! // Encode the export section.
//! let mut exports = ExportSection::new();
//! exports.export("f", Export::Function(0));
//! module.section(&exports);
//!
//! // Encode the code section.
//! let mut codes = CodeSection::new();
//! let locals = vec![];
//! let mut f = Function::new(locals);
//! f.instruction(&Instruction::LocalGet(0));
//! f.instruction(&Instruction::LocalGet(1));
//! f.instruction(&Instruction::I32Add);
//! f.instruction(&Instruction::End);
//! codes.function(&f);
//! module.section(&codes);
//!
//! // Extract the encoded Wasm bytes for this module.
//! let wasm_bytes = module.finish();
//!
//! // We generated a valid Wasm module!
//! assert!(wasmparser::validate(&wasm_bytes).is_ok());
//! ```

#![deny(missing_docs, missing_debug_implementations)]

mod component;
mod core;
mod custom;
mod raw;

pub use self::component::*;
pub use self::core::*;
pub use self::custom::*;
pub use self::raw::*;
pub mod encoders;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_encodes_an_empty_module() {
        let bytes = Module::new().finish();
        assert_eq!(
            bytes,
            [0x00, 'a' as u8, 's' as u8, 'm' as u8, 0x01, 0x00, 0x00, 0x00]
        );
    }

    #[test]
    fn it_encodes_an_empty_component() {
        let bytes = Component::new().finish();
        assert_eq!(
            bytes,
            [0x00, 'a' as u8, 's' as u8, 'm' as u8, 0x0a, 0x00, 0x01, 0x00]
        );
    }
}
