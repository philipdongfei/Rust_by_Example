use std::arch::asm;


fn main() {
    let x: u64;


    unsafe {
        asm!("nop");
        //outputs
        asm!("mov {}, 5", out(reg) x);
        //
    }
    assert_eq!(x, 5);

    // inputs
    let i: u64 = 3;
    let o: u64;
    unsafe {
        asm!(
            "mov {0}, {1}",
            "add {0}, 5",
            out(reg) o,
            in(reg) i,
        );
    }
    assert_eq!(o, 8);

    // refine the above example to avoid the mov instruction.
    let mut x: u64 = 3;
    unsafe {
        asm!("add {0}, 5", inout(reg) x);
    }
    assert_eq!(x, 8);

    // specify different variables for the input and output parts of an inout operand
    let x: u64 = 3;
    let y: u64;
    unsafe {
        asm!("add {0}, 5", inout(reg) x => y);
    }
    assert_eq!(y, 8);


    // an example where inlateout cannot be used
    let mut a: u64 = 4;
    let b: u64 = 4;
    let c: u64 = 4;
    unsafe {
        asm!(
            "add {0}, {1}",
            "add {0}, {2}",
            inout(reg) a,
            in(reg) b,
            in(reg) c,
        );
    }
    assert_eq!(a, 12);

    // use inlateout
    let mut a: u64 = 4;
    let b: u64 = 4;
    unsafe {
        asm!("add {0}, {1}", inlateout(reg)a, in(reg) b);
    }
    assert_eq!(a, 8);

}
