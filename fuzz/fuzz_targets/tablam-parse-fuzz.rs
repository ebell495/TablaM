#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;
use tablam_eval::program::Program;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            Program::from_src(in_string);
        },
        Err(..)=>()
    }
});