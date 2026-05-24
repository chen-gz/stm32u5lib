482

Power control (PWR)

RM0456

NSPRIV in PWR_PRIVCFGR can be written with privileged access only, secure or
nonsecure. This bit configures the privileged access of all PWR securable functions that are
configured as nonsecure (defined by PWR_SECCFGR, GTZC, RCC or GPIO as shown in
Table 109)).
When NSPRIV is set in PWR_PRIVCFGR:

10.9

•

The PWR securable bits that are configured as nonsecure, can be written only with
privileged access.

•

The PWR securable bits that are configured as nonsecure, can be read only with
privileged access except PWR_PRIVCFGR that can be read by privileged or
unprivileged accesses.

•

The VOSRDY and BOOSTRDY bits in PWR_VOSR, PWR_SR, PWR_SVMSR,
PWR_BDSR and PWR_WUSR, can be read with privileged or unprivileged accesses.

•

An unprivileged access to a privileged PWR bit or register is discarded: the bits are
read as zero and the write to these bits is ignored (RAZ/WI).

PWR interrupts
The table below gives a summary of the interrupt sources and the way to control them.
Table 110. PWR interrupt requests

Interrupt
vector

PWR_S3WU(1)

PVD_PVM

<!-- pagebreak -->

Exit
Exit Sleep,
Stop 3,
Stop 0, 1, Standby,
2 modes Shutdown
modes

Interrupt event

Event flag

Enable
control bit

Interrupt clear
method

wake-up
interrupt flag

WUFx
(x = 1 to 8)

WUPENx
(x = 1 to 8)

Write CWUFx = 1
(x = 1 to 8)

No

Yes(2)

Programmable
voltage detector
through EXTI
line 16

PVDO

EXTI line 16
enabled

Write EXTI PIF16 = 1

Yes

No

USB supply
voltage monitor
through EXTI
line 19

VDDUSBRDY

EXTI line 19
enabled

Write EXTI PIF19 = 1

VDDIO2 supply
voltage monitor
through EXTI
line 20

VDDIO2RDY

EXTI line 20
enabled

Write EXTI PIF20 = 1

Analog supply
voltage monitor1
through EXTI
line 21

Yes

No

VDDA1RDY

EXTI line 21
enabled

Write EXTI PIF21 = 1

Analog supply
voltage monitor2
through EXTI
line 22

VDDA2RDY

EXTI line 22
enabled

Write EXTI PIF22 = 1

RM0456 Rev 6

RM0456

Power control (PWR)

1. The PWR_S3WU interrupt is generated only when STOP3 mode is selected (LPMS = 011 in PWR_CR1 register, not
applicable in Stop 0, Stop 1, and Stop 2 modes).
2. Only an interrupt can wake up from Stop 3 mode (not possible with an event).

10.10

PWR registers

10.10.1

PWR control register 1 (PWR_CR1)
Address offset: 0x00
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
This register is protected against nonsecure access when LPMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access

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

FORCE
_USBP
WR

Res.

Res.

Res.

rw

SRAM6 SRAM5 SRAM4 SRAM3 SRAM2 SRAM1 ULPME
RRSB2 RRSB1
PD
PD
PD
PD
PD
PD
N
rw

rw

rw

rw

rw

rw

rw

rw

rw

LPMS[2:0]
rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 FORCE_USBPWR: OTG_HS PHY power maintained during Stop 2, Stop 3, and Standby
low-power modes.
0: OTG_HS PHY power is not maintained during low-power modes.
1: OTG_HS PHY power is maintained during low-power modes.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 14 Reserved, must be kept at reset value.
Bit 13 SRAM6PD: SRAM6 power down
This bit is used to reduce the consumption by powering off the SRAM6.
0: SRAM6 powered on
1: SRAM6 powered off
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 12 SRAM5PD: SRAM5 power down
This bit is used to reduce the consumption by powering off the SRAM5.
0: SRAM5 powered on
1: SRAM5 powered off
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 11 SRAM4PD: SRAM4 power down
This bit is used to reduce the consumption by powering off the SRAM4.
0: SRAM4 powered on
1: SRAM4 powered off
Bit 10 SRAM3PD: SRAM3 power down
This bit is used to reduce the consumption by powering off the SRAM3.
0: SRAM3 powered on
1: SRAM3 powered off
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 9 SRAM2PD: SRAM2 power down
This bit is used to reduce the consumption by powering off the SRAM2.
0: SRAM2 powered on
1: SRAM2 powered off
Bit 8 SRAM1PD: SRAM1 power down
This bit is used to reduce the consumption by powering off the SRAM1.
0: SRAM1 powered on
1: SRAM1 powered off
Bit 7 ULPMEN: BOR0 ultra-low power mode
This bit is used to reduce the consumption by configuring the BOR in discontinuous mode.
This bit has effect only when the BOR level 0 is selected and when the device is in Standby
mode.
0: BOR level 0 operating in continuous (normal) mode in Standby mode
1: BOR level 0 operating in discontinuous (ultra-low power) mode in Standby mode
Caution: This bit must be set to reach the lowest power consumption in Standby mode.
Bit 6 RRSB2: SRAM2 page 2 retention in Stop 3 and Standby modes
This bit is used to keep the SRAM2 page 2 content in Stop 3 and Standby modes.
The SRAM2 page 2 corresponds to the last 56 Kbytes of the SRAM2
(from SRAM2 base address + 0x2000 to SRAM2 base address + 0xFFFF).
0: SRAM2 page2 content not retained in Stop3 and Standby modes
1: SRAM2 page2 content retained in Stop 3 and Standby modes
Note: This bit has no effect in Shutdown mode.
The backup SRAM is also retained when this bit is set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bit 5 RRSB1: SRAM2 page 1 retention in Stop 3 and Standby modes
This bit is used to keep the SRAM2 page 1 content in Stop 3 and Standby modes. The
SRAM2 page 1 corresponds to the first 8 Kbytes of the SRAM2
(from SRAM2 base address to SRAM2 base address + 0x1FFF).
0: SRAM2 page1 content not retained in Stop 3 and Standby modes
1: SRAM2 page1 content retained in Stop 3 and Standby modes
Note: This bit has no effect in Shutdown mode.
The backup SRAM is also retained when this bit is set.
Bits 4:3 Reserved, must be kept at reset value.
Bits 2:0 LPMS[2:0]: Low-power mode selection
These bits select the low-power mode entered when the CPU enters Deepsleep mode.
000: Stop 0 mode
001: Stop 1 mode
010: Stop 2 mode
011: Stop 3 mode
10x: Standby mode (Standby mode also entered if LPMS = 11X in PWR_CR1 with BREN = 1
in PWR_BDCR1)
11x: Shutdown mode if BREN = 0 in PWR_BDCR1

