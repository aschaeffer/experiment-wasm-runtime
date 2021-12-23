#[link(wasm_import_module = "host")]
extern "C" {
    fn two_times(v: i32) -> i32;
}

#[no_mangle]
pub extern "C" fn four_times(v: i32) -> i32 {
    unsafe {
        return two_times(v) * 2;
    }
}
