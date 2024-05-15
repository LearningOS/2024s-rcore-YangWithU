use core::panic::PanicInfo;

#[panic_handler] // 标记 #[panic_handler] 告知编译器采用我们的实现
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}