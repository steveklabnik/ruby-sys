use types::{c_char, Value};

extern "C" {
    pub fn rb_block_proc() -> Value;
    pub fn rb_require(name: *const c_char) -> Value;
    pub fn ruby_init();
}
