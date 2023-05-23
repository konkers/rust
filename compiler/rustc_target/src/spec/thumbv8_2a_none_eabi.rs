// Targets the Cortex-A55 processor

use crate::spec::{Target, TargetOptions};

pub fn target() -> Target {
    Target {
        llvm_target: "thumbv8.2a-none-eabi".into(),
        pointer_width: 32,
        // See: https://llvm.org/docs/LangRef.html#data-layout
        // e - little endian
        // m:e - elf mangling
        // p:32:32 - 32 bit pointers, 32 bit alignment
        // Fi8 - Function pointers independently aligned at 8 (bits?).
        // i64:64 - 64 bit ints are 64 bit aligned
        // v128:64:128 - 128 bit vectors are 64 bit aligned but prefer 128 bit alignment
        // a:0:32 - object of aggregate type no alignment, 32 bit alignment preferred.
        // n32 - native integer width: 32 bits
        // S64 - Natural stack alignment: 64 bits
        data_layout: "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64".into(),
        arch: "arm".into(),

        options: TargetOptions {
            abi: "eabi".into(),
            // TODO: floating point extnesion?
            max_atomic_width: Some(32),
            ..super::thumb_base::opts()
        },
    }
}
