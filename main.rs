#![feature(never_type)]
#![feature(error_in_core)]
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

    pub struct Errno(pub i32);

    impl<T: core::error::Error> From<T> for Errno {
        fn from(_value: T) -> Self {
            Self(-1)
        }
    }

    pub type Result<T, E = Errno> = core::result::Result<T, E>;

    pub fn check_error(res: i32) -> Result<usize, Errno> {
        if res >= 0 {
            Ok(res as usize)
        } else {
            Err(Errno(-res))
        }
    }

    /// Writes the text slice to stdout
    pub fn write(s: &str) -> Result<usize, Errno> {
        write_u8(s.as_bytes())
    }

    /// Same as `write` but for arbitrary data
    pub fn write_u8(buf: &[u8]) -> Result<usize, Errno> {
        check_error(unsafe { crate::syscalls::write(STDOUT, buf.as_ptr() as *const _, buf.len()) })
    }

    pub fn read_u8(buf: &mut [u8]) -> Result<usize, Errno> {
        check_error(unsafe { crate::syscalls::read(STDIN, buf.as_ptr() as *mut _, buf.len()) })
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

mod stdlib_stuff {
    pub struct ExitCode(pub i32);

    impl ExitCode {
        pub const SUCCESS: ExitCode = ExitCode(0);
        pub const FAILURE: ExitCode = ExitCode(1);
    }

    pub trait Termination {
        fn report(self) -> ExitCode;
    }

    impl Termination for ! {
        fn report(self) -> ExitCode {
            match self {}
        }
    }

    impl Termination for ExitCode {
        fn report(self) -> ExitCode {
            self
        }
    }

    impl Termination for () {
        fn report(self) -> ExitCode {
            ExitCode::SUCCESS
        }
    }

    // impl<T: Termination, E> Termination for Result<T, E> {
    //     fn report(self) -> ExitCode {
    //         match self {
    //             Ok(v) => v.report(),
    //             Err(_) => ExitCode::FAILURE,
    //         }
    //     }
    // }

    impl<T> Termination for Result<T, crate::utils::Errno> {
        fn report(self) -> ExitCode {
            match self {
                Ok(v) => ExitCode::SUCCESS,
                Err(crate::utils::Errno(err)) => ExitCode(err),
            }
        }
    }
}

#[no_mangle]
fn _start() -> ! {
    let result = main();
    let exit_code = stdlib_stuff::Termination::report(result).0;
    unsafe { syscalls::exit(exit_code) }
}

#[inline(always)]
fn main() -> utils::Result<()> {
    utils::write("Hello world")?;
    Ok(())
}
