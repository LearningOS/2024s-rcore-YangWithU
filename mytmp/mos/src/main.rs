// #![no_std]
// #![no_main]

// use core::fmt::Write;

// mod lang_items;

// const SYSCALL_EXIT: usize = 93;
// const SYSCALL_WRITE: usize = 64;

// fn syscall(id: usize, args: [usize; 3]) -> isize {
//     let mut ret;
//     unsafe {
//         core::arch::asm!(
//             "ecall", //RV64I（RISC-V 64-bit base integer ISA）指令
//             inlateout("x10") args[0] => ret,//args[0]` 的值加载到 `x10` 寄存器，执行 "ecall" 指令，然后将 `x10` 寄存器的值存回 `ret` 中
//             in("x11") args[1],//args[1] 的值加载到 x11 寄存器中
//             in("x12") args[2],//args[2] 的值加载到 x12 寄存器中
//             in("x17") id,//id 的值加载到 x17 寄存器中
//         );
//     }
//     ret
// }

// pub fn sys_exit(xstate: i32) -> isize {
//     syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
// }


// // 尝试支持println!
// pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
//     syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
// }

// struct Stdout;

// impl Write for Stdout {
//     // 实现 `write_fmt` 方法时，需要先实现 `write_str` 方法
//     fn write_str(&mut self, s: &str) -> core::fmt::Result {
//         sys_write(1, s.as_bytes());
//         Ok(())
//     }
// }

// pub fn print(args: core::fmt::Arguments) {
//     Stdout.write_fmt(args).unwrap();
// }

// #[macro_export] // 这是一个属性宏标记，用于将宏导出到外部，使其可以在其他模块中使用
// macro_rules! print {
//     // $fmt: literal 表示匹配一个字符字面值（即，字符串字面量
//     // `$($arg: tt)+` 则表示一个或多个 tokens（tt）的重复模式
//     // 宏展开部分包含了对 `$crate::console::print` 函数和 `format_args!` 宏的调用，
//     // 其中使用了 `$fmt` 和可能的 `$arg` tokens 来构造一个格式化字符串并将其传递给 `print` 函数
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         $crate::console::print(format_args!($fmt $(, $(arg)+)?));
//     };
// }

// #[macro_export]
// macro_rules! println {
//     ($fmt: literal $(, $($arg: tt)+)?) => {
//         print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
//     }
// }


// #[no_mangle]
// extern "C" fn _start() {
//     println!("Hello, world!");
//     sys_exit(9);
// }


#![no_std]
#![no_main]

mod lang_items;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
