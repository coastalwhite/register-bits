use immediate::*;
use immediate::Immediate32;

#[test]
fn downcasting() {
    let imm32 = Immediate32::new(0xFABB_1234);

    let low_imm4: Immediate32<4> = imm32.take_low();
    let high_imm4: Immediate32<4> = imm32.take_high();

    assert_eq!(low_imm4, 0x4);
    assert_eq!(high_imm4, 0xF);
}

#[test]
fn upcasting() {
    let imm32 = Immediate32::new(0xFABB_1234);

    let low_imm4: Immediate32<4> = imm32.take_low();
    let imm32: Immediate32<32> = low_imm4.zero_extend();

    assert_eq!(low_imm4, 0x4);
    assert_eq!(imm32, 0x4);
}