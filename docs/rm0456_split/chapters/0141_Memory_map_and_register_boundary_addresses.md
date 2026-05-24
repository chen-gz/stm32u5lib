RM0456 Rev 6

RM0456

2.3.2

Memory map and register boundary addresses
Figure 3. Memory map based on IDAU mapping for STM32U535/545
0x6000 0000
0x5602 6000
0x5602 0000
Non-secure

0x5600 8000
0x5600 0400

Secure, non-secure callable

0x520D 2800
0x5202 0000
0x5003 6C00
0x5002 0000

0xFFFF FFFF
Cortex M33
non-secure

0x5001 5C00
0x5001 2C00

0xE000 0000

0x5000 E000
0xA000 0000

0x5000 0000
OCTOSPI1 bank
non-secure

0x4602 6000
0x4602 0000

0x9000 0000

0x4600 8000
0x6000 0000

0x4600 0400
Peripherals
non-secure callable
Peripherals
non-secure

0x4003 6C00
0x4002 0000

0x4000 0000
0x3800 4000
0x3800 0000
0x3004 0000
0x3000 0000
0x2800 4000

0x2800 0000
0x2004 0000

0x4001 5C00
SRAM4
non-secure callable

0x4001 2C00
0x4000 E000

SRAM1/2
non-secure callable
SRAM4
non-secure

0x4000 0000
0x2000 0000
0x1000 0000
0x0FF8 8000
0x0FF8 0000
0x0E04 0000

SRAM1/2
non-secure

0x0E03 0000
0x0E00 0000

0x2000 0000
Code
non-secure

0x0C08 0000
0x0C00 0000

0x1000 0000
Code
non-secure callable
0x0C00 0000

0x0000 0000

0x420D 2800
0x4202 0000

0x5000 0000

0x0BFA 0200
0x0BFA 0000
0x0BF9 8000

Code
non-secure

0x0BF9 0000
0x0A04 0000
0x0A03 0000
0x0A00 0000
0x0808 0000
0x0800 0000
0x0000 0000

Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1
Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1

External memories
Reserved
RSS
Reserved
SRAM2
SRAM1
Reserved
FLASH
Reserved
OTP
Reserved
System memory
Reserved
SRAM2
SRAM1
Reserved
FLASH
External memories remap
MSv70740V2

RM0456 Rev 6

<!-- pagebreak -->

150

RM0456
Figure 4. Memory map based on IDAU mapping for STM32U575/585
0x6000 0000
0x5602 6000
0x5602 0000
Non-secure

0x5600 8000
0x5600 0400

Secure, non-secure callable

0x520D 2800
0x5202 0000
0x5003 6C00
0x5002 0000

0xFFFF FFFF
Cortex-M33
non-secure

0x5001 5C00
0x5001 2C00

0xE000 0000

0x5000 E000
0xA000 0000

0x5000 0000
OCTOSPI1 bank
non-secure

0x4602 6000
0x4602 0000

0x9000 0000

0x4600 8000

FMC bank 3
non-secure

0x4600 0400

0x8000 0000

0x420D 2800

OCTOSPI2 bank
non-secure

0x4202 0000
0x4003 6C00

0x7000 0000
FMC bank 1
non-secure

0x4002 0000
0x4001 5C00

0x6000 0000

0x4001 2C00

Peripherals
non-secure callable

0x4000 E000

0x5000 0000

0x4000 0000

Peripherals
non-secure

0x2000 0000
0x1000 0000

0x4000 0000
0x3800 4000
0x3800 0000
0x300C 0000
0x3000 0000
0x2800 4000

0x2800 0000
0x200C 0000

SRAM4
non-secure callable

0x0FF8 8000
0x0FF8 0000
0x0E0C 0000
0x0E04 0000

SRAM1/2/3
non-secure callable

0x0E03 0000
0x0E00 0000

SRAM4
non-secure

0x0C20 0000
0x0C00 0000

SRAM1/2/3
non-secure

0x0BFA 0200
0x0BFA 0000

0x2000 0000

0x0BF9 8000

Code
non-secure

0x0BF9 0000

0x1000 0000

0x0A0C 0000
Code
non-secure callable

0x0A04 0000
0x0A03 0000

0x0C00 0000

0x0000 0000

Code
non-secure

0x0A00 0000
0x0820 0000
0x0800 0000
0x0000 0000

Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1
Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1

External memories
Reserved
RSS
Reserved
SRAM3
SRAM2
SRAM1
Reserved
FLASH
Reserved
OTP
Reserved
System memory
Reserved
SRAM3
SRAM2
SRAM1
Reserved
FLASH
External memories remap
MSv63640V2

<!-- pagebreak -->

RM0456 Rev 6

RM0456
Figure 5. Memory map based on IDAU mapping for STM32U59x/5Ax
0x6000 0000
0x5602 6000
0x5602 0000
0x5600 8000
0x5600 0400
0x520D 3800
0x5202 0000

Non-secure

0x5003 6C00
Secure, non-secure callable

0x5002 0000
0x5001 7C00
0x5001 2C00

0xFFFF FFFF

0x5000 E000

Cortex M33
non-secure

0x5000 0000

0xE000 0000
0xB000 0000
0xA000 0000
0x9000 0000
0x8000 0000
0x7000 0000
0x6000 0000
0x5000 0000
0x4000 0000
0x3800 4000
0x3800 0000
0x3500 0000
0x3400 0000
0x3027 0000
0x3000 0000
0x2800 4000
0x2800 0000
0x2500 0000
0x2400 0000
0x2027 0000
0x2000 0000
0x1000 0000
0x0C00 0000
0x0000 0000

0x4602 6000
0x4602 0000

HSPI1 bank
non-secure

0x4600 8000
0x4600 0400

OCTOSPI1 bank
non-secure

0x420D 3800

FMC bank 3
non-secure

0x4202 0000
0x4003 6C00

OCTOSPI2 bank
non-secure

0x4002 0000

FMC bank 1
non-secure

0x4001 7C00

Peripherals
non-secure callable

0x4000 E000

0x4001 2C00

Peripherals
non-secure

0x4000 0000
0x2000 0000

SRAM4
non-secure callable

0x1000 0000
0x0FF8 8000
0x0FF8 0000

GFXMMU virtual buffer
non-secure callable
SRAM1/2/3/5
non-secure callable
SRAM4
non-secure

0x0E27 0000
0x0E1A 0000
0x0E0D 0000
0x0E0C 0000
0x0E00 0000
0x0C40 0000(3)

GFXMMU virtual buffer
non-secure

0x0C00 0000
0x0BFA 0200

SRAM1/2/3/5
non-secure

0x0BFA 0000

Code
non-secure

0x0BF9 0000

0x0BF9 8000
0x0A27 0000

Code
non-secure callable

0x0A1A 0000
0x0A0D 0000

Code
non-secure

0x0A0C 0000
0x0A00 0000
0x0840 0000
0x0800 0000
0x0000 0000

RM0456 Rev 6

Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1
Reserved
AHB3
Reserved
APB3
Reserved
AHB2
Reserved
AHB1
Reserved
APB2
Reserved
APB1

External memories
Reserved
RSS
Reserved
SRAM5
SRAM3
SRAM2
SRAM1
Reserved
FLASH
Reserved
OTP
Reserved
System memory
Reserved
SRAM5
SRAM3
SRAM2
SRAM1
Reserved
FLASH
External memories remap
MSv65678V2

<!-- pagebreak -->

150

RM0456
Figure 6. Memory map based on IDAU mapping for STM32U5Fx/5Gx
Non-secure

0x6000 0000
0x5602 6000

Secure, non-secure callable

0x5602 0000
0x5600 8000

0xFFFF FFFF

0x5600 0400

Cortex M33
non-secure

0x520D 3800

0xE000 0000

0x5202 0000
0x5003 6C00

0xB000 0000

0x5002 0000

HSPI1 bank
non-secure

0x5001 7C00

0xA000 0000

0x5001 2C00

OCTOSPI1 bank
non-secure

0x5000 E000

0x9000 0000

0x5000 0000
FMC bank 3
non-secure

0x4602 6000
0x4602 0000

0x8000 0000

0x4600 8000

OCTOSPI2 bank
non-secure

0x4600 0400

0x7000 0000

0x420D 3800

FMC bank 1
non-secure

0x4202 0000
0x4003 6C00

0x6000 0000
Peripherals
non-secure callable

0x4002 0000
0x4001 7C00

0x5000 0000

0x4001 2C00

Peripherals
non-secure
0x4000 0000
0x3800 4000

0x4000 E000
0x4000 0000

SRAM4
non-secure callable

0x2000 0000
0x1000 0000

0x3800 0000
0x3500 0000
GFXMMU virtual buffer
non-secure callable
0x3400 0000
0x302F 0000
0x3000 0000
0x2800 4000

0x0FF8 0000
0x0E2F 0000

SRAM1/2/3/5/6
non-secure callable

0x0E27 0000
0x0E1A 0000
0x0E0D 0000

SRAM4
non-secure

0x0E0C 0000

0x2800 0000
0x2500 0000

0x0E00 0000
GFXMMU virtual buffer
non-secure

0x2400 0000
0x202F 0000

0x0C40 0000
0x0C00 0000
0x0BFA 0200

SRAM1/2/3/5/6
non-secure

0x0BFA 0000

Code
non-secure

0x0BF9 0000

0x0BF9 8000

0x2000 0000

0x0A2F 0000

0x1000 0000

0x0A27 0000

Code
non-secure callable

0x0A1A 0000

0x0C00 0000

0x0000 0000

0x0FF8 8000

0x0A0D 0000
Code
non-secure

0x0A0C 0000
0x0A00 0000
0x0840 0000
0x0800 0000
0x0000 0000

<!-- pagebreak -->

