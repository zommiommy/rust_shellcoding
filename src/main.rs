
#![no_std]
#![no_main]
#![feature(core_intrinsics)]

/// Handle the panics, by default abort
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    core::intrinsics::abort()
}

// simple macro to call the write syscall with an integer
macro_rules! print {
    ($value:expr) => {
        unsafe{
            core::arch::asm!(
                "push {input}",  // put the value to print on stack 
                "mov rdx, 8",   // len = 8
                "mov rsi, rsp", //char *buf = rsp
                "mov rdi, 1",   // fd = stdout
                "mov rax, 1",   // write
                "syscall",

                input = in(reg) $value,
                out("rdx") _,
                out("rsi") _,
                out("rdi") _,
                out("rax") _,
            );
        }
    };
}

// safe wrapper, this might fails only if we are not on x86_64 
pub fn rdtsc() -> u64 {
    unsafe{core::arch::x86_64::_rdtsc()}
}

const N: u64 = 100;

// our shellcode
#[no_mangle]
pub fn _start() {
    let mut buffer = [0; 1024];
    let mut sum = 0;
    for i in 0..N {
        let start = rdtsc();
        buffer[i as usize] = 0;
        let end = rdtsc();
        sum += end - start;
    }
    print!(sum / N);
}