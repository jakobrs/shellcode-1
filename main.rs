#![feature(start)]
#![feature(strict_provenance)]
#![no_std]
#![no_main]

#[cfg(include_panic_handler)]
use core::panic::PanicInfo;

mod ctypes {
    #![allow(non_camel_case_types)]
    pub type int = i32;
    pub type char = u8;
    pub type size_t = usize;
}

mod syscalls {
    use crate::ctypes::*;
    use core::arch::asm;

    pub unsafe fn write(fd: int, data: *const char, count: size_t) -> int {
        let ret;
        asm!(
            "syscall",
            in("rdi") fd,
            in("rsi") data,
            in("rdx") count,
            in("rax") 1,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
        ret
    }

    pub unsafe fn exit(code: int) -> ! {
        asm!(
            "syscall",
            in("rax") 60,
            in("rdi") code,
            options(noreturn, nostack)
        );
    }
}

mod utils {
    pub const STDOUT: crate::ctypes::int = 1;

    pub fn write(text: &str) {
        unsafe {
            crate::syscalls::write(STDOUT, text.as_ptr() as *const _, text.len());
        }
    }
}

#[panic_handler]
#[cfg(include_panic_handler)]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
fn print_number(mut n: u32) {
    if n == 0 {
        unsafe {
            syscalls::write(utils::STDOUT, b"0" as *const _, 1);
        }
        return;
    }

    const SIZE: usize = 10;

    let mut buf = [0u8; SIZE];
    let mut length = 0;

    while n != 0 {
        buf[SIZE - length - 1] = b'0' + (n % 10) as u8;
        n /= 10;
        length += 1;
    }

    unsafe {
        syscalls::write(
            utils::STDOUT,
            buf.as_ptr().map_addr(|addr| addr + SIZE - length as usize),
            length,
        );
    }
}

#[no_mangle]
fn _start() -> ! {
    utils::write("Hello world!\n");
    for i in 0..10 {
        utils::write("Next number: ");
        print_number(i);
        utils::write("\n");
    }
    unsafe {
        syscalls::exit(0);
    }
}
