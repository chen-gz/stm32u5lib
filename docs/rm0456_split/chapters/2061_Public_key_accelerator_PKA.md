RM0456 Rev 6

Refer to Section 2.3 for the register boundary addresses.
XONEIF
SEIF

0
0

SEIE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0
XONEIE

Reset value
KEIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

31
30
29
28
27
26
25
24
23
22
21
20
19
18
17
16
15
14
13
12
11
10
9
8
7
6
5
4
3
2
1
0

OTFDEC_ISR
Res.

XONEIF
SEIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Register
name

KEIF

Reset value

KEIE

Reset value
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OTFDEC_ICR

Res.

0x304
Res.

0x300

Res.

Offset

Res.

On-the-fly decryption engine (OTFDEC)
RM0456

Table 492. OTFDEC register map and reset values (continued)

0
0
0

0
0
0

RM0456

53

Public key accelerator (PKA)

Public key accelerator (PKA)
This section only applies to STM32U545/585/5Ax/5Gx devices.

53.1

Introduction
PKA (public key accelerator) is intended for the computation of cryptographic public key
primitives, specifically those related to RSA, Diffie-Hellmann or ECC (elliptic curve
cryptography) over GF(p) (Galois fields). To achieve high performance at a reasonable cost,
these operations are executed in the Montgomery domain.
For a given operation, all needed computations are performed within the accelerator, so no
further hardware/software elaboration is needed to process the inputs or the outputs.
When manipulating secrets, the PKA incorporates a protection against side-channel attacks
(SCA), including differential power analysis (DPA), certified SESIP and PSA security
assurance level 3.

53.2

PKA main features
•

Acceleration of RSA, DH and ECC over GF(p) operations, based on the Montgomery
method for fast modular multiplications. More specifically:
–

RSA modular exponentiation, RSA chinese remainder theorem (CRT)
exponentiation

–

ECC scalar multiplication, point on curve check, complete addition, double base
ladder, projective to affine

–

ECDSA signature generation and verification

•

Capability to handle operands up to 4160 bits for RSA/DH and 640 bits for ECC

•

When manipulating secrets: protection against side-channel attacks (SCA), including
differential power analysis (DPA), certified SESIP and PSA security assurance level 3
–

Applicable to modular exponentiation, ECC scalar multiplication and ECDSA
signature generation

•

Arithmetic and modular operations such as addition, subtraction, multiplication,
modular reduction, modular inversion, comparison, and Montgomery multiplication

•

Built-in Montgomery domain inward and outward transformations

•

AMBA AHB slave peripheral, accessible through 32-bit word single accesses only
(otherwise an AHB bus error is generated, and write accesses are ignored)

RM0456 Rev 6

<!-- pagebreak -->

