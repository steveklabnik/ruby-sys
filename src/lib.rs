extern crate libc;

pub mod array;
pub mod constant;
pub mod class;
pub mod fixnum;
pub mod types;
pub mod hash;
pub mod rproc;
pub mod string;
pub mod symbol;
pub mod util;
pub mod value;
pub mod vm;

use types::Value;

extern "C" {
    pub static rb_cObject: Value;
}
