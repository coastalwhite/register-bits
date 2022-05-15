use immediate::*;

#[test]
fn concat() {
    let imm32 = Immediate32::new(0x1234_5432);

    let imm16: Immediate32<16> = imm32.take_low();
    let imm4: Immediate32<4> = imm32.take_high();

    let imm20 = imm4.concat(imm16);

    assert_eq!(imm20, 0x1_5432);
}