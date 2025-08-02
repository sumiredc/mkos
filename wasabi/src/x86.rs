use core::arch::asm;

/// インラインアセンブリで HLT を呼び出すための関数
pub fn hlt() {
    unsafe { asm!("hlt") }
}

pub fn write_io_port_u8(port: u16, data: u8) {
    unsafe {
        asm!("out dx, al",
            in("al") data,
            in("dx") port)
    }
}
