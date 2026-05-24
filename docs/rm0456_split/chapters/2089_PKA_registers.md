Event

Event flag

Enable control bit

Clear method

Unsupported operation

OPERRF

OPERRIE

Set OPERRFC bit

Access to unmapped address error

ADDRERRF

ADDRERRIE

Set ADDRERRFC bit

PKA RAM access error

RAMERRF

RAMERRIE

Set RAMERRFC bit

PKA end of operation

PROCENDF

PROCENDIE

Set PROCENDFC bit

RM0456 Rev 6

RM0456

Public key accelerator (PKA)

53.7

PKA registers

53.7.1

PKA control register (PKA_CR)
Address offset: 0x00
Reset value: 0x0000 0000

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

OP
ERRIE

ADDR
ERRIE

RAM
ERRIE

18

17

16

Res.

PROC
ENDIE

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

rw

rw

rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

START

EN

rw

rw

MODE[5:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 OPERRIE: Operation error interrupt enable
0: No interrupt is generated when OPERRF flag is set in PKA_SR.
1: An interrupt is generated when OPERRF flag is set in PKA_SR.
Bit 20 ADDRERRIE: Address error interrupt enable
0: No interrupt is generated when ADDRERRF flag is set in PKA_SR.
1: An interrupt is generated when ADDRERRF flag is set in PKA_SR.
Bit 19 RAMERRIE: RAM error interrupt enable
0: No interrupt is generated when RAMERRF flag is set in PKA_SR.
1: An interrupt is generated when RAMERRF flag is set in PKA_SR.
Bit 18 Reserved, must be kept at reset value.
Bit 17 PROCENDIE: End of operation interrupt enable
0: No interrupt is generated when PROCENDF flag is set in PKA_SR.
1: An interrupt is generated when PROCENDF flag is set in PKA_SR.
Bits 16:14 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2093

Public key accelerator (PKA)

RM0456

Bits 13:8 MODE[5:0]: PKA operation code
000000: Montgomery parameter computation then modular exponentiation
000001: Montgomery parameter computation only
000010: Modular exponentiation only (Montgomery parameter must be loaded first)
000011: Modular exponentiation (protected, used when manipulating secrets)
100000: Montgomery parameter computation then ECC scalar multiplication (protected)
100100: ECDSA sign (protected)
100110: ECDSA verification
101000: Point on elliptic curve Fp check
000111: RSA CRT exponentiation
001000: Modular inversion
001001: Arithmetic addition
001010: Arithmetic subtraction
001011: Arithmetic multiplication
001100: Arithmetic comparison
001101: Modular reduction
001110: Modular addition
001111: Modular subtraction
010000: Montgomery multiplication
100011: ECC complete addition
100111: ECC double base ladder
101111: ECC projective to affine
When an operation not listed here is written by the application with EN bit set, OPERRF bit is
set in PKA_SR register, and the write to MODE bitfield is ignored.
Bits 7:2 Reserved, must be kept at reset value.
Bit 1 START: start the operation
Set this bit to start the operation selected by the MODE[5:0] bitfield, using the operands and
data already written to the PKA RAM. This bit is always read as 0.
When an illegal operation is selected while START bit is set no operation is started, and
OPERRF bit is set in PKA_SR.
Note: START is ignored if PKA is busy.
Bit 0 EN: PKA enable
0: Disable PKA
1: Enable PKA. PKA becomes functional when INITOK is set by hardware in PKA_SR.
When an illegal operation is selected while EN = 1, OPERRF bit is set in PKA_SR. See
PKA_CR.MODE bitfield for details.
Note: When EN = 0, PKA RAM can still be accessed by the application.

<!-- pagebreak -->