10.10.2

PWR control register 2 (PWR_CR2)
Address offset: 0x04
Reset value: 0x0000 0000
This register is protected against nonsecure access when LPMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1.

31
SRDR
UN

30
Res.

29
Res.

28
Res.

27
Res.

rw
15
Res.

14

13

12

11

26

25

JPEGR
DSIRA
AMPD
MPDS
S

24

23

22

21

20

rw

rw

rw

18

17

16

GPRA SRAM3 SRAM3 SRAM3 SRAM3 SRAM3 SRAM3 SRAM3 SRAM3
MPDS PDS8
PDS7
PDS6
PDS5
PDS4
PDS3
PDS2
PDS1

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

DMA2D
FLASH SRAM4 PKARA PRAM
DC1RA ICRAM DC2RA SRAM4 SRAM2 SRAM2
RAMP
FWU
FWU
MPDS
PDS
MPDS
PDS
MPDS
PDS
PDS2
PDS1
DS
rw

19

rw

rw

rw

rw

rw

rw

rw

Res.

SRAM1 SRAM1 SRAM1
PDS3
PDS2
PDS1
rw

rw

rw

Bit 31 SRDRUN: SmartRun domain in Run mode
0: SmartRun domain AHB3 and APB3 clocks disabled by default in Stop 0/1/2 modes
1: SmartRun domain AHB3 and APB3 clocks kept enabled in Stop 0/1/2 modes
Bits 30:27 Reserved, must be kept at reset value.
Bit 26 JPEGRAMPDS: JPEG SRAM power-down in Stop 0/1 modes
JPEG SRAM content is always lost in Stop 2 and Stop 3 modes.
0: JPEG SRAM content retained in Stop 0 and Stop 1 modes
1: JPEG SRAM content lost in Stop 0 and Stop 1 modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 25 DSIRAMPDS: DSI SRAM power-down in Stop 0/1 modes
DSI SRAM content is always lost in Stop 2 and Stop 3 modes.
0: DSI SRAM content retained in Stop 0 and Stop 1 modes
1: DSI SRAM content lost in Stop 0 and Stop 1 modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 24 GPRAMPDS: Graphic peripherals (LTDC, GFXMMU) SRAM power-down in all Stop modes
0: Graphic peripherals SRAM content retained in Stop modes
1: Graphic peripherals SRAM content lost in Stop modes
Note: LTDC SRAM content is always lost in Stop 2 and Stop 3 modes. It can be retained only
in Stop 0 and Stop 1 modes.
This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 23 SRAM3PDS8: SRAM3 page 8 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 8 content retained in Stop modes
1: SRAM3 page 8 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 22 SRAM3PDS7: SRAM3 page 7 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 7 content retained in Stop modes
1: SRAM3 page 7 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 21 SRAM3PDS6: SRAM3 page 6 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 6 content retained in Stop modes
1: SRAM3 page 6 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 20 SRAM3PDS5: SRAM3 page 5 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 5 content retained in Stop modes
1: SRAM3 page 5 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 19 SRAM3PDS4: SRAM3 page 4 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 4 content retained in Stop modes
1: SRAM3 page 4 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bit 18 SRAM3PDS3: SRAM3 page 3 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 3 content retained in Stop modes
1: SRAM3 page 3 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 17 SRAM3PDS2: SRAM3 page 2 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 2 content retained in Stop modes
1: SRAM3 page 2 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 16 SRAM3PDS1: SRAM3 page 1 (64 Kbytes) power-down in all Stop modes
0: SRAM3 page 1 content retained in Stop modes
1: SRAM3 page 1 content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 FLASHFWU: Flash memory fast wake-up from Stop 0 and Stop 1 modes
This bit is used to obtain the best trade-off between low-power consumption and wake-up
time when exiting the Stop 0 or Stop 1 modes.
When this bit is set, the flash memory remains in normal mode in Stop 0 and Stop 1 modes,
which offers a faster startup time with higher consumption.
0: Flash memory enters low-power mode in Stop 0/1 modes (lower-power consumption).
1: Flash memory remains in normal mode in Stop 0/1 modes (faster wake-up time).
Bit 13 SRAM4FWU: SRAM4 fast wake-up from Stop 0/1/2 modes
This bit is used to obtain the best trade-off between low-power consumption and wake-up
time. SRAM4 wake-up time increases the wake-up time when exiting Stop 0/1/2 modes, and
also increases the LPDMA access time to SRAM4 during Stop modes.
0: SRAM4 enters low-power mode in Stop 0/1/2 modes (source biasing for lower-power
consumption).
1: SRAM4 remains in normal mode in Stop 0/1/2 modes (higher consumption but
no SRAM4 wake-up time).
Bit 12 PKARAMPDS: PKA SRAM power-down in all Stop modes (Stop 0/1/2/3)
0: PKA SRAM content retained in Stop modes
1: PKA SRAM content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series.
Refer to the device datasheet for availability of its associated peripheral.
If not present, consider this bit reserved and keep at reset value.
Bit 11 PRAMPDS: FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM power-down in all Stop
modes (Stop 0/1/2/3)
0: FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content retained in Stop modes
1: FMAC, FDCAN, and USB/OTG_FS/OTG_HS SRAM content lost in Stop modes

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 10 DMA2DRAMPDS: DMA2D SRAM power-down in all Stop modes
0: DMA2D SRAM content retained in Stop modes
1: DMA2D SRAM content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 9 DC1RAMPDS: DCACHE1 SRAM power-down in all Stop modes
0: DCACHE1 SRAM content retained in Stop modes
1: DCACHE1 SRAM content lost in Stop modes
Bit 8 ICRAMPDS: ICACHE SRAM power-down in all Stop modes
0: ICACHE SRAM content retained in Stop modes
1: ICACHE SRAM content lost in Stop modes
Bit 7 DC2RAMPDS: DCACHE2 SRAM power-down in all Stop modes
0: DCACHE2 SRAM content retained in Stop modes
1: DCACHE2 SRAM content lost in Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 6 SRAM4PDS: SRAM4 power-down in all Stop modes
0: SRAM4 content retained in Stop modes
1: SRAM4 content lost in Stop modes
Bit 5 SRAM2PDS2: SRAM2 page 2 (56 Kbytes) power-down in Stop 0/1/2 modes
0: SRAM2 page 2 content retained in Stop modes
1: SRAM2 page 2 content lost in Stop modes
Note: The SRAM2 page 2 retention in Stop 3 is controlled by RRSB2 bit in PWR_CR1.
Bit 4 SRAM2PDS1: SRAM2 page 1 (8 Kbytes) power-down in Stop 0/1/2 modes
0: SRAM2 page 1 content retained in Stop modes
1: SRAM2 page 1 content lost in Stop modes
Note: The SRAM2 page 1 retention in Stop 3 is controlled by RRSB1 bit in PWR_CR1.
Bit 3 Reserved, must be kept at reset value.
Bit 2 SRAM1PDS3: SRAM1 page 3 (64 Kbytes) power-down in all Stop modes
0: SRAM1 page 3 content retained in Stop modes
1: SRAM1 page 3 content lost in Stop modes
Bit 1 SRAM1PDS2: SRAM1 page 2 (64 Kbytes) power-down in all Stop modes
0: SRAM1 page 2 content retained in Stop modes
1: SRAM1 page 2 content lost in Stop modes
Bit 0 SRAM1PDS1: SRAM1 page 1 (64 Kbytes) power-down in all Stop modes
0: SRAM1 page 1 content retained in Stop modes
1: SRAM1 page 1 content lost in Stop modes

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.3

