use libc::size_t;
use std::mem;
use types::{c_long, InternalValue, RBasic, Value};

extern "C" {
    pub fn rb_ary_entry(array: Value, offset: c_long) -> Value;
    pub fn rb_ary_join(array: Value, separator: Value) -> Value;
    pub fn rb_ary_new() -> Value;
    pub fn rb_ary_push(array: Value, item: Value) -> Value;
    pub fn rb_ary_store(array: Value, index: c_long, item: Value) -> Value;
}

const FL_USHIFT: isize = 12;
const FL_USER_1: isize = 1 << (FL_USHIFT + 1);
const FL_USER_3: isize = 1 << (FL_USHIFT + 3);
const FL_USER_4: isize = 1 << (FL_USHIFT + 4);

#[repr(C)]
enum RArrayEmbed {
    LenMask = FL_USER_4 | FL_USER_3,
    Flag = FL_USER_1,
    LenShift = FL_USHIFT + 3,
}

#[repr(C)]
struct RArrayAs {
    heap: RArrayHeap,
}

#[repr(C)]
struct RArrayHeap {
    len: c_long,
    // Really, this is a union but value is the largest item.
    value: InternalValue,
    ptr: InternalValue,
}

#[repr(C)]
struct RArray {
    basic: RBasic,
    as_: RArrayAs,
}

pub fn rb_ary_len(value: Value) -> c_long {
    unsafe {
        let basic: *const RBasic = mem::transmute(value.value);
        let flags = (*basic).flags;
        if flags & (RArrayEmbed::Flag as size_t) == 0 {
            let array: *const RArray = mem::transmute(value.value);
            (*array).as_.heap.len
        } else {
            ((flags as i64 >> RArrayEmbed::LenShift as i64) &
             (RArrayEmbed::LenMask as i64 >> RArrayEmbed::LenShift as i64)) as c_long
        }
    }
}
