363

Embedded flash memory (FLASH)

7.9.11

RM0456

FLASH ECC register (FLASH_ECCR)
Address offset: 0x30
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register is nonsecure. It can be read and written by both secure and nonsecure access.
This register can be protected against unprivileged access when NSPRIV = 1
in FLASH_PRIVCFGR register.

31

30

ECCD

ECCC

rc_w1

rc_w1

15

r

29

28

27

26

25

Res.

Res.

Res.

Res.

Res.

14

13

12

11

10

9

r

r

r

r

r

r

24
ECCIE

23
Res.

rw
8

7

22

21

20

SYSF_ BK_EC
ECC
C

19

18

17

16

ADDR_ECC[20:16]

r

r

r

r

r

r

r

6

5

4

3

2

1

0

r

r

r

r

r

r

r

ADDR_ECC[15:0]
r

r

Bit 31 ECCD: ECC detection
This bit is set by hardware when two ECC errors have been detected (only if ECCC and
ECCD were previously cleared). When this bit is set, a NMI is generated. This bit is cleared
by writing 1.
Bit 30 ECCC: ECC correction
This bit is set by hardware when one ECC error has been detected and corrected (only if
ECCC and ECCD were previously cleared). An interrupt is generated if ECCIE is set. This bit
is cleared by writing 1.
Bits 29:25 Reserved, must be kept at reset value.
Bit 24 ECCIE: ECC correction interrupt enable
This bit enables the interrupt generation when the ECCC bit in the FLASH_ECCR register is
set.
0: ECCC interrupt disabled
1: ECCC interrupt enabled.
Bit 23 Reserved, must be kept at reset value.
Bit 22 SYSF_ECC: System flash memory ECC fail
This bit indicates that the ECC error correction or double ECC error detection is located in
the system flash memory.
Bit 21 BK_ECC: ECC fail bank
This bit indicates which bank is concerned by the ECC error correction or by the double ECC
error detection.
0: Bank 1
1: Bank 2
Bits 20:0 ADDR_ECC[20:0]: ECC fail address
This field indicates which address is concerned by the ECC error correction or by the double
ECC error detection. The address is given by bank from address 0x0 0000 to address:
0x7 FFF0: upper address for STM32U535/545
0xF FFF0: upper address for STM32U575/585
0x1F FFF0: upper address for STM32U59x/5Ax/5Fx/5Gx

<!-- pagebreak -->