PWR control register 3 (PWR_CR3)
Address offset: 0x08
Power-on reset value: 0x0000 0000
Exit from Standby modes: not affected
System reset: not affected, except REGSEL that is cleared to 0
This register is protected against nonsecure access when VDMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
VDMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when VDMSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access.

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

Res.

Res.

Res.

Res.

Res.

REGSE
FSTEN
L
rw

Res.

rw

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 FSTEN: Fast soft start
0: LDO/SMPS fast startup disabled (limited inrush current)
1: LDO/SMPS fast startup enabled
Bit 1 REGSEL: Regulator selection
0: LDO selected
1: SMPS selected
Note: REGSEL is reserved and must be kept at reset value in packages without SMPS.
Bit 0 Reserved, must be kept at reset value.

10.10.4

PWR voltage scaling register (PWR_VOSR)
Address offset: 0x0C
Reset value: 0x0000 8000
Some register fields are protected against nonsecure access depending
on RCC_SECCFGR. These fields can be protected against unprivileged access depending
on PWR_PRIVCFGR.

31
Res.

15

30
Res.

14

29
Res.

13

USBBO
VOSR BOOST
OSTRD
DY
RDY
Y
r

r

28
Res.

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21

20

19

18

VDD11
USBBO USBP BOOST
USBDI
OSTEN WREN
EN
S

17

16

VOS[1:0]

rw

rw

rw

rw

rw

rw

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

Res.

Res.

Res.

Res.

Res.

r

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 VDD11USBDIS: OTG_HS VDD11USB disable
This bit is protected against nonsecure access when SYSCLKSEC = 1 in RCC_SECCFGR.
It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and
SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
0: VDD11USB enabled
1: VDD11USB disabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 20 USBBOOSTEN: OTG_HS EPOD booster enable
This bit is protected against nonsecure access when SYSCLKSEC = 1 in RCC_SECCFGR.
It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and
SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
This bit must be set in range 1 and range 2 before enabling the OTG_HS.
This bit is reset when going in all Stop modes.
0: OTG_HS booster disabled
1: OTG_HS booster enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 19 USBPWREN: OTG_HS power enable
This bit is protected against nonsecure access when SYSCLKSEC = 1 in RCC_SECCFGR.
It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and
SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
0: OTG_HS power disabled
1: OTG_HS power enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 18 BOOSTEN: EPOD booster enable
This bit is protected against nonsecure access when SYSCLKSEC = 1 in RCC_SECCFGR.
It is protected against unprivileged access when SYSCLKSEC = 1 in RCC_SECCFGR and
SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and NSPRIV = 1.
This bit must be set in range 1 and range 2 before increasing the system clock frequency
above 55 MHz. This bit is reset when going in all Stop modes.
0: Booster disabled
1: Booster enabled
Bits 17:16 VOS[1:0]: Voltage scaling range selection
This field is protected against nonsecure access when SYSCLKSEC = 1 in
RCC_SECCFGR. It is protected against unprivileged access when SYSCLKSEC = 1 in
RCC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SYSCLKSEC = 0 and
NSPRIV = 1.
00: Range 4 (lowest power)
01: Range 3
10: Range 2
11: Range 1 (highest frequency)
Bit 15 VOSRDY: Ready bit for VCORE voltage scaling output selection
0: Not ready, voltage level < VOS selected level
1: Ready, voltage level ≥ VOS selected level

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bit 14 BOOSTRDY: EPOD booster ready
This bit is set to one by hardware when the power booster startup time is reached.
The system clock frequency can be switched higher than 55 MHz only after this bit is set.
0: Power booster not ready
1: Power booster ready
Bit 13 USBBOOSTRDY: OTG_HS EPOD booster ready
This bit is set to one by hardware when the power booster startup time is reached.
The OTG_HS clock can be provided only after this bit is set.
0: OTG_HS power booster not ready
1: OTG_HS power booster ready
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bits 12:0 Reserved, must be kept at reset value.

10.10.5

PWR supply voltage monitoring control register (PWR_SVMCR)
Address offset: 0x10
Reset value: 0x0000 0000
This register is protected against nonsecure access when VDMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
VDMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when VDMSEC = 0 and NSPRIV = 1.

31
Res.

30

29

28

ASV

IO2SV

USV

rw

rw

rw

27

26

25

AVM2E AVM1E IO2VM
N
N
EN
rw

rw

rw

24

23

22

21

20

19

18

17

16

UVME
N

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

rw

15

14

13

12

11

10

9

8

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

7

PVDLS[2:0]
rw

rw

rw

4

3

2

1

0

PVDE

Res.

Res.

Res.

