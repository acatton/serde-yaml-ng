#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if data.len() <= 10240 {
        _ = serde_yaml_ng::from_slice::<serde_yaml_ng::Value>(data);
    }
});
