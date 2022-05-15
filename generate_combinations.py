#! /usr/bin/python3

IMPL8_NAME="impl_imm8"
IMPL16_NAME="impl_imm16"
IMPL32_NAME="impl_imm32"
IMPL64_NAME="impl_imm64"
IMPL128_NAME="impl_imm128"

leq = []
outputs = []
for i in range(1, 33):
    leq.append(str(i))

    concats = []
    for j in range(1, 33-i):
        concats.append("({}, {})".format(j, j+i));

    output = "{}!({}, [{}], [{}]);".format(
        IMPL32_NAME,
        i,
        ', '.join(leq),
        ', '.join(concats)
    )
    outputs.append(output)

print('\n'.join(outputs))