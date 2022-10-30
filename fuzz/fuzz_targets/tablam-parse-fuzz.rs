#![no_main]
use libfuzzer_sys::fuzz_target;
use std::str;
use tablam_parser::parser::Parser;

fuzz_target!(|data: &[u8]| {
    match str::from_utf8(data) {
        Ok(in_string)=>{
            Parser::from_src(in_string).parse();
        },
        Err(..)=>()
    }
});