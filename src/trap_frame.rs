// should be macro obv
pub unsafe fn trap_frame() {
    asm!(
    "push rax",
    "push rdi",
    "push rsi",
    "push rdx",
    "push r10",
    "push r8",
    "push r9"
    "push rcx"
    "push rdx"
    "push r11"
    );
}

//should be macro obv
pub unsafe fn release_frame() {
    asm!(
    "pop r11",
    "pop rdx",
    "pop rcx",
    "pop r9",
    "pop r8",
    "pop r10",
    "pop rdx",
    "pop rsi",
    "pop rdi",
    "pop rax"
    );
}