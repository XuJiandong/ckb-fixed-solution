#![cfg_attr(not(any(feature = "native-simulator", test)), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::default_alloc!();

use ckb_fixed::I64F64;
pub fn program_entry() -> i8 {
    ckb_std::debug!("This is an example for ckb-fixed");
    let a = I64F64::from_num(1024).unwrap();
    let b = a.log2().unwrap();
    assert_eq!(b, I64F64::from_num(10).unwrap());
    0
}
