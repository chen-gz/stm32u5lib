2093

Public key accelerator (PKA)
1.

Inward (or outward) conversion into (or from) Montgomery domain
a)

b)

2.

3.

RM0456

Assuming that A is an integer in the natural domain:
–

Compute r2modn using Montgomery parameter computation.

–

Result AR= A x r2modn mod n is A in the Montgomery domain.

Assuming that BR is an integer in the Montgomery domain:
–

Result B = BR x 1 mod n is B in the natural domain.

–

Similarly, above value AR computed in a) can be converted into the natural
domain by computing A = AR x 1 mod n.

Simple modular multiplication A x B mod n
a)

Compute r2modn using Montgomery parameter computation.

b)

Compute AR = A x r2modn mod n. Output is in the Montgomery domain.

c)

Compute AB= AR x B mod n. Output is in natural domain.

Multiple modular multiplication A x B x C mod n
a)

Compute r2modn using Montgomery parameter computation.

b)

Compute AR = A x r2modn mod n. Output is in the Montgomery domain.

c)

Compute BR = B x r2modn mod n. Output is in the Montgomery domain.

d)

Compute ABR= AR x BR mod n. Output is in the Montgomery domain.

e)

Compute CR = C x r2modn mod n. Output is in the Montgomery domain.

f)

Compute ABCR= ABR x CR mod n. Output is in the Montgomery domain.

g)

(optional) Repeat the two steps above if more operands need to be multiplied.

h)

Compute ABC= ABCR x 1 mod n to retrieve the result in natural domain.

Operation instructions for Montgomery multiplication are summarized in Table 499.
Table 499. Montgomery multiplication
Parameters with direction

IN

Storage

Size

PKA_CR

6 bits
64 bits

MODE

0x10

Operand length

(in bits, not null)

RAM@0x408

Operand A

(0 ≤ A < n)

RAM@0xA50

Operand B

(0 ≤ B < n)

Modulus value n
OUT

Value (note)

RAM@0xC68
4160

(odd integer only, n < 2

Result: AxB mod n(1)

-

)

RAM@0x1088

ROS

RAM@0xE78

1. Result in Montgomery domain or in natural domain, depending upon the inputs nature (see examples 2 and
3).

53.4.6

Modular exponentiation
Modular exponentiation operation is commonly used to perform a single-step RSA
operation. It consists in the computation of Ae mod n.
RSA operation involving public information (RSA encryption) can use the normal or fast
mode detailed on Table 500 and Table 501. RSA operation involving secret information
(RSA decryption) must use the protected mode detailed on Table 502, for security reason.

Note:

<!-- pagebreak -->

