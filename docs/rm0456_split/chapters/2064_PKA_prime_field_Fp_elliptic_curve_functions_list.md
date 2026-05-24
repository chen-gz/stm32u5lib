2093

Public key accelerator (PKA)

RM0456

Table 495. PKA prime field (Fp) elliptic curve functions list
PKA_CR.MODE[5:0]
Performed operation

Reference

Hex

Binary

0x28

101000

Point on elliptic curve Fp check

Section 53.4.14

0x20

100000

ECC scalar multiplication kP
(protected)

Section 53.4.15

0x23

100011

ECC complete addition

Section 53.4.18

0x24

100100

ECDSA sign (protected)

Section 53.4.16

0x26

100110

ECDSA verification

Section 53.4.17

0x27

100111

ECC double base ladder

Section 53.4.19

0x2F

101111

ECC projective to affine

Section 53.4.20

Each of these operating modes has an associated code that has to be written to the MODE
field in the PKA_CR register. If the application selects any value that is not documented
below the write to MODE bitfield is ignored, and an operation error (OPERRF) is triggered.
When this happens, a new operation must be selected after the error is cleared.
Some operations in Table 494 and Table 495 are indicated as protected. Those operations
are used when manipulating secret keys (modular exponentiation for RSA decryption,
scalar multiplication and signature for ECC). Those secrets (protected against side channel
attacks) and any intermediate values are automatically erased when PKA RAM is cleared at
the end of the protected operations (BUSY goes low). They are also protected against side
channel attacks.
Caution:

For security reason it is very important to select protected modular exponentiation
(MODE = 0x3) when performing RSA decryption.

Montgomery space and fast mode operations
For efficiency reason the PKA internally performs modular multiply operations in the
Montgomery domain, automatically performing inward and outward transformations.
As Montgomery parameter computation is time consuming the application can decide to use
a faster mode of operation, during which the precomputed Montgomery parameter is
supplied before starting the operation. Performance improvement is detailed in
Section 53.5.2: Computation times.
The only operation using fast mode is modular exponentiation (MODE = 0x02).

<!-- pagebreak -->