Res.

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 ASV: VDDA independent analog supply valid
This bit is used to validate the VDDA supply for electrical and logical isolation purpose.
Setting this bit is mandatory to use the analog peripherals. If VDDA is not always present in
the application, the VDDA voltage monitor can be used to determine whether this supply is
ready or not.
0: VDDA not present: logical and electrical isolation is applied to ignore this supply.
1: VDDA valid
Bit 29 IO2SV: VDDIO2 independent I/Os supply valid
This bit is used to validate the VDDIO2 supply for electrical and logical isolation purpose.
Setting this bit is mandatory to use PG[15:2]. If VDDIO2 is not always present in the
application, the VDDIO2 voltage monitor can be used to determine whether this supply is
ready or not.
0: VDDIO2 not present: logical and electrical isolation is applied to ignore this supply.
1: VDDIO2 valid

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 28 USV: VDDUSB independent USB supply valid
This bit is used to validate the VDDUSB supply for electrical and logical isolation purpose.
Setting this bit is mandatory to use the USB/OTG_FS/OTG_HS. If VDDUSB is not always
present in the application, the VDDUSB voltage monitor can be used to determine whether
this supply is ready or not.
0: VDDUSB not present: logical and electrical isolation is applied to ignore this supply.
1: VDDUSB valid
Bit 27 AVM2EN: VDDA independent analog supply voltage monitor 2 enable (1.8 V threshold)
0: VDDA voltage monitor 2 disabled
1: VDDA voltage monitor 2 enabled
Bit 26 AVM1EN: VDDA independent analog supply voltage monitor 1 enable (1.6 V threshold)
0: VDDA voltage monitor 1 disabled
1: VDDA voltage monitor 1 enabled
Bit 25 IO2VMEN: VDDIO2 independent I/Os voltage monitor enable
0: VDDIO2 voltage monitor disabled
1: VDDIO2 voltage monitor enabled
Bit 24 UVMEN: VDDUSB independent USB voltage monitor enable
0: VDDUSB voltage monitor disabled
1: VDDUSB voltage monitor enabled
Bits 23:8 Reserved, must be kept at reset value.
Bits 7:5 PVDLS[2:0]: Programmable voltage detector (PVD) level selection
These bits select the voltage threshold detected by the PVD:
000: VPVD0 around 2.0 V
001: VPVD1 around 2.2 V
010: VPVD2 around 2.4 V
011: VPVD3 around 2.5 V
100: VPVD4 around 2.6 V
101: VPVD5 around 2.8 V
110: VPVD6 around 2.9 V
111: External input analog voltage PVD_IN (compared internally to VREFINT)
Bit 4 PVDE: Programmable voltage detector enable
0: PVD disabled
1: PVD enabled
Bits 3:0 Reserved, must be kept at reset value.

10.10.6

PWR wake-up control register 1 (PWR_WUCR1)
Access: 14 AHB clock cycles added compared to a standard AHB access
Address offset: 0x14
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each WUPENx (x = 1 to 8) is protected against nonsecure access when WUPxSEC = 1
in PWR_SECCFGR. Each WUPENx is protected against unprivileged access when

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)
WUPxSEC = 1 in PWR_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when
WUPxSEC = 0 and NSPRIV =1 .

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

WUPE
N8

WUPE
N7

WUPE
N6

WUPE
N5

WUPE
N4

WUPE
N3

WUPE
N2

WUPE
N1

rw

rw

rw

rw

rw

rw

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 WUPEN8: wake-up pin WKUP8 enable
0: WKUP8 disabled
1: WKUP8 enabled
Bit 6 WUPEN7: wake-up pin WKUP7 enable
0: WKUP7 disabled
1: WKUP7 enabled
Bit 5 WUPEN6: wake-up pin WKUP6 enable
0: WKUP6 disabled
1: WKUP6 enabled
Bit 4 WUPEN5: wake-up pin WKUP5 enable
0: WKUP5 disabled
1: WKUP5 enabled
Bit 3 WUPEN4: wake-up pin WKUP4 enable
0: WKUP4 disabled
1: WKUP4 enabled
Bit 2 WUPEN3: wake-up pin WKUP3 enable
0: WKUP3 disabled
1: WKUP3 enabled
Bit 1 WUPEN2: wake-up pin WKUP2 enable
0: WKUP2 disabled
1: WKUP2 enabled
Bit 0 WUPEN1: wake-up pin WKUP1 enable
0: WKUP1 disabled
1: WKUP1 enabled

10.10.7

PWR wake-up control register 2 (PWR_WUCR2)
Address offset: 0x18
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each WUPPx (x = 1 to 8) is protected against nonsecure access when WUPxSEC = 1
in PWR_SECCFGR. Each WUPPx is protected against unprivileged access when

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

WUPxSEC = 1 in PWR_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when
WUPxSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

WUPP
8

WUPP
7

WUPP
6

WUPP
5

WUPP
4

WUPP
3

WUPP
2

WUPP
1

rw

rw

rw

rw

rw

rw

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 WUPP8: wake-up pin WKUP8 polarity
This bit must be configured when WUPEN8 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 6 WUPP7: wake-up pin WKUP7 polarity
This bit must be configured when WUPEN7 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 5 WUPP6: wake-up pin WKUP6 polarity
This bit must be configured when WUPEN6 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 4 WUPP5: wake-up pin WKUP5 polarity
This bit must be configured when WUPEN5 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 3 WUPP4: wake-up pin WKUP4 polarity
This bit must be configured when WUPEN4 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 2 WUPP3: wake-up pin WKUP3 polarity
This bit must be configured when WUPEN3 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 1 WUPP2: wake-up pin WKUP2 polarity
This bit must be configured when WUPEN2 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)
Bit 0 WUPP1: wake-up pin WKUP1 polarity.
This bit must be configured when WUPEN1 = 0.
0: Detection on high level (rising edge)
1: Detection on low level (falling edge)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.8

PWR wake-up control register 3 (PWR_WUCR3)
Address offset: 0x1C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each WUSELx (x = 1 to 8) is protected against nonsecure access when WUPxSEC = 1
in PWR_SECCFGR. Each WUSELx is protected against unprivileged access when
WUPxSEC = 1 in PWR_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when
WUPxSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access

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

WUSEL8[1:0]

WUSEL7[1:0]

WUSEL6[1:0]

WUSEL5[1:0]

WUSEL4[1:0]

WUSEL3[1:0]

WUSEL2[1:0]

WUSEL1[1:0]

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:14 WUSEL8[1:0]: wake-up pin WKUP8 selection
This field must be configured when WUPEN8 = 0.
00: WKUP8_0
01: WKUP8_1
10: WKUP8_2
11: WKUP8_3
Bits 13:12 WUSEL7[1:0]: wake-up pin WKUP7 selection
This field must be configured when WUPEN7 = 0.
00: WKUP7_0
01: WKUP7_1
10: WKUP7_2
11: WKUP7_3
Bits 11:10 WUSEL6[1:0]: wake-up pin WKUP6 selection
This field must be configured when WUPEN6 = 0.
00: WKUP6_0
01: WKUP6_1
10: WKUP6_2
11: WKUP6_3
Bits 9:8 WUSEL5[1:0]: wake-up pin WKUP5 selection
This field must be configured when WUPEN5 = 0.
00: WKUP5_0
01: WKUP5_1
10: WKUP5_2
11: WKUP5_3

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bits 7:6 WUSEL4[1:0]: wake-up pin WKUP4 selection
This field must be configured when WUPEN4 = 0.
00: WKUP4_0
01: WKUP4_1
10: WKUP4_2
11: WKUP4_3
Bits 5:4 WUSEL3[1:0]: wake-up pin WKUP3 selection
This field must be configured when WUPEN3 = 0.
00: WKUP3_0
01: WKUP3_1
10: WKUP3_2
11: WKUP3_3
Bits 3:2 WUSEL2[1:0]: wake-up pin WKUP2 selection
This field must be configured when WUPEN2 = 0.
00: WKUP2_0
01: WKUP2_1
10: WKUP2_2
11: WKUP2_3
Bits 1:0 WUSEL1[1:0]: wake-up pin WKUP1 selection
This field must be configured when WUPEN1 = 0.
00: WKUP0_0
01: WKUP0_1
10: WKUP0_2
11: WKUP0_3

