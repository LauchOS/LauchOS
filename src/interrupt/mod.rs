pub mod interrupt;
pub mod tss;
pub mod gdt;
pub mod pics;

pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;
pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;