2093

Public key accelerator (PKA)

RM0456

P for an ECC P256 curve (EOS = 5 words), the least significant bit must be placed in bit 0 at
address offset 0x578, and the most significant bit in bit 63 at address offset 0x590. Then, as
mentioned above, the application must write the empty word 0x0000000000000000 at
address offset 0x598.
As a second example, still to prepare the operation ECC Fp scalar multiplication, when the
application need to write the information a = -3, on a curve with a modulus length of 224 bits
(that is, four 64-bit words, rounded up, plus one) following data must be written in PKA
memory:

53.4.2

@RAM+410

0x0000000000000001 /* curve coefficient 'a' sign without extra word */

@RAM+418

0x0000000000000011 /* value of |a| LSB

@RAM+420

0x0000000000000000 ...

@RAM+428

0x0000000000000000 ...

@RAM+430

0x0000000000000000 value of |a| MSB */

@RAM+438

0x0000000000000000 /* additional all 0 word */

Montgomery parameter computation
This function is used to compute the Montgomery parameter (R2 mod n) used by PKA to
convert operands into the Montgomery residue system representation. This operation can
be very useful when fast mode operation is used, because in this case the Montgomery
parameter is passed as input, saving the time for its computation.

Note:

This operation can also be used with ECC curves. In this case prime modulus length and
EOS size must be used.
Operation instructions for Montgomery parameter computation are summarized in
Table 496.
Table 496. Montgomery parameter computation
Parameters with direction

IN

OUT

53.4.3

Value (note)

Storage

Size

PKA_CR

6 bits
64 bits

MODE

0x01

Modulus length

(in bits, 0 ≤ value < 4160)

RAM@0x408

Modulus value n

(odd integer only, n < 24160)

RAM@0x1088

-

RAM@0x620

2 mod n

Result: R

Modular addition
Modular addition operation consists in the computation of A + B mod n. Operation
instructions are summarized in Table 497.

<!-- pagebreak -->