10.10.9

PWR backup domain control register 1 (PWR_BDCR1)
Address offset: 0x20
Backup domain reset value: 0x0000 0000
Power-on reset: not affected
The register is not affected when exiting Standby mode.
System reset: not affected
This register is write-protected when DBP is cleared in PWR_DBPR. This register is
protected against nonsecure access when VBSEC = 1 in PWR_SECCFGR. This register is
protected against unprivileged access when VBSEC = 1 and SPRIV = 1 in
PWR_PRIVCFGR, or when VBSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access

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

Res.

Res.

Res.

MONE
N

Res.

Res.

Res.

BREN

rw

Bits 31:5 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Power control (PWR)

Bit 4 MONEN: Backup domain voltage and temperature monitoring enable
0: Backup domain voltage and temperature monitoring disabled
1: Backup domain voltage and temperature monitoring enabled
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 BREN: Backup RAM retention in Standby and VBAT modes
When this bit is set, the backup RAM content is kept in Standby(1) and VBAT modes.
If BREN is reset, the backup RAM can still be used in Run, Sleep, and Stop modes. However,
its content is lost in Standby, Shutdown, and VBAT modes. This bit can be written only when
the regulator is LDO, which must be configured before switching to SMPS.
0: Backup RAM content lost in Standby(1) and VBAT modes
1: Backup RAM content preserved in Standby and VBAT modes
Note: Backup RAM cannot be preserved in Shutdown mode.
1. The backup SRAM content is lost in Standby mode without SRAM2 retention. If either RRSB1 or RRSB2 bit is set
in Standby mode, the backup SRAM is also retained.

10.10.10 PWR backup domain control register 2 (PWR_BDCR2)
Address offset: 0x24
Power-on reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
System reset: not affected
This register is protected against nonsecure access when VBSEC = 1 in PWR_SECCFGR.
This register is protected against unprivileged access when VBSEC = 1 and SPRIV = 1
in PWR_PRIVCFGR, or when VBSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

Res.

Res.

Res.

Res.

Res.

Res.

VBRS

VBE

rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 VBRS: VBAT charging resistor selection
0: Charge VBAT through a 5 kΩ resistor.
1: Charge VBAT through a 1.5 kΩ resistor.
Bit 0 VBE: VBAT charging enable
0: VBAT battery charging disabled
1: VBAT battery charging enabled

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

10.10.11 PWR disable backup domain register (PWR_DBPR)
Address offset: 0x28
Reset value: 0x0000 0000
This register is protected against nonsecure access when VBSEC = 1 in PWR_SECCFGR.
This register is protected against unprivileged access when VBSEC = 1 and SPRIV = 1 in
PWR_PRIVCFGR, or when VBSEC = 0 and NSPRIV = 1.
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DBP
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 DBP: Disable backup domain write protection
In reset state, all registers and SRAM in backup domain are protected against parasitic write
access. This bit must be set to enable the write access to these registers.
0: Write access to backup domain disabled
1: Write access to backup domain enabled

10.10.12 PWR UCPD register (PWR_UCPDR)
Address offset: 0x2C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
This register is protected against nonsecure access when UCPD1SEC = 1
in TZSC_SECCFGR. This register is protected against unprivileged access when
UCPD1SEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when UCPD1SEC = 0 and
NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

Res.

Res.

Res.

Res.

Res.

Res.

UCPD_ UCPD_
STBY DBDIS
rw

Bits 31:2 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Power control (PWR)

Bit 1 UCPD_STBY: UCPD Stop 3 and Standby modes
When set, this bit is used to memorize the UCPD configuration in Stop 3 and Standby modes.
This bit must be written to one just before entering Stop 3 or Standby mode when
using UCPD. It must be written to zero after exiting Stop 3 or Standby mode, and before
writing any UCPD registers.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.
Bit 0 UCPD_DBDIS: UCPD dead battery disable
After exiting reset, the USB Type-C “dead battery” behavior is enabled, which may have a
pull-down effect on CC1 and CC2 pins. It is recommended to disable it in all cases, either
to stop this pull-down, or to handover control to the UCPD (that must be initialized before
doing the disable).
0: UCPD dead battery pull-down behavior enabled on UCPDx_CC1 and UCPDx_CC2 pins
1: UCPD dead battery pull-down behavior disabled on UCPDx_CC1 and UCPDx_CC2 pins
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and keep it at reset value.

10.10.13 PWR security configuration register (PWR_SECCFGR)
Address offset: 0x30
Reset value: 0x0000 0000
This register can be written only when the access is secure. It can be read by secure or
nonsecure access. This register is write-protected against unprivileged write access when
SPRIV = 1 in PWR_PRIVCFGR.
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

WUP8
SEC

WUP7
SEC

WUP6
SEC

WUP5
SEC

WUP4
SEC

WUP3
SEC

WUP2
SEC

WUP1
SEC

rw

rw

rw

rw

rw

rw

rw

rw

