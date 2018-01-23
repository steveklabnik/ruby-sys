extern crate ruby_sys;
use ruby_sys::{vm,rproc, value, fixnum};

#[test]
fn rb_proc_new_works_with_one_argument() {
    unsafe { vm::ruby_init() };

    let nil = value::Value { value: value::RubySpecialConsts::Nil as usize };

    extern fn test_fn(v: value::Value) -> value::Value {
        return v;
    }

    unsafe {
        let val = fixnum::rb_int2inum(3);
        let rproc = rproc::rb_proc_new(test_fn, nil);
        let result = rproc::rb_proc_call_with_block(rproc, 1, vec![val].as_ptr(), nil);

        assert!(!result.is_nil());
        let val_result = fixnum::rb_num2int(result);
        assert!(3 == val_result);
    }

    unsafe { vm::ruby_cleanup(0) };
}
