use register_bits::prelude::*;
use register_bits::reg32::Reg32Bits as Bits;

#[test]
#[no_mangle]
fn concat() {
    let imm32 = Bits::new(0x1234_5432);

    let imm16: Bits<16> = imm32.take_low();
    let imm4: Bits<4> = imm32.take_high();

    let imm20 = imm4.concat(imm16);

    assert_eq!(imm20, 0x1_5432u32);
}