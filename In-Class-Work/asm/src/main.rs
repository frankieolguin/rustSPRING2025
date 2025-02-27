use std::arch::asm;

fn main() {
    let message = b"Hello, direct syscall!\n";

    unsafe {
        // write syscall
        asm!(
            // %rax = 1 for sys_write %rdi %rsi %rdx for sys_write
            "mov rax, 1",  // syscall number for write
            "mov rdi, 1",  // file descriptor: 1 is stdout
            "syscall",
            in("rsi") message.as_ptr(),
            in("rdx") message.len(), // rdx needs number of bytes
            out("rax") _, //
            out("rcx") _, //
            out("r11") _, //
            clobber_abi("system") //
        );

        // exit syscall
        asm!(
            "mov rax, 60", // syscall number for exit sys_exit rax value
            "xor rdi, rdi", // status code 0
            "syscall",
            options(noreturn)
        );
    }
}
