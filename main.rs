#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod ctypes {
    //! Type aliases for C types

    #![allow(non_camel_case_types)]
    pub type int = i32;
    pub type char = u8;
    pub type size_t = usize;
}

mod syscalls {
    //! Direct syscalls

    use crate::ctypes::*;
    use core::arch::asm;

    pub unsafe fn read(fd: int, data: *mut char, count: size_t) -> int {
        let ret;
        asm!(
            "syscall",
            in("rdi") fd,
            in("rsi") data,
            in("rdx") count,
            in("rax") 0,
            lateout("rax") ret,
            lateout("rcx") _,
            lateout("r11") _,
            options(nostack)
        );
        ret
    }

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
    //! Convenience functions

    pub const STDIN: crate::ctypes::int = 0;
    pub const STDOUT: crate::ctypes::int = 1;

    /// Writes the text slice to stdout
    pub fn write(s: &str) {
        write_u8(s.as_bytes());
    }

    /// Same as `write` but for arbitrary data
    pub fn write_u8(buf: &[u8]) {
        unsafe {
            crate::syscalls::write(STDOUT, buf.as_ptr() as *const _, buf.len());
        }
    }

    #[inline(always)]
    pub unsafe fn assume(cond: bool) {
        if !cond {
            core::hint::unreachable_unchecked();
        }
    }

    #[inline(always)]
    pub unsafe fn assume_in_range<T>(value: T, range: core::ops::Range<T>)
    where
        T: PartialOrd,
    {
        assume(range.contains(&value));
    }
}

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo) -> ! {
    loop {}
}

#[inline(never)]
fn print_number(mut n: u32) {
    if n == 0 {
        utils::write("0");
        return;
    }

    const SIZE: usize = 10;

    let mut buf = [0u8; SIZE];
    let mut length = 0;

    while n != 0 {
        unsafe { utils::assume_in_range(SIZE - length - 1, 0..SIZE) }
        buf[SIZE - length - 1] = b'0' + (n % 10) as u8;
        n /= 10;
        length += 1;
    }

    unsafe { utils::assume_in_range(SIZE - length - 1, 0..SIZE) }
    utils::write_u8(&buf[SIZE - length - 1..]);
}

#[no_mangle]
fn _start() -> ! {
    main();
    unsafe { syscalls::exit(0) }
}

fn main() {
    use core::str::FromStr;

    unsafe {
        utils::write("Number-tripling tool\n");
        utils::write("Enter your number: ");

        let mut buf = [0u8; 16];
        let i = syscalls::read(5, &mut buf as *mut _, buf.len()) as usize;

        utils::assume_in_range(i - 1, 0..buf.len());
        let n: i32 =
            i32::from_str(core::str::from_utf8_unchecked(&buf[..i - 1])).unwrap_unchecked();

        utils::write("The result is: ");
        print_number((n * 3) as u32);
    }
}
