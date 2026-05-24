RM0456 Rev 6

RM0456

53.3.4

Public key accelerator (PKA)

PKA public key acceleration
Overview
Public key accelerator (PKA) is used to accelerate Rivest, Shamir and Adleman (RSA),
Diffie-Hellman (DH) as well as ECC over prime field operations. Supported operand sizes is
up to 4160 bits for RSA and DH, and up to 640 bits for ECC.
The PKA supports all non-singular elliptic curves defined over prime fields, that can be
described with a short Weierstrass equation y2 = x3 + ax + b (mod p). More information can
be found in Section 53.5.1.

Note:

Binary curves, Edwards curves and Curve25519 are not supported by the PKA.
A memory of 5336 bytes (667 words of 64 bits) called PKA RAM is used to provide initial
data to the PKA, and to hold the results after computation is completed. Access is done
though the PKA AHB interface.

PKA operating modes
The list of operations the PKA can perform is detailed in Table 494 and Table 495,
respectively, for integer arithmetic functions and prime field (Fp) elliptic curve functions.
Table 494. PKA integer arithmetic functions list
PKA_CR.MODE[5:0]
Performed operation

Reference

Hex

Binary

0x01

000001

Montgomery parameter computation R2 mod n

Section 53.4.2

0x0E

001110

Modular addition (A+B) mod n

Section 53.4.3

0x0F

001111

Modular subtraction (A-B) mod n

Section 53.4.4

0x10

010000

Montgomery multiplication (AxB) mod n

Section 53.4.5

0x00

000000

Modular exponentiation Ae mod n

0x02

000010

Modular exponentiation Ae mod n (fast mode)

0x03

000011

Modular exponentiation Ae mod n (protected)
-1 mod n

Section 53.4.6
Section 53.4.6
Section 53.4.7

0x08

001000

Modular inversion A

0x0D

001101

Modular reduction A mod n

Section 53.4.8

0x09

001001

Arithmetic addition A+B

Section 53.4.9

0x0A

001010

Arithmetic subtraction A-B

Section 53.4.10

0x0B

001011

Arithmetic multiplication AxB

Section 53.4.11

0x0C

001100

Arithmetic comparison (A=B, A>B, A<B)

Section 53.4.12

0x07

000111

RSA CRT exponentiation

Section 53.4.13

RM0456 Rev 6

<!-- pagebreak -->

