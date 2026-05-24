RM0456 Rev 6

rw

rw

RM0456

Reset and clock control (RCC)

Bit 17 APB3DIS: APB3 clock disable
This bit can be set in order to further reduce power consumption, when none of the APB3
peripherals from RCC_APB3ENR are used and when their clocks are disabled in
RCC_APB3ENR. When this bit is set, all the APB3 peripherals clocks are off.
0: APB3 clock enabled, distributed to peripherals according to their dedicated clock enable
control bits
1: APB3 clock disabled
Bit 16 AHB3DIS: AHB3 clock disable
This bit can be set in order to further reduce power consumption, when none of the AHB3
peripherals (except SRAM4) are used and when their clocks are disabled in
RCC_AHB3ENR. When this bit is set, all the AHB3 peripherals clocks are off, except for
SRAM4.
0: AHB3 clock enabled, distributed to peripherals according to their dedicated clock enable
control bits
1: AHB3 clock disabled
Bits 15:7 Reserved, must be kept at reset value.
Bits 6:4 PPRE3[2:0]: APB3 prescaler
This bitfield is set and cleared by software to control the division factor of the APB3 clock
(PCLK3).
0xx: HCLK not divided
100: HCLK divided by 2
101: HCLK divided by 4
110: HCLK divided by 8
111: HCLK divided by 16
Bits 3:0 Reserved, must be kept at reset value.

11.8.9

RCC PLL1 configuration register (RCC_PLL1CFGR)
Address offset: 0x028
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

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

PLL1R
EN

PLL1Q
EN

PLL1P
EN

rw

rw

rw

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

Res.

Res.

Res.

PLL1F
RACEN

PLL1MBOOST[3:0]
rw

rw

rw

PLL1M[3:0]
rw

rw

rw

rw

rw

rw

PLL1RGE[1:0]

PLL1SRC[1:0]

rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bit 18 PLL1REN: PLL1 DIVR divider output enable
This bit is set and reset by software to enable the pll1_r_ck output of the PLL1. To save
power, PLL1RENPLL2REN and PLL1R bits must be set to 0 when pll1_r_ck is not used.
This bit can be cleared only when the PLL1 is not used as SYSCLK.
0: pll1_r_ck output disabled
1: pll1_r_ck output enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 17 PLL1QEN: PLL1 DIVQ divider output enable
This bit is set and reset by software to enable the pll1_q_ck output of the PLL1. To save
power, PLL1QEN and PLL1Q bits must be set to 0 when pll1_q_ck is not used.
0: pll1_q_ck output disabled
1: pll1_q_ck output enabled
Bit 16 PLL1PEN: PLL1 DIVP divider output enable
This bit is set and reset by software to enable the pll1_p_ck output of the PLL1. To save
power, PLL1PEN and PLL1P bits must be set to 0 when pll1_p_ck is not used.
0: pll1_p_ck output disabled
1: pll1_p_ck output enabled
Bits 15:12 PLL1MBOOST[3:0]: Prescaler for EPOD booster input clock
This bitfield is set and cleared by software to configure the prescaler of the PLL1, used for
the EPOD booster. The EPOD booster input frequency is
PLL1 input clock frequency/PLL1MBOOST.
This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0) and
EPODboost mode is disabled (see Section 10: Power control (PWR)).
0000: division by 1 (bypass)
0001: division by 2
0010: division by 4
0011: division by 6
0100: division by 8
0101: division by 10
0110: division by 12
0111: division by 14
1000: division by 16
others: reserved
Bits 11:8 PLL1M[3:0]: Prescaler for PLL1
This bitfield is set and cleared by software to configure the prescaler of the PLL1. The VCO1
input frequency is PLL1 input clock frequency/PLL1M.
This bit can be written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
0000: division by 1 (bypass)
0001: division by 2
0010: division by 3
...
1111: division by 16
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 PLL1FRACEN: PLL1 fractional latch enable
This bit is set and reset by software to latch the content of PLL1FRACN in the Σ∆ modulator.
In order to latch the PLL1FRACN value into the Σ∆ modulator, PLL1FRACEN must be
set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL1FRACN into the
modulator (see PLL initialization phase for details).
Bits 3:2 PLL1RGE[1:0]: PLL1 input frequency range
This bit is set and reset by software to select the proper reference frequency range used for
PLL1. It must be written before enabling the PLL1.
00-01-10: PLL1 input (ref1_ck) clock range frequency between 4 and 8 MHz
11: PLL1 input (ref1_ck) clock range frequency between 8 and 16 MHz

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 1:0 PLL1SRC[1:0]: PLL1 entry clock source
This bitfield is set and cleared by software to select PLL1 clock source. It can be written only
when the PLL1 is disabled. In order to save power, when no PLL1 is used, this bitfield value
must be zero.
00: No clock sent to PLL1
01: MSIS clock selected as PLL1 clock entry
10: HSI16 clock selected as PLL1 clock entry
11: HSE clock selected as PLL1 clock entry

11.8.10

RCC PLL2 configuration register (RCC_PLL2CFGR)
Address offset: 0x02C
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

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

PLL2R
EN

PLL2Q
EN

PLL2P
EN

rw

rw

rw

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

Res.

PLL2F
RACEN

Res.

Res.

Res.

Res.

PLL2M[3:0]
rw

rw

rw

Res.
rw

Res.

rw

PLL2RGE[1:0]

PLL2SRC[1:0]

rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.
Bit 18 PLL2REN: PLL2 DIVR divider output enable
This bit is set and reset by software to enable the pll2_r_ck output of the PLL2. To save
power, PLL2REN and PLL2R bits must be set to 0 when pll2_r_ck is not used.
0: pll2_r_ck output disabled
1: pll2_r_ck output enabled
Bit 17 PLL2QEN: PLL2 DIVQ divider output enable
This bit is set and reset by software to enable the pll2_q_ck output of the PLL2. To save
power, PLL2QEN and PLL2Q bits must be set to 0 when pll2_q_ck is not used.
0: pll2_q_ck output disabled
1: pll2_q_ck output enabled
Bit 16 PLL2PEN: PLL2 DIVP divider output enable
This bit is set and reset by software to enable the pll2_p_ck output of the PLL2. To save
power, PLL2PEN and PLL2P bits must be set to 0 when pll2_p_ck is not used.
0: pll2_p_ck output disabled
1: pll2_p_ck output enabled
Bits 15:12 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