APCSE
VDMS LPMSE
VBSEC
C
EC
C
rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 APCSEC: Pull-up/pull-down secure protection
0: PWR_APCR can be read and written with secure or nonsecure access.
1: PWR_APCR can be read and written only with secure access.
Bit 14 VBSEC: Backup domain secure protection
0: PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written with secure or
nonsecure access.
1: PWR_BDCR1, PWR_BDCR2, and PWR_DBPR can be read and written only with
secure access.
Bit 13 VDMSEC: Voltage detection and monitoring secure protection
0: PWR_SVMCR and PWR_CR3 can be read and written with secure or nonsecure access.
1: PWR_SVMCR and PWR_CR3 can be read and written only with secure access.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 12 LPMSEC: Low-power modes secure protection
0: PWR_CR1, PWR_CR2 and CSSF in the PWR_SR can be read and written with secure or
nonsecure access.
1: PWR_CR1, PWR_CR2, and CSSF in the PWR_SR can be read and written only with
secure access.
Bits 11:8 Reserved, must be kept at reset value.
Bit 7 WUP8SEC: WUP8 secure protection
0: Bits related to WKUP8 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP8 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 6 WUP7SEC: WUP7 secure protection
0: Bits related to WKUP7 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP7 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 5 WUP6SEC: WUP6 secure protection
0: Bits related to WKUP6 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP6 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 4 WUP5SEC: WUP5 secure protection
0: Bits related to WKUP5 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP5 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 3 WUP4SEC: WUP4 secure protection
0: Bits related to WKUP4 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP4 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 2 WUP3SEC: WUP3 secure protection
0: Bits related to WKUP3 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP3 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 1 WUP2SEC: WUP2 secure protection
0: Bits related to WKUP2 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR be read and written with secure or nonsecure access.
1: Bits related to WKUP2 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.
Bit 0 WUP1SEC: WUP1 secure protection
0: Bits related to WKUP1 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written with secure or nonsecure access.
1: Bits related to WKUP1 pin in PWR_WUCR1, PWR_WUCR2, PWR_WUCR3, and
PWR_WUSCR can be read and written only with secure access.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.14 PWR privilege control register (PWR_PRIVCFGR)
Address offset: 0x34
Reset value: 0x0000 0000
This register can be written only when the access is privileged. It can be read by privileged
or unprivileged access.
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

NSPRI
V

SPRIV

rw

rw

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

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 NSPRIV: PWR nonsecure functions privilege configuration
This bit is set and reset by software. It can be written only by privileged access, secure or
nonsecure.
0: Read and write to PWR nonsecure functions can be done by privileged or unprivileged
access.
1: Read and write to PWR nonsecure functions can be done by privileged access only.
Bit 0 SPRIV: PWR secure functions privilege configuration
This bit is set and reset by software. It can be written only by a secure privileged access.
0: Read and write to PWR secure functions can be done by privileged or unprivileged
access.
1: Read and write to PWR secure functions can be done by privileged access only.

10.10.15 PWR status register (PWR_SR)
Address offset: 0x38
Reset value: 0x0000 0000
Some register fields are protected against nonsecure access depending
on PWR_SECCFGR. Some register fields are protected against unprivileged access
depending on PWR_PRIVCFGR.
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

Res.

Res.

Res.

Res.

Res.

SBF

STOPF

CSSF

r

r

w

Bits 31:3 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 2 SBF: Standby flag
This bit is set by hardware when the device enters Standby mode, and is cleared by writing
one to CSSF bit, or by a power-on reset. It is not cleared by the system reset.
0: The device did not enter Standby mode.
1: The device entered Standby mode.
Bit 1 STOPF: Stop flag
This bit is set by hardware when the device enters a Stop mode, and is cleared by software
by writing one to CSSF bit.
0: The device did not enter any Stop mode.
1: The device entered a Stop mode.
Bit 0 CSSF: Clear Stop and Standby flags
This bit is protected against nonsecure access when LPMSEC = 1 in PWR_SECCFGR. This
bit is protected against unprivileged access when LPMSEC = 1 and SPRIV = 1
in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1.
Writing 1 to this bit clears the STOPF and SBF flags.

10.10.16 PWR supply voltage monitoring status register (PWR_SVMSR)
Address offset: 0x3C
Reset value: 0x0000 8000
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

ACTVOS[1:0]

r

r

r

r

15

14

13

12

11

10

9

r

r

8

7

6

5

4

3

2

1

0

ACTVO
SRDY

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

PVDO

Res.

Res.

REGS

Res.

VDDA2 VDDA1 VDDIO VDDUS
RDY
RDY
2RDY BRDY

r

r

r

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 VDDA2RDY: VDDA ready versus 1.8 V voltage monitor
0: VDDA is below the threshold of the VDDA voltage monitor 2 (around 1.8 V).
1: VDDA is equal or above the threshold of the VDDA voltage monitor 2 (around 1.8 V).
Bit 26 VDDA1RDY: VDDA ready versus 1.6V voltage monitor
0: VDDA is below the threshold of the VDDA voltage monitor 1 (around 1.6 V).
1: VDDA is equal or above the threshold of the VDDA voltage monitor 1 (around 1.6 V).
Bit 25 VDDIO2RDY: VDDIO2 ready
0: VDDIO2 is below the threshold of the VDDIO2 voltage monitor.
1: VDDIO2 is equal or above the threshold of the VDDIO2 voltage monitor.
Bit 24 VDDUSBRDY: VDDUSB ready
0: VDDUSB is below the threshold of the VDDUSB voltage monitor.
1: VDDUSB is equal or above the threshold of the VDDUSB voltage monitor.
Bits 23:18 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

16

RM0456

Power control (PWR)

Bits 17:16 ACTVOS[1:0]: VOS currently applied to VCORE(last VOS value)
00: Range 4 (lowest power)
01: Range 3
10: Range 2
11: Range 1 (highest frequency)
Bit 15 ACTVOSRDY: Voltage level ready for currently used VOS
0: VCORE is above or below the current voltage scaling provided by ACTVOS[1:0].
1: VCORE is equal to the current voltage scaling provided by ACTVOS[1:0]
Bits 14:5 Reserved, must be kept at reset value.
Bit 4 PVDO: Programmable voltage detector output
0: VDD is equal or above the PVD threshold selected by PVDLS[2:0].
1: VDD is below the PVD threshold selected by PVDLS[2:0].
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 REGS: Regulator selection
0: LDO selected
1: SMPS selected
Bit 0 Reserved, must be kept at reset value.

10.10.17 PWR backup domain status register (PWR_BDSR)
Address offset: 0x40
Backup domain reset value: 0x0000 0000
Power-on reset: not affected
The register is not affected when exiting Standby mode.
System reset: not affected
Access: 14 AHB clock cycles added compared to a standard AHB access
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

Res.

Res.

Res.

Res.

TEMPH TEMPL VBATH
r

r

Res.

r

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 TEMPH: Temperature level monitoring versus high threshold
0: Temperature < high threshold
1: Temperature ≥ high threshold
Bit 2 TEMPL: Temperature level monitoring versus low threshold
0: Temperature > low threshold
1: Temperature ≤ low threshold
Bit 1 VBATH: Backup domain voltage level monitoring versus high threshold
0: Backup domain voltage level < high threshold
1: Backup domain voltage level ≥ high threshold

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bit 0 Reserved, must be kept at reset value.

10.10.18 PWR wake-up status register (PWR_WUSR)
Address offset: 0x44
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

WUF8

WUF7

WUF6

WUF5

WUF4

WUF3

WUF2

WUF1

r

r

r

r

r

r

r

