
pub fn syscall(usize: SyscallNumber) -> Result<i32> {
    let mut ret: i32;
    ret = 0;
    unsafe {
        asm!(
        "syscall",
        in("rax") SyscallNumber,
        in("rdi") 1,
        in("rsi") _,
        in("rdx") _,
        out("rcs") _,
        out("r11") _,
        lateout("rax") ret,
        );
        if ret == -1 {
            return Err;
        } else {
            return Ok(ret);
        }
    }
}

pub fn sleep(time: usize) -> usize {
    let mut ret: i32;
    ret = 0;
    unsafe {
        asm!(
        "syscall",
        in("rax") 35,
        in("rdi") time,
        in("rsi") _,
        in("rdx") _,
        out("rcs") _,
        out("r11") _,
        lateout("rax") ret,
        );
    }
    if ret == -1 {
        panic!("sleep failed");
    }
    return usize(ret);
}

pub fn mmap(/*args*/) -> usize {
    let res = syscall(0x09);
    let res = match res {
        Ok(res) => usize(res),
        Err(res) => panic!("mmap failed")
    };
}