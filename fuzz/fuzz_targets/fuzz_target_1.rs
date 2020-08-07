#![no_main]
use libfuzzer_sys::fuzz_target;
extern crate playground_rocket;


fuzz_target!(|data: &[u8]| {
    // fuzzed code goes here
});