r

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 WUF8: wake-up flag 8
This bit is set when a wake-up event is detected on WKUP8 pin. This bit is cleared by writing
one in CWUF8 bit of PWR_WUSCR when WUSEL8 ≠ 11, or by hardware when
WUPEN8 = 0.
If WUSEL8 = 11, this bit is cleared by hardware when all internal wake-up source are
cleared.
Bit 6 WUF7: wake-up flag 7
This bit is set when a wake-up event is detected on WKUP7 pin. This bit is cleared by writing
one in CWUF7 bit of PWR_WUSCR when WUSEL7 ≠ 11, or by hardware when
WUPEN7 = 0.
If WUSEL7 = 11, this bit is cleared by hardware when all internal wake-up source are
cleared.
Bit 5 WUF6: wake-up flag 6
This bit is set when a wake-up event is detected on WKUP6 pin. This bit is cleared by writing
one in CWUF6 bit of PWR_WUSCR when WUSEL6 ≠ 11, or by hardware when
WUPEN6 = 0.
If WUSEL6 = 11, this bit is cleared by hardware when all internal wake-up source are
cleared.
Bit 4 WUF5: wake-up flag 5
This bit is set when a wake-up event is detected on WKUP5 pin. This bit is cleared by writing
1 in the CWUF5 bit of PWR_WUSCR, or by hardware when WUPEN5 = 0.
Bit 3 WUF4: wake-up flag 4
This bit is set when a wake-up event is detected on WKUP4 pin. This bit is cleared by writing
one in CWUF4 bit of PWR_WUSCR, or by hardware when WUPEN4 = 0.
Bit 2 WUF3: wake-up flag 3
This bit is set when a wake-up event is detected on WKUP3 pin. This bit is cleared by writing
one in CWUF3 bit of PWR_WUSCR, or by hardware when WUPEN3 = 0.
Bit 1 WUF2: wake-up flag 2
This bit is set when a wake-up event is detected on WKUP2 pin. This bit is cleared by writing
one in CWUF2 bit of PWR_WUSCR, or by hardware when WUPEN2 = 0.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bit 0 WUF1: wake-up flag 1
This bit is set when a wake-up event is detected on WKUP1 pin. This bit is cleared by writing
one in CWUF1 bit of PWR_WUSCR, or by hardware when WUPEN1 = 0.

10.10.19 PWR wake-up status clear register (PWR_WUSCR)
Address offset: 0x48
Reset value: 0x0000 0000
Each CWUFx (x = 1 to 8) is protected against nonsecure access when WUPxSEC = 1
in PWR_SECCFGR. Each CWUFx is protected against unprivileged access when
WUPxSEC = 1 in PWR_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when
WUPxSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

CWUF
8

CWUF
7

CWUF
6

CWUF
5

CWUF
4

CWUF
3

CWUF
2

CWUF
1

w

w

w

w

w

w

w

w

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 CWUF8: wake-up flag 8
Writing one to this bit clears the WUF8 flag in PWR_WUSR.
Bit 6 CWUF7: wake-up flag 7
Writing one to this bit clears the WUF7 flag in PWR_WUSR.
Bit 5 CWUF6: wake-up flag 6
Writing one to this bit clears the WUF6 flag in PWR_WUSR.
Bit 4 CWUF5: wake-up flag 5
Writing one to this bit clears the WUF5 flag in PWR_WUSR.
Bit 3 CWUF4: wake-up flag 4
Writing one to this bit clears the WUF4 flag in PWR_WUSR.
Bit 2 CWUF3: wake-up flag 3
Writing one to this bit clears the WUF3 flag in PWR_WUSR.
Bit 1 CWUF2: wake-up flag 2
Writing one to this bit clears the WUF2 flag in PWR_WUSR.
Bit 0 CWUF1: wake-up flag 1
Writing one to this bit clears the WUF1 flag in PWR_WUSR.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

10.10.20 PWR apply pull configuration register (PWR_APCR)
Address offset: 0x4C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
This register is protected against nonsecure access when APCSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
APCSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when APCSEC = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

APC
rw

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 APC: Apply pull-up and pull-down configuration
1: I/O pull-up and pull-down configurations defined in PWR_PUCRx and PWR_PDCRx are
applied.
0: PWR_PUCRx and PWR_PDCRx are not applied to the I/Os.
Note: When APC is set, I/Os configurations from GPIO registers are still applied in Run and
Sleep modes (ORed with PWR registers), so care must be taken to define a coherent
I/O configuration in GPIO and PWR pull-up/pull_down control registers.

10.10.21 PWR port A pull-up control register (PWR_PUCRA)
Address offset: 0x50
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOA_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOA_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

Res.

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bit 15 PU15: Port A pull-up bit 15
When set, this bit activates the pull-up on PA15 when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PD15 bit is also set.
Bit 14 Reserved, must be kept at reset value.
Bits 13:0 PUy: Port A pull-up bit y (y = 13 to 0)
When set, each bit activates the pull-up on PAy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.22 PWR port A pull-down control register (PWR_PDCRA)
Address offset: 0x54
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOA_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOA_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD14

Res.

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 PD14: Port A pull-down bit 14
When set, this bit activates the pull-down on PA14 when APC is set in PWR_APCR.
Bit 13 Reserved, must be kept at reset value.
Bits 12:0 PDy: Port A pull-down bit y (y = 12 to 0)
When set, each bit activates the pull-down on PAy when APC is set in PWR_APCR.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

10.10.23 PWR port B pull-up control register (PWR_PUCRB)
Address offset: 0x58
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOB_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOB_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port B pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PBy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.24 PWR port B pull-down control register (PWR_PDCRB)
Address offset: 0x5C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOB_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOB_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

Res.

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:5 PDy: Port B pull-down bit y (y = 15 to 5)
When set, each bit activates the pull-down on PBy when APC is set in PWR_APCR.
Bit 4 Reserved, must be kept at reset value.
Bits 3:0 PDy: Port B pull-down bit y (y = 3 to 0)
When set, each bit activates the pull-down on PBy when APC is set in PWR_APCR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.25 PWR port C pull-up control register (PWR_PUCRC)
Address offset: 0x60
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each register bit PUy is protected against nonsecure access when SECy = 1 in
GPIOC_SECCFGR.
Each register bit PUy is protected against unprivileged access when SECy = 1 in
GPIOC_SECCFGR and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and
NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port C pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PCy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.26 PWR port C pull-down control register (PWR_PDCRC)
Address offset: 0x64
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOC_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOC_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port C pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PCy when APC is set in PWR_APCR.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

