use register_bits::prelude::*;
use register_bits::reg32::Reg32Bits as Bits;

#[test]
fn downcasting() {
    let imm32 = Bits::new(0xFABB_1234);

    let low_imm4: Bits<4> = imm32.take_low();
    let high_imm4: Bits<4> = imm32.take_high();

    assert_eq!(low_imm4, 0x4u32);
    assert_eq!(high_imm4, 0xFu32);
}

#[test]
fn upcasting() {
    let imm32 = Bits::new(0xFABB_1234);

    let low_imm4: Bits<4> = imm32.take_low();
    let imm32: Bits<32> = low_imm4.zero_extend();

    assert_eq!(low_imm4, 0x4u32);
    assert_eq!(imm32, 0x4u32);
}