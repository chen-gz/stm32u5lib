609

Reset and clock control (RCC)

RM0456

Bits 1:0 SW[1:0]: system clock switch
This bitfield is set and cleared by software to select system clock source (SYSCLK). It is
configured by hardware to force MSIS oscillator selection when exiting Standby or Shutdown
mode. This bitfield is configured by hardware to force MSIS or HSI16 oscillator selection
when exiting Stop mode or in case of HSE oscillator failure, depending on STOPWUCK.
00: MSIS selected as system clock
01: HSI16 selected as system clock
10: HSE selected as system clock
11: PLL pll1_r_ck selected as system clock

11.8.7

RCC clock configuration register 2 (RCC_CFGR2)
Address offset: 0x020
Reset value: 0x0000 0000 (for STM32U535/545/575/585)
Reset value: 0x0000 6000 (for STM32U59x/5Ax/5Fx/5Gx)
Access: word, half-word, and byte access
From 0 to 15 wait states are inserted if the access occurs when the APB or AHB prescalers
values update is on going.

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

Res.

Res.

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

Res.

DPRE[2:0]
rw

rw

Res.
rw

PPRE2[2:0]
rw

rw

Res.
rw

20

19

rw

17

16

APB2D APB1D AHB2D AHB2D AHB1D
IS
IS
IS2
IS1
IS
rw

rw

rw

rw

rw

4

3

2

1

0

PPRE1[2:0]
rw

18

HPRE[3:0]
rw

rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 APB2DIS: APB2 clock disable
This bit can be set in order to further reduce power consumption, when none of the APB2
peripherals are used and when their clocks are disabled in RCC_APB2ENR. When this bit is
set, all APB2 peripherals clocks are off.
0: APB2 clock enabled, distributed to peripherals according to their dedicated clock enable
control bits
1: APB2 clock disabled
Bit 19 APB1DIS: APB1 clock disable
This bit can be set in order to further reduce power consumption, when none of the APB1
peripherals (except IWDG) are used and when their clocks are disabled in RCC_APB1ENR.
When this bit is set, all the APB1 peripherals clocks are off, except for IWDG.
0: APB1 clock enabled, distributed to peripherals according to their dedicated clock enable
control bits
1: APB1 clock disabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 18 AHB2DIS2: AHB2_2 clock disable
This bit can be set in order to further reduce power consumption, when none of the AHB2
peripherals from RCC_AHB2ENR2 are used and when their clocks are disabled in
RCC_AHB2ENR2. When this bit is set, all the AHB2 peripherals clocks from
RCC_AHB2ENR2 are off.
0: AHB2_2 clock enabled, distributed to peripherals according to their dedicated clock
enable control bits
1: AHB2_2 clock disabled
Bit 17 AHB2DIS1: AHB2_1 clock disable
This bit can be set in order to further reduce power consumption, when none of the AHB2
peripherals from RCC_AHB2ENR1 (except SRAM2 and SRAM3) are used and when their
clocks are disabled in RCC_AHB2ENR1. When this bit is set, all the AHB2 peripherals
clocks from RCC_AHB2ENR1 are off, except for SRAM2 and SRAM3.
0: AHB2_1 clock enabled, distributed to peripherals according to their dedicated clock
enable control bits
1: AHB2_1 clock disabled
Bit 16 AHB1DIS: AHB1 clock disable
This bit can be set in order to further reduce power consumption, when none of the AHB1
peripherals (except those listed hereafter) are used and when their clocks are disabled in
RCC_AHB1ENR. When this bit is set, all the AHB1 peripherals clocks are off, except for
FLASH, BKPSRAM, ICACHE, DCACHE1 and SRAM1.
0: AHB1 clock enabled, distributed to peripherals according to their dedicated clock enable
control bits
1: AHB1 clock disabled
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 DPRE[2:0]: DSI PHY prescaler
This bitfield is set and cleared by software to control the division factor of DSI PHY bus clock
(DCLK).
0xx: DCLK not divided
100: DCLK divided by 2
101: DCLK divided by 4
110: DCLK divided by 8
111: DCLK divided by 16
Note: This bitfield is only available on some devices in the STM32U5 Series. Refer to the
device datasheet for availability of its associated peripheral. If not present, consider this
bitfield as reserved and keep it at reset value.
Bit 11 Reserved, must be kept at reset value.
Bits 10:8 PPRE2[2:0]: APB2 prescaler
This bitfield is set and cleared by software to control the division factor of APB2 clock
(PCLK2).
0xx: PCLK2 not divided
100: PCLK2 divided by 2
101: PCLK2 divided by 4
110: PCLK2 divided by 8
111: PCLK2 divided by 16
Bit 7 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

