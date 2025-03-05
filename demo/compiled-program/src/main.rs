#![no_std]
#![no_main]

use panic_halt as _;

fn add(a: i32, b: i32) -> i32 {
    a + b
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn _start() -> ! {
    let a = 1;
    let b = 2;
    let _c = add(a, b);
    
    // Since we don't have `println!`, we would typically write to a UART for debugging.
    
    loop {} // Infinite loop to prevent execution from continuing randomly
}
