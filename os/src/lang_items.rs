//! The panic handler

use crate::sbi::shutdown;
use core::panic::PanicInfo;

// #[panic_handler]
/// panic handler
// fn panic(info: &PanicInfo) -> ! {
//     if let Some(location) = info.location() {
//         println!(
//             "[kernel] Panicked at {}:{} {}",
//             location.file(),
//             location.line(),
//             info.message().unwrap()
//         );
//     } else {
//         println!("[kernel] Panicked: {}", info.message().unwrap());
//     }
//     shutdown()
// }


#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("Panicked: {}", info.message().unwrap());
    }
    shutdown()
}