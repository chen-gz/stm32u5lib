RM0456 Rev 6

ROS

RM0456

Public key accelerator (PKA)
Table 497. Modular addition
Parameters with direction

IN

OUT

53.4.4

Value (note)

Storage

Size

PKA_CR

6 bits
64 bits

MODE

0x0E

Operand length

(in bits, not null)

RAM@0x408

Operand A

(0 ≤ A < n)

RAM@0xA50

Operand B

(0 ≤ B < n)

RAM@0xC68

Modulus value n

(n < 2

4160

RAM@0x1088

Result: A+B mod n

(0 ≤ result < n)

RAM@0xE78

)

ROS

Modular subtraction
Modular subtraction operation consists in the following computations:
•

If A ≥ B result equals A - B mod n

•

If A < B result equals A + n - B mod n

Operation instructions are summarized in Table 498.
Table 498. Modular subtraction
Parameters with direction

IN

OUT

53.4.5

Value (note)

Storage

Size

PKA_CR

6 bits
64 bits

MODE

0x0F

Operand length

(in bits, not null)

RAM@0x408

Operand A

(0 ≤ A < n)

RAM@0xA50

Operand B

(0 ≤ B < n)

RAM@0xC68

Modulus value n

(n < 24160)

RAM@0x1088

Result: A-B mod n

(0 ≤ result < n)

RAM@0xE78

ROS

Modular and Montgomery multiplication
To be more efficient when performing a sequence of multiplications the PKA accelerates
multiplication which has at least one input in the Montgomery domain. The two main uses of
this operation are:
•

Map a value from natural domain to Montgomery domain and vice-versa

•

Perform a modular multiplication A x B mod n

The method to perform above operations are described below. Note that “x” function is this
operation, and A, B, C operands are in the natural domain.

RM0456 Rev 6

<!-- pagebreak -->

