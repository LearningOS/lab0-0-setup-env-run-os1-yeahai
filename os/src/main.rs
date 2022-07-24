// os/src/main.rs
#![no_main]
#![no_std]

mod lang_items;
#[macro_use]
mod console;

core::arch::global_asm!(include_str!("entry.asm"));




const SYSCALL_WRITE: usize = 64;

pub fn sys_write(fd:usize,buffer: &[u8]) -> isize {
    syscall(SYSCALL_WRITE,[fd,buffer.as_ptr() as usize,buffer.len()])
}

const SYSCALL_EXIT: usize = 93;

fn syscall(id: usize,args:[usize;3]) -> isize {
    let mut ret;
    unsafe {
        core::arch::asm!(
            "ecall",
            inlateout("x10") args[0] => ret,
            in("x11") args[1],
            in("x12") args[2],
            in("x17") id,
        );
    }
    ret
}
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT,[xstate as usize,0,0])
}

#[no_mangle]
extern "C" fn _start(){
    print!("Hello,");
    println!("world!");
    sys_exit(9);
    
}
fn clear_bss() {
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe {(a as *mut u8).write_volatile(0)}
    })
}
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    shutdown();
}


fn main() {
    //  println!("Hello, world!");
}
