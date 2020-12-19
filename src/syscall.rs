
pub unsafe fn syscall(usize: SyscallNumber) -> Result<i32> {
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