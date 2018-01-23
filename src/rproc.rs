use types::{Argc, Value};

extern "C" {
    pub fn rb_proc_call_with_block(rproc: Value,
                                   argc: Argc,
                                   argv: *const Value,
                                   pass_procval: Value)
                                   -> Value;

    pub fn rb_proc_new(func: extern fn(Value) -> Value, binding: Value) -> Value;
}
