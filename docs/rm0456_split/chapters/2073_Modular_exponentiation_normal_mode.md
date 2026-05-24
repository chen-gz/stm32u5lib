Once this operation is started PKA control register and PKA memory is no more available.
Access is restored once BUSY bit is set to 0 by the PKA.
RM0456 Rev 6

RM0456

Public key accelerator (PKA)
When this operation completes with errors due to unexpected hardware events a PKA
tamper event is triggered to TAMP peripheral, and access to PKA RAM becomes blocked
until erased by hardware.

Note:

When MODE = 0x03, the error code is not affected when PKA automatically clears the PKA
RAM at the end of this protected operation. When the error output equals 0xD60D, the
result output is not affected either.
Operation instructions for modular exponentiation are summarized in Table 500 (normal
mode), Table 501 (fast mode) and in Table 502 (protected mode). Fast mode usage is
explained in Section 53.3.6.
Table 500. Modular exponentiation (normal mode)
Parameters with direction
IN
IN
IN/OUT
IN
OUT

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x00

Exponent length

(in bits, not null)

RAM@0x400

Operand length

(in bits, not null)

RAM@0x408

Operand A (base of
(0 ≤ A < n)
exponentiation)

RAM@0xC68

Exponent e

(0 ≤ e < n)

RAM@0xE78

Modulus value n

(odd integer only, n < 24160)

RAM@0x1088

Result: Ae mod n

(0 ≤ result < n)

RAM@0x838

64 bits

ROS

Table 501. Modular exponentiation (fast mode)
Parameters with direction
IN
IN
IN/OUT
IN

Value (note)

Storage

Size

PKA_CR

6 bits

MODE

0x02

Exponent length

(in bits, not null)

RAM@0x400

Operand length

(in bits, not null)

RAM@0x408

Operand A (base of
exponentiation)

(0 ≤ A < n)

RAM@0xC68

Exponent e

(0 ≤ e < n)

RAM@0xE78
4160)

RAM@0x1088

Modulus value n

(odd integer only, n < 2

IN/OUT

Montgomery
parameter R2 mod n

(mandatory)

RAM@0x620

OUT

Result: Ae mod n

(0 ≤ result < n)

RAM@0x838

RM0456 Rev 6

64 bits

ROS

<!-- pagebreak -->

