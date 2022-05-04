use std::arch::asm;


fn main() {
    let mut ebx: u32;
    let mut edx: u32;
    let mut ecx: u32;

    unsafe {
        asm!(
            "push rbx",
            "cpuid",
            "mov {0:e}, ebx",
            "pop rbx",
            // String is stored as ascii in ebx, edx, ecx in order
            // Because ebx is reserved, we get a scratch register and move from
            // ebx into it in the asm. The asm needs to preserve the value of
            // that register though, so it is pushed and poped around the main asm
            // (in 64 bit mode for 64 bit processors, 32 bit processors would use ebx)
            out(reg) ebx,
            out("edx") edx,
            out("ecx") ecx,
            // EAX 0 selects CPUID parameter and manufacturer ID
            inout("eax") 0 => _,
        );
    }

    // Turn the resulting values into a string
    let mut s = String::with_capacity(12);
    ebx.to_ne_bytes().map(|b| s.push(char::from(b)));
    edx.to_ne_bytes().map(|b| s.push(char::from(b)));
    ecx.to_ne_bytes().map(|b| s.push(char::from(b)));
    println!("CPU Manufacturer ID: {}", s);

    // Multiply x by 6 using shifts and adds
    let mut x: u64 = 4;
    unsafe {
        asm!(
            "mov {tmp}, {x}",
            "shl {tmp}, 1",
            "shl {x}, 2",
            "add {x}, {tmp}",
            x = inout(reg) x,
            tmp = out(reg) _,
        );
    }
    assert_eq!(x, 4 * 6);

}
