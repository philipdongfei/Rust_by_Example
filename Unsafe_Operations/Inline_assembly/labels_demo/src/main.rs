use std::arch::asm;


fn main() {
    let mut a = 0;
    unsafe {
        asm!(
            "mov {0}, 10",
            "2:",
            "sub {0}, 1",
            "cmp {0}, 3",
            "jle 2f",
            "jmp 2b",
            "2:",
            "add {0}, 2", 
            out(reg) a
        );
    }
    assert_eq!(a, 5);
}