10.10.27 PWR port D pull-up control register (PWR_PUCRD)
Address offset: 0x68
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOD_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOD_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port D pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PDy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.28 PWR port D pull-down control register (PWR_PDCRD)
Address offset: 0x6C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOD_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOD_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port D pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PDy when APC is set in PWR_APCR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.29 PWR port E pull-up control register (PWR_PUCRE)
Address offset: 0x70
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOE_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOE_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port E pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PEy when the APC bit is set in PWR_APCR. The
pull-up is not activated if the corresponding PDy bit is also set.

10.10.30 PWR port E pull-down control register (PWR_PDCRE)
Address offset: 0x74
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOE_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOE_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy =0 and NSPRIV =1 .
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port E pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PEy when APC is set in PWR_APCR.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

10.10.31 PWR port F pull-up control register (PWR_PUCRF)
Address offset: 0x78
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOF_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOF_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port F pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PFy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.32 PWR port F pull-down control register (PWR_PDCRF)
Address offset: 0x7C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOF_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOF_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV =1 .
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port F pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PFy when APC is set in PWR_APCR.

10.10.33 PWR port G pull-up control register (PWR_PUCRG)
Address offset: 0x80
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOG_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOG_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port G pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PGy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.34 PWR port G pull-down control register (PWR_PDCRG)
Address offset: 0x84
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOG_SECCFGR.
Each PDy is protected against unprivileged access when SECy =1 in GPIOG_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bits 15:0 PDy: Port G pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PGy when APC is set in PWR_APCR .

10.10.35 PWR port H pull-up control register (PWR_PUCRH)
Address offset: 0x88
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOH_SECCFGR.
Each PUy is protected against unprivileged access when SECy = 1 in GPIOH_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port H pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PHy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.36 PWR port H pull-down control register (PWR_PDCRH)
Address offset: 0x8C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOH_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOH_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port H pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PHy when APC is set in PWR_APCR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.37 PWR port I pull-up control register (PWR_PUCRI)
Address offset: 0x90
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOI_SECCFGR.
Each PUy is protected against unprivileged access when SECy=1 in GPIOI_SECCFGR and
SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PU15

PU14

PU13

PU12

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PUy: Port I pull-up bit y (y = 15 to 0)
When set, each bit activates the pull-up on PIy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

10.10.38 PWR port I pull-down control register (PWR_PDCRI)
Address offset: 0x94
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOI_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOI_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PD15

PD14

PD13

PD12

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 PDy: Port I pull-down bit y (y = 15 to 0)
When set, each bit activates the pull-down on PIy when APC is set in PWR_APCR.

10.10.39 PWR port J pull-up control register (PWR_PUCRJ)
Address offset: 0x98
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PUy is protected against nonsecure access when SECy = 1 in GPIOJ_SECCFGR.
Each PUy is protected against unprivileged access when SECy=1 in GPIOJ_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PU11

PU10

PU9

PU8

PU7

PU6

PU5

PU4

PU3

PU2

PU1

PU0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 PUy: Port J pull-up bit y (y = 11 to 0)
When set, each bit activates the pull-up on PJy when APC is set in PWR_APCR. The pull-up
is not activated if the corresponding PDy bit is also set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Power control (PWR)

10.10.40 PWR port J pull-down control register (PWR_PDCRJ)
Address offset: 0x9C
Reset value: 0x0000 0000
The register is not affected when exiting Standby mode.
Each PDy is protected against nonsecure access when SECy = 1 in GPIOJ_SECCFGR.
Each PDy is protected against unprivileged access when SECy = 1 in GPIOJ_SECCFGR
and SPRIV = 1 in PWR_PRIVCFGR, or when SECy = 0 and NSPRIV = 1.
Access: 14 AHB clock cycles added compared to a standard AHB access
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

PD11

PD10

PD9

PD8

PD7

PD6

PD5

PD4

PD3

PD2

PD1

PD0

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 PDy: Port J pull-down bit y (y = 11 to 0)
When set, each bit activates the pull-down on PJy when APC is set in PWR_APCR

10.10.41 PWR control register 4 (PWR_CR4)
Address offset: 0xA8
Reset value: 0x0000 0000
This register is protected against nonsecure access when LPMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1.
Note:

31
Res.

15
Res.

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.
30
Res.

14

29
Res.

13

28

27

26

rw

rw

rw

rw

12

11

10

9

SRAM3 SRAM3 SRAM3 SRAM3 SRAM3
PDS13 PDS12 PDS11 PDS10 PDS9
rw

rw

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

SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5 SRAM5
PDS13 PDS12 PDS11 PDS10 PDS9
PDS8
PDS7
PDS6
PDS5
PDS4
PDS3
PDS2
PDS1

rw

rw

rw

Res.

rw

rw

rw

rw

rw

rw

rw

rw

rw

8

7

6

5

4

3

2

1

0

SRAM1 SRAM1 SRAM1 SRAM1 SRAM1 SRAM1 SRAM1 SRAM1 SRAM1
PDS12 PDS11 PDS10 PDS9
PDS8
PDS7
PDS6
PDS5
PDS4
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

Bits 28:16 SRAM5PDSx: SRAM5 64-Kbyte page x (x = 13 to 1) power-down in all Stop modes
0: SRAM5 page x content retained in Stop modes
1: SRAM5 page x content lost in Stop modes
Bit 15 Reserved, must be kept at reset value.
Bits 14:10 SRAM3PDSx: SRAM3 64-Kbyte page x (x = 13 to 9) power-down in all Stop modes
0: SRAM3 page x content retained in Stop modes
1: SRAM3 page x content lost in Stop modes
Bit 9 Reserved, must be kept at reset value.
Bits 8:0 SRAM1PDSx: SRAM1 64-Kbyte page x (x = 12 to 4) power-down in all Stop modes
0: SRAM1 page x content retained in Stop modes
1: SRAM1 page x content lost in Stop modes

10.10.42 PWR control register 5 (PWR_CR5)
Address offset: 0xAC
Reset value: 0x0000 0000
This register is protected against nonsecure access when LPMSEC = 1
in PWR_SECCFGR. This register is protected against unprivileged access when
LPMSEC = 1 and SPRIV = 1 in PWR_PRIVCFGR, or when LPMSEC = 0 and NSPRIV = 1.
Note:

Some bits are only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit
reserved and kept at reset value.

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

SRAM6 SRAM6 SRAM6 SRAM6 SRAM6 SRAM6 SRAM6 SRAM6
PDS8
PDS7
PDS6
PDS5
PDS4
PDS3
PDS2
PDS1
rw

rw

rw

rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 SRAM6PDSx: SRAM6 64-Kbyte page x (x = 8 to 1) power-down in all Stop modes
0: SRAM6 page x content retained in Stop modes
1: SRAM6 page x content lost in Stop modes

<!-- pagebreak -->

