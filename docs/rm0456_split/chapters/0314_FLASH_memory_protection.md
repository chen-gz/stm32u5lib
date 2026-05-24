363

Embedded flash memory (FLASH)

RM0456

The figures below show how security attributes and protections behave in case of bank
swap.
Figure 23. Flash memory security attributes and protections
in case of no bank swap (SWAP_BANK = 0)
0x0800 0000/
0x0C00 0000
SECWM1_PSTRT

Bank 1

Bank 2

Page 0
Page 1

Page 0
Page 1
Privilege/secure

HDP
HDP1_PEND

0x0820 0000(1)/
0x0C20 0000
PRIV2BB2 = SEC2BB2 = 1
WRP2A_PSTRT

WRP
WRP2A_PEND
Secure
Secure

SEC2BB40=1

SECWM1_PEND
Page 255(2)

Page 255(2)
MSv65676V4

1. Valid for STM32U59x/5Ax/5Fx/5Gx. Bank 2 base address is 0x0810 0000/0x0C10 0000
for STM32U575/585, and 0x0804 0000/0x0C04 0000 for STM32U535/545.
2. Refer to Table 51 to Table 53 for last page number on each device.

Figure 24. Flash memory security attributes and protections
in case of bank swap (SWAP_BANK = 1)
0x0800 0000/
0x0C00 0000
PRIV2BB2 = SEC2BB2 = 1

Bank 2

Bank 1

Page 0
Page 1
Privilege/secure

Page 0
Page 1
HDP

0x0820 0000(1)/
0x0C20 0000
SECWM1_PSTRT
HDP1_PEND

WRP2A_PSTRT
WRP
WRP2A_PEND
Secure
SEC2BB40=1

Secure
SECWM1_PEND

Page 255(2)

Page 255(2)
MSv65677V4

1. Valid for STM32U59x/5Ax/5Fx/5Gx. Bank 1 base address is 0x0810 0000/0x0C10 0000 for
STM32U575/585, and 0x0804 0000/0x0C04 0000 for STM32U535/545.
2. Refer to Table 51 to Table 53 for last page number on each device.

7.6

FLASH memory protection
The flash memory interface implements the following protection mechanisms:

<!-- pagebreak -->

