use types::{c_void, RawFd, Value};

extern "C" {
    pub fn rb_thread_create(function: extern "C" fn(*mut c_void) -> Value,
                            data: *mut c_void)
                            -> Value;
    pub fn rb_thread_wait_fd(fd: RawFd);
}
