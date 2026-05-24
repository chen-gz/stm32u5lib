609

Reset and clock control (RCC)

RM0456

Bits 11:8 PLL2M[3:0]: Prescaler for PLL2
This bitfield is set and cleared by software to configure the prescaler of the PLL2. The VCO2
input frequency is PLL2 input clock frequency/PLL2M.
This bit can be written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
0000: division by 1 (bypass)
0001: division by 2
0010: division by 3
...
1111: division by 16
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 PLL2FRACEN: PLL2 fractional latch enable
This bit is set and reset by software to latch the content of PLL2FRACN in the Σ∆ modulator.
In order to latch the PLL2FRACN value into the Σ∆ modulator, PLL2FRACEN must be
set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL2FRACN into the
modulator (see PLL initialization phase for details).
Bits 3:2 PLL2RGE[1:0]: PLL2 input frequency range
This bitfield is set and reset by software to select the proper reference frequency range used
for PLL2. It must be written before enabling the PLL2.
00-01-10: PLL2 input (ref2_ck) clock range frequency between 4 and 8 MHz
11: PLL2 input (ref2_ck) clock range frequency between 8 and 16 MHz
Bits 1:0 PLL2SRC[1:0]: PLL2 entry clock source
This bitfield is set and cleared by software to select PLL2 clock source. It can be written only
when the PLL2 is disabled. To save power, when no PLL2 is used, this bitfield value must
be zero.
00: No clock sent to PLL2
01: MSIS clock selected as PLL2 clock entry
10: HSI16 clock selected as PLL2 clock entry
11: HSE clock selected as PLL2 clock entry

11.8.11

RCC PLL3 configuration register (RCC_PLL3CFGR)
Address offset: 0x030
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

PLL3R
EN

PLL3Q
EN

PLL3P
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

Res.

Res.

Res.

Res.

PLL3F
RACEN

PLL3M[3:0]
rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

PLL3RGE[1:0]

PLL3SRC[1:0]

rw

rw

rw

rw

RM0456

Reset and clock control (RCC)

Bit 18 PLL3REN: PLL3 DIVR divider output enable
This bit is set and reset by software to enable the pll3_r_ck output of the PLL3. To save
power, PLL3REN and PLL3R bits must be set to 0 when pll3_r_ck is not used.
0: pll3_r_ck output disabled
1: pll3_r_ck output enabled
Bit 17 PLL3QEN: PLL3 DIVQ divider output enable
This bit is set and reset by software to enable the pll3_q_ck output of the PLL3. To save
power, PLL3QEN and PLL3Q bits must be set to 0 when pll3_q_ck is not used.
0: pll3_q_ck output disabled
1: pll3_q_ck output enabled
Bit 16 PLL3PEN: PLL3 DIVP divider output enable
This bit is set and reset by software to enable the pll3_p_ck output of the PLL3. To save
power, PLL3PEN and PLL3P bits must be set to 0 when pll3_p_ck is not used.
0: pll3_p_ck output disabled
1: pll3_p_ck output enabled
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:8 PLL3M[3:0]: Prescaler for PLL3
This bitfield is set and cleared by software to configure the prescaler of the PLL3. The VCO3
input frequency is PLL3 input clock frequency/PLL3M. This bitfield can be written only when
the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
0000: division by 1 (bypass)
0001: division by 2
0010: division by 3
...
1111: division by 16
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 PLL3FRACEN: PLL3 fractional latch enable
This bit is set and reset by software to latch the content of PLL3FRACN in the Σ∆ modulator.
In order to latch the PLL3FRACN value into the Σ∆ modulator, PLL3FRACEN must be
set to 0, then set to 1: the transition 0 to 1 transfers the content of PLL3FRACN into the
modulator (see PLL initialization phase for details).
Bits 3:2 PLL3RGE[1:0]: PLL3 input frequency range
This bit is set and reset by software to select the proper reference frequency range used
for PLL3. It must be written before enabling the PLL3.
00-01-10: PLL3 input (ref3_ck) clock range frequency between 4 and 8 MHz
11: PLL3 input (ref3_ck) clock range frequency between 8 and 16 MHz
Bits 1:0 PLL3SRC[1:0]: PLL3 entry clock source
This bitfield is set and cleared by software to select PLL3 clock source. It can be written only
when the PLL3 is disabled. To save power, when no PLL3 is used, this bitfield value must
be zero.
00: No clock sent to PLL3
01: MSIS clock selected as PLL3 clock entry
10: HSI16 clock selected as PLL3 clock entry
11: HSE clock selected as PLL3 clock entry

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.12

RM0456

RCC PLL1 dividers register (RCC_PLL1DIVR)
Address offset: 0x034
Reset value: 0x0101 0280
Access: no wait state; word, half-word, and byte access

31

30

29

28

Res.

27

26

25

24

PLL1R[6:0]

15

23

22

21

rw

rw

rw

rw

rw

rw

rw

14

13

12

11

10

9

8

7

rw

rw

rw

19

18

17

16

PLL1Q[6:0]
rw

rw

rw

rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

PLL1P[6:0]
rw

20

Res.

PLL1N[8:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:24 PLL1R[6:0]: PLL1 DIVR division factor
This bitfield is set and reset by software to control frequency of the pll1_r_ck clock. It can be written
only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0). Only division by one and even
division factors are allowed.
0000000: pll1_r_ck = vco1_ck
0000001: pll1_r_ck = vco1_ck / 2 (default after reset)
0000010: reserved
0000011: pll1_r_ck = vco1_ck / 4
...
1111111: pll1_r_ck = vco1_ck / 128
Bit 23 Reserved, must be kept at reset value.
Bits 22:16 PLL1Q[6:0]: PLL1 DIVQ division factor
This bitfield is set and reset by software to control the frequency of the pll1_q_ck clock. It can be
written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
0000000: pll1_q_ck = vco1_ck
0000001: pll1_q_ck = vco1_ck / 2 (default after reset)
0000010: pll1_q_ck = vco1_ck / 3
0000011: pll1_q_ck = vco1_ck / 4
...
1111111: pll1_q_ck = vco1_ck / 128
Bits 15:9 PLL1P[6:0]: PLL1 DIVP division factor
This bitfield is set and reset by software to control the frequency of the pll1_p_ck clock. It can be
written only when the PLL1 is disabled (PLL1ON = 0 and PLL1RDY = 0).
0000000: pll1_p_ck = vco1_ck
0000001: pll1_p_ck = vco1_ck / 2 (default after reset)
0000010: pll1_p_ck = vco1_ck / 3
0000011: pll1_p_ck = vco1_ck / 4
...
1111111: pll1_p_ck = vco1_ck / 128

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 8:0 PLL1N[8:0]: Multiplication factor for PLL1 VCO
This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be
written only when the PLL is disabled (PLL1ON = 0 and PLL1RDY = 0).
0x003: PLL1N = 4
0x004: PLL1N = 5
0x005: PLL1N = 6
...
0x080: PLL1N = 129 (default after reset)
...
0x1FF: PLL1N = 512
Others: reserved
VCO output frequency = Fref1_ck x PLL1N, when fractional value 0 has been loaded in PLL1FRACN,
with:
–
PLL1N between 4 and 512
–
input frequency Fref1_ck between 4 and 16 MHz

11.8.13

RCC PLL1 fractional divider register (RCC_PLL1FRACR)
Address offset: 0x038
Reset value: 0x0000 0000
Access: no wait state; word and half-word access

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

PLL1FRACN[12:0]
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

2

1

0

Res.

Res.

Res.

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:3 PLL1FRACN[12:0]: Fractional part of the multiplication factor for PLL1 VCO
This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor.
It can be written at any time, allowing dynamic fine-tuning of the PLL1 VCO.
VCO output frequency = Fref1_ck x (PLL1N + (PLL1FRACN / 213)), with:
–
PLL1N must be between 4 and 512.
–
PLL1FRACN can be between 0 and 213- 1.
–
The input frequency Fref1_ck must be between 4 and 16 MHz.
To change the FRACN value on-the-fly even if the PLL is enabled, the application must proceed
as follows:
–
Set PLL1FRACEN = 0.
–
Write the new fractional value into PLL1FRACN.
–
Set PLL1FRACEN = 1.
Bits 2:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.14

RM0456

RCC PLL2 dividers configuration register (RCC_PLL2DIVR)
Address offset: 0x03C
Reset value: 0x0101 0280
Access: no wait state; word, half-word, and byte access

31

30

29

28

Res.

27

26

25

24

PLL2R[6:0]

15

23

22

21

rw

rw

rw

rw

rw

rw

rw

14

13

12

11

10

9

8

7

rw

rw

rw

19

18

17

16

PLL2Q[6:0]
rw

rw

rw

rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

PLL2P[6:0]
rw

20

Res.

PLL2N[8:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:24 PLL2R[6:0]: PLL2 DIVR division factor
This bitfield is set and reset by software to control the frequency of the pll2_r_ck clock. It can be
written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
0000000: pll2_r_ck = vco2_ck
0000001: pll2_r_ck = vco2_ck / 2 (default after reset)
0000010: pll2_r_ck = vco2_ck / 3
0000011: pll2_r_ck = vco2_ck / 4
...
1111111: pll2_r_ck = vco2_ck / 128
Bit 23 Reserved, must be kept at reset value.
Bits 22:16 PLL2Q[6:0]: PLL2 DIVQ division factor
This bitfield is set and reset by software to control the frequency of the pll2_q_ck clock. It can be
written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
0000000: pll2_q_ck = vco2_ck
0000001: pll2_q_ck = vco2_ck / 2 (default after reset)
0000010: pll2_q_ck = vco2_ck / 3
0000011: pll2_q_ck = vco2_ck / 4
...
1111111: pll2_q_ck = vco2_ck / 128
Bits 15:9 PLL2P[6:0]: PLL2 DIVP division factor
This bitfield is set and reset by software to control the frequency of the pll2_p_ck clock. It can be
written only when the PLL2 is disabled (PLL2ON = 0 and PLL2RDY = 0).
0000000: pll2_p_ck = vco2_ck
0000001: pll2_p_ck = vco2_ck / 2 (default after reset)
0000010: pll2_p_ck = vco2_ck / 3
0000011: pll2_p_ck = vco2_ck / 4
...
1111111: pll2_p_ck = vco2_ck / 128

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 8:0 PLL2N[8:0]: Multiplication factor for PLL2 VCO
This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be
written only when the PLL is disabled (PLL2ON = 0 and PLL2RDY = 0).
0x003: PLL2N = 4
0x004: PLL2N = 5
0x005: PLL2N = 6
...
0x080: PLL2N = 129 (default after reset)
...
0x1FF: PLL2N = 512
Others: reserved
VCO output frequency = Fref2_ck x PLL2N, when fractional value 0 has been loaded in PLL2FRACN,
with:
–
PLL2N between 4 and 512
–
input frequency Fref2_ck between 1MHz and 16MHz

11.8.15

RCC PLL2 fractional divider register (RCC_PLL2FRACR)
Address offset: 0x040
Reset value: 0x0000 0000
Access: no wait state; word and half-word access

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

PLL2FRACN[12:0]
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

2

1

0

Res.

Res.

Res.

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:3 PLL2FRACN[12:0]: Fractional part of the multiplication factor for PLL2 VCO
This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor.
It can be written at any time, allowing dynamic fine-tuning of the PLL2 VCO.
VCO output frequency = Fref2_ck x (PLL2N + (PLL2FRACN / 213)), with
–
PLL2N must be between 4 and 512.
–
PLL2FRACN can be between 0 and 213 - 1.
–
The input frequency Fref2_ck must be between 4 and 16 MHz.
In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must
proceed as follows:
–
Set the bit PLL2FRACEN to 0.
–
Write the new fractional value into PLL2FRACN.
–
Set the bit PLL2FRACEN to 1.
Bits 2:0

Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.16

RM0456

RCC PLL3 dividers configuration register (RCC_PLL3DIVR)
Address offset: 0x044
Reset value: 0x0101 0280
Access: no wait state; word, half-word, and byte access

31

30

29

28

Res.

27

26

25

24

PLL3R[6:0]

15

23

22

21

rw

rw

rw

rw

rw

rw

rw

14

13

12

11

10

9

8

7

rw

rw

rw

19

18

17

16

PLL3Q[6:0]
rw

rw

rw

rw

rw

rw

rw

6

5

4

3

2

1

0

rw

rw

rw

rw

PLL3P[6:0]
rw

20

Res.

PLL3N[8:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:24 PLL3R[6:0]: PLL3 DIVR division factor
This bitfield is set and reset by software to control the frequency of the pll3_r_ck clock. It can be
written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
0000000: pll3_r_ck = vco3_ck
0000001: pll3_r_ck = vco3_ck / 2 (default after reset)
0000010: pll3_r_ck = vco3_ck / 3
0000011: pll3_r_ck = vco3_ck / 4
...
1111111: pll3_r_ck = vco3_ck / 128
Bit 23 Reserved, must be kept at reset value.
Bits 22:16 PLL3Q[6:0]: PLL3 DIVQ division factor
This bitfield is set and reset by software to control the frequency of the pll3_q_ck clock. It can be
written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
0000000: pll3_q_ck = vco3_ck
0000001: pll3_q_ck = vco3_ck / 2 (default after reset)
0000010: pll3_q_ck = vco3_ck / 3
0000011: pll3_q_ck = vco3_ck / 4
...
1111111: pll3_q_ck = vco3_ck / 128
Bits 15:9 PLL3P[6:0]: PLL3 DIVP division factor
This bitfield is set and reset by software to control the frequency of the pll3_p_ck clock. It can be
written only when the PLL3 is disabled (PLL3ON = 0 and PLL3RDY = 0).
0000000: pll3_p_ck = vco3_ck
0000001: pll3_p_ck = vco3_ck / 2 (default after reset)
0000010: pll3_p_ck = vco3_ck / 3
0000011: pll3_p_ck = vco3_ck / 4
...
1111111: pll3_p_ck = vco3_ck / 128

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bits 8:0 PLL3N[8:0]: Multiplication factor for PLL3 VCO
This bitfield is set and reset by software to control the multiplication factor of the VCO. It can be
written only when the PLL is disabled (PLL3ON = 0 and PLL3RDY = 0).
0x003: PLL3N = 4
0x004: PLL3N = 5
0x005: PLL3N = 6
...
0x080: PLL3N = 129 (default after reset)
...
0x1FF: PLL3N = 512
Others: reserved
VCO output frequency = Fref3_ck x PLL3N, when fractional value 0 has been loaded in PLL3FRACN,
with:
–
PLL3N between 4 and 512
–
input frequency Fref3_ck between 4 and 16MHz

11.8.17

RCC PLL3 fractional divider register (RCC_PLL3FRACR)
Address offset: 0x048
Reset value: 0x0000 0000
Access: no wait state; word and half-word access

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

PLL3FRACN[12:0]
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

2

1

0

Res.

Res.

Res.

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:3 PLL3FRACN[12:0]: Fractional part of the multiplication factor for PLL3 VCO
This bitfield is set and reset by software to control the fractional part of the VCO multiplication factor.
It can be written at any time, allowing dynamic fine-tuning of the PLL3 VCO.
VCO output frequency = Fref3_ck x (PLL3N + (PLL3FRACN / 213)), with:
–
PLL3N must be between 4 and 512.
–
PLL3FRACN can be between 0 and 213 - 1.
–
The input frequency Fref3_ck must be between 4 and 16 MHz.
In order to change the FRACN value on-the-fly even if the PLL is enabled, the application must
proceed as follows:
–
Set the bit PLL3FRACEN to 0.
–
Write the new fractional value into PLL3FRACN.
–
Set the bit PLL3FRACEN to 1.
Bits 2:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.18

RM0456

RCC clock interrupt enable register (RCC_CIER)
Address offset: 0x050
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

SHSIR
DYIE

MSIKR
DYIE

Res.

PLL3R
DYIE

PLL2R
DYIE

rw

rw

rw

rw

Res.

Res.

Res.

PLL1R HSI48R HSERD HSIRD MSISR LSERD LSIRD
DYIE
DYIE
YIE
YIE
DYIE
YIE
YIE
rw

rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 SHSIRDYIE: SHSI ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the SHSI
oscillator stabilization.
0: SHSI ready interrupt disabled
1: SHSI ready interrupt enabled
Bit 11 MSIKRDYIE: MSIK ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the MSIK
oscillator stabilization.
0: MSIK ready interrupt disabled
1: MSIK ready interrupt enabled
Bits 10:9 Reserved, must be kept at reset value.
Bit 8 PLL3RDYIE: PLL3 ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by PLL3 lock.
0: PLL3 lock interrupt disabled
1: PLL3 lock interrupt enabled
Bit 7 PLL2RDYIE: PLL2 ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by PLL2 lock.
0: PLL2 lock interrupt disabled
1: PLL2 lock interrupt enabled
Bit 6 PLL1RDYIE: PLL ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by PLL1 lock.
0: PLL1 lock interrupt disabled
1: PLL1 lock interrupt enabled
Bit 5 HSI48RDYIE: HSI48 ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the HSI48
oscillator stabilization.
0: HSI48 ready interrupt disabled
1: HSI48 ready interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Reset and clock control (RCC)

Bit 4 HSERDYIE: HSE ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the HSE
oscillator stabilization.
0: HSE ready interrupt disabled
1: HSE ready interrupt enabled
Bit 3 HSIRDYIE: HSI16 ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the HSI16
oscillator stabilization.
0: HSI16 ready interrupt disabled
1: HSI16 ready interrupt enabled
Bit 2 MSISRDYIE: MSIS ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the MSIS
oscillator stabilization.
0: MSIS ready interrupt disabled
1: MSIS ready interrupt enabled
Bit 1 LSERDYIE: LSE ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the LSE
oscillator stabilization.
0: LSE ready interrupt disabled
1: LSE ready interrupt enabled
Bit 0 LSIRDYIE: LSI ready interrupt enable
This bit is set and cleared by software to enable/disable interrupt caused by the LSI
oscillator stabilization.
0: LSI ready interrupt disabled
1: LSI ready interrupt enabled

11.8.19

RCC clock interrupt flag register (RCC_CIFR)
Address offset: 0x054
Reset value: 0x0000 0000
Access: no wait state, word; half-word, and byte access

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

SHSIR
DYF

MSIKR
DYF

CSSF

Res.

PLL3R
DYF

PLL2R
DYF

r

r

r

r

r

Res.

Res.

PLL1R HSI48R HSERD HSIRD MSISR LSERD LSIRD
DYF
DYF
YF
YF
DYF
YF
YF
r

r

r

r

r

r

r

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 SHSIRDYF: SHSI ready interrupt flag
This bit is set by hardware when the SHSI clock becomes stable and SHSIRDYIE is set. It is
cleared by software by setting the SHSIRDYC bit.
0: No clock ready interrupt caused by the SHSI oscillator
1: Clock ready interrupt caused by the SHSI oscillator

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 11 MSIKRDYF: MSIK ready interrupt flag
This bit is set by hardware when the MSIK clock becomes stable and MSIKRDYIE is set. It is
cleared by software by setting the MSIKRDYC bit.
0: No clock ready interrupt caused by the MSIK oscillator
1: Clock ready interrupt caused by the MSIK oscillator
Bit 10 CSSF: Clock security system interrupt flag
This bit is set by hardware when a failure is detected in the HSE oscillator. It is cleared by
software by setting the CSSC bit.
0: No clock security interrupt caused by HSE clock failure
1: Clock security interrupt caused by HSE clock failure
Bit 9 Reserved, must be kept at reset value.
Bit 8 PLL3RDYF: PLL3 ready interrupt flag
This bit is set by hardware when the PLL3 locks and PLL3RDYIE is set. It is cleared by
software by setting the PLL3RDYC bit.
0: No clock ready interrupt caused by PLL3 lock
1: Clock ready interrupt caused by PLL3 lock
Bit 7 PLL2RDYF: PLL2 ready interrupt flag
This bit is set by hardware when the PLL2 locks and PLL2RDYIE is set. It is cleared by
software by setting the PLL2RDYC bit.
0: No clock ready interrupt caused by PLL2 lock
1: Clock ready interrupt caused by PLL2 lock
Bit 6 PLL1RDYF: PLL1 ready interrupt flag
This bit is set by hardware when the PLL1 locks and PLL1RDYIE is set. It is cleared by
software by setting the PLL1RDYC bit.
0: No clock ready interrupt caused by PLL1 lock
1: Clock ready interrupt caused by PLL1 lock
Bit 5 HSI48RDYF: HSI48 ready interrupt flag
This bit is set by hardware when the HSI48 clock becomes stable and HSI48RDYIE is set.
it is cleared by software by setting the HSI48RDYC bit.
0: No clock ready interrupt caused by the HSI48 oscillator
1: Clock ready interrupt caused by the HSI48 oscillator
Bit 4 HSERDYF: HSE ready interrupt flag
This bit is set by hardware when the HSE clock becomes stable and HSERDYIE is set. It is
cleared by software by setting the HSERDYC bit.
0: No clock ready interrupt caused by the HSE oscillator
1: Clock ready interrupt caused by the HSE oscillator
Bit 3 HSIRDYF: HSI16 ready interrupt flag
This bit is set by hardware when the HSI16 clock becomes stable and HSIRDYIE = 1
in response to setting the HSION (see RCC_CR). When HSION = 0 but the HSI16 oscillator
is enabled by the peripheral through a clock request, this bit is not set and no interrupt is
generated. This bit is cleared by software by setting the HSIRDYC bit.
0: No clock ready interrupt caused by the HSI16 oscillator
1: Clock ready interrupt caused by the HSI16 oscillator
Bit 2 MSISRDYF: MSIS ready interrupt flag
This bit is set by hardware when the MSIS clock becomes stable and MSISRDYIE is set.
It is cleared by software by setting the MSISRDYC bit.
0: No clock ready interrupt caused by the MSIS oscillator
1: Clock ready interrupt caused by the MSIS oscillator

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 1 LSERDYF: LSE ready interrupt flag
This bit is set by hardware when the LSE clock becomes stable and LSERDYIE is set. It is
cleared by software by setting the LSERDYC bit.
0: No clock ready interrupt caused by the LSE oscillator
1: Clock ready interrupt caused by the LSE oscillator
Bit 0 LSIRDYF: LSI ready interrupt flag
This bit is set by hardware when the LSI clock becomes stable and LSIRDYIE is set. It is
cleared by software by setting the LSIRDYC bit.
0: No clock ready interrupt caused by the LSI oscillator
1: Clock ready interrupt caused by the LSI oscillator

11.8.20

RCC clock interrupt clear register (RCC_CICR)
Address offset: 0x058
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

SHSIR
DYC

MSIKR
DYC

CSSC

Res.

PLL3R
DYC

PLL2R
DYC

w

w

w

w

w

Res.

Res.

PLL1R HSI48R HSERD HSIRD MSISR LSERD LSIRD
DYC
DYC
YC
YC
DYC
YC
YC
w

w

w

w

w

w

w

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 SHSIRDYC: SHSI oscillator ready interrupt clear
Writing this bit to 1 clears the SHSIRDYF flag. Writing 0 has no effect.
Bit 11 MSIKRDYC: MSIK oscillator ready interrupt clear
Writing this bit to 1 clears the MSIKRDYF flag. Writing 0 has no effect.
Bit 10 CSSC: Clock security system interrupt clear
Writing this bit to 1 clears the CSSF flag. Writing 0 has no effect.
Bit 9 Reserved, must be kept at reset value.
Bit 8 PLL3RDYC: PLL3 ready interrupt clear
Writing this bit to 1 clears the PLL3RDYF flag. Writing 0 has no effect.
Bit 7 PLL2RDYC: PLL2 ready interrupt clear
Writing this bit to 1 clears the PLL2RDYF flag. Writing 0 has no effect.
Bit 6 PLL1RDYC: PLL1 ready interrupt clear
Writing this bit to 1 clears the PLL1RDYF flag. Writing 0 has no effect.
Bit 5 HSI48RDYC: HSI48 ready interrupt clear
Writing this bit to 1 clears the HSI48RDYF flag. Writing 0 has no effect.
Bit 4 HSERDYC: HSE ready interrupt clear
Writing this bit to 1 clears the HSERDYF flag. Writing 0 has no effect.
Bit 3 HSIRDYC: HSI16 ready interrupt clear
Writing this bit to 1 clears the HSIRDYF flag. Writing 0 has no effect.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 2 MSISRDYC: MSIS ready interrupt clear
Writing this bit to 1 clears the MSISRDYF flag. Writing 0 has no effect.
Bit 1 LSERDYC: LSE ready interrupt clear
Writing this bit to 1 clears the LSERDYF flag. Writing 0 has no effect.
Bit 0 LSIRDYC: LSI ready interrupt clear
Writing this bit to 1 clears the LSIRDYF flag. Writing 0 has no effect.

11.8.21

RCC AHB1 peripheral reset register (RCC_AHB1RSTR)
Address offset: 0x060
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

Res.

Res.

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

CRCR
ST

JPEGR
ST

Res.

rw

Res.

Res.

Res.

Res.

Res.

rw

Res.

Res.

20

19

18

17

16

GFXM
GPU2D
DMA2D RAMC TSCRS
MURS
RST
RST FGRST
T
T
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

Res.

MDF1R FMAC
ST
RST
rw

rw

CORDI GPDM
CRST A1RST
rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 GPU2DRST: GPU2D reset
This bit is set and cleared by software.
0: No effect
1: Reset the GPU2D.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 GFXMMURST: GFXMMU reset
This bit is set and cleared by software.
0: No effect
1: Reset the GFXMMU.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 DMA2DRST: DMA2D reset
This bit is set and cleared by software.
0: No effect
1: Reset the DMA2D.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 17 RAMCFGRST: RAMCFG reset
This bit is set and cleared by software.
0: No effect
1: Reset the RAMCFG.
Bit 16 TSCRST: TSC reset
This bit is set and cleared by software.
0: No effect
1: Reset the TSC.
Bit 15 JPEGRST: JPEG reset
This bit is set and cleared by software.
0: No effect
1: Reset the JPEG.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 14:13 Reserved, must be kept at reset value.
Bit 12 CRCRST: CRC reset
This bit is set and cleared by software.
0: No effect
1: Reset the CRC.
Bits 11:4 Reserved, must be kept at reset value.
Bit 3 MDF1RST: MDF1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the MDF1.
Bit 2 FMACRST: FMAC reset
This bit is set and cleared by software.
0: No effect
1: Reset the FMAC.
Bit 1 CORDICRST: CORDIC reset
This bit is set and cleared by software.
0: No effect
1: Reset the CORDIC.
Bit 0 GPDMA1RST: GPDMA1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the GPDMA1.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.22

RM0456

RCC AHB2 peripheral reset register 1 (RCC_AHB2RSTR1)
Address offset: 0x064
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

31

30

29

28

27

SDMM SDMM
C2RST C1RST

Res.

Res.

Res.

rw

rw

15

14

13

12

Res.

DCMI_
PSSIR
ST

Res.

OTGR
ST
rw

rw

26

25

Res.

Res.

11

10

9

Res.

ADC12
RST
rw

24

23

OTFDE OTFDE
C2RST C1RST
rw

rw

8

7

22
Res.

6

21

20

19

18

17

16

OCTOS
SAESR PKARS RNGR HASHR AESRS
PIMRS
ST
T
ST
ST
T
T
rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

GPIOJ GPIOIR GPIOH GPIOG GPIOF GPIOE GPIOD GPIOC GPIOB GPIOA
RST
ST
RST
RST
RST
RST
RST
RST
RST
RST
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

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 SDMMC2RST: SDMMC2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SDMMC2.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 27 SDMMC1RST: SDMMC1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SDMMC1.
Bits 26:25 Reserved, must be kept at reset value.
Bit 24 OTFDEC2RST: OTFDEC2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the OTFDEC2.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 23 OTFDEC1RST: OTFDEC1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the OTFDEC1.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 22 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 21 OCTOSPIMRST: OCTOSPIM reset
This bit is set and cleared by software.
0: No effect
1: Reset the OCTOSPIM.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 20 SAESRST: SAES hardware accelerator reset
This bit is set and cleared by software.
0: No effect
1: Reset the SAES.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 PKARST: PKA reset
This bit is set and cleared by software.
0: No effect
1: Reset the PKA.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 RNGRST: RNG reset
This bit is set and cleared by software.
0: No effect
1: Reset the RNG.
Bit 17 HASHRST: HASH reset
This bit is set and cleared by software.
0: No effect
1: Reset the HASH.
Bit 16 AESRST: AES hardware accelerator reset
This bit is set and cleared by software.
0: No effect
1: Reset the AES.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 OTGRST: OTG_FS or OTG_HS reset
This bit is set and cleared by software.
0: No effect
1: Reset the OTG_FS or OTG_HS.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 12 DCMI_PSSIRST: DCMI and PSSI reset
This bit is set and cleared by software.
0: No effect
1: Reset the DCMI and PSSI.
Bit 11 Reserved, must be kept at reset value.
Bit 10 ADC12RST: ADC1 and ADC2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the ADC1 and ADC2.
Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2
in STM32U59x/5Ax/5Fx/5Gx.
Bit 9 GPIOJRST: I/O port J reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port J.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 8 GPIOIRST: I/O port I reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port .I
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 7 GPIOHRST: I/O port H reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port H.
Bit 6 GPIOGRST: I/O port G reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port G.
Bit 5 GPIOFRST: I/O port F reset
This bit is set and cleared by software.
0: No effect
1: Reset I/O port F
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral.
If not present, consider this bit as reserved and keep it at reset value.
Bit 4 GPIOERST: I/O port E reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port E.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 3 GPIODRST: I/O port D reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port D.
Bit 2 GPIOCRST: I/O port C reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port C.
Bit 1 GPIOBRST: I/O port B reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port B.
Bit 0 GPIOARST: I/O port A reset
This bit is set and cleared by software.
0: No effect
1: Reset the I/O port A.

11.8.23

RCC AHB2 peripheral reset register 2 (RCC_AHB2RSTR2)
Address offset: 0x068
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

HSPI1
RST

Res.

Res.

Res.

OCTOS
PI2RST

Res.

Res.

Res.

OCTOS
PI1RST

Res.

Res.

Res.

FSMC
RST

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bit 12 HSPI1RST: HSPI1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the HSPI1.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 OCTOSPI2RST: OCTOSPI2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the OCTOSPI2.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bits 7:5 Reserved, must be kept at reset value.
Bit 4 OCTOSPI1RST: OCTOSPI1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the OCTOSPI1.
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 FSMCRST: Flexible memory controller reset
This bit is set and cleared by software.
0: No effect
1: Reset the FSMC
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

11.8.24

RCC AHB3 peripheral reset register (RCC_AHB3RSTR)
Address offset: 0x06C
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

LPGPI
O1RST

Res.

Res.

Res.

Res.

Res.

ADF1R LPDMA
ST
1RST
rw

Res.

Res.

rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bit 10 ADF1RST: ADF1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the ADF1.
Bit 9 LPDMA1RST: LPDMA1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPDMA1.
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 DAC1RST: DAC1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the DAC1.
Bit 5 ADC4RST: ADC4 reset
This bit is set and cleared by software.
0: No effect
1: Reset the ADC4 interface.

<!-- pagebreak -->

DAC1R ADC4R
ST
ST

RM0456 Rev 6

rw

Res.

Res.

Res.

rw

RM0456

Reset and clock control (RCC)

Bits 4:1 Reserved, must be kept at reset value.
Bit 0 LPGPIO1RST: LPGPIO1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPGPIO1.

11.8.25

RCC APB1 peripheral reset register 1 (RCC_APB1RSTR1)
Address offset: 0x074
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

31

30

29

28

27

26

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

Res.

SPI2R
ST

Res.

Res.

Res.

Res.

25

24

USART CRSRS
6RST
T
rw

rw

9

8

Res.

Res.

23
Res.

7
Res.

rw

22

21

20

19

18

17

16

I2C2RS I2C1RS UART5 UART4 USART USART
T
T
RST
RST
3RST
2RST

Res.

rw

rw

rw

rw

rw

6

5

4

3

2

rw
1

0

Res.

TIM7R
ST

TIM6R
ST

TIM5R
ST

TIM4R
ST

TIM3R
ST

TIM2R
ST

rw

rw

rw

rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 USART6RST: USART6 reset
This bit is set and cleared by software.
0: No effect
1: Reset the USART6.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 24 CRSRST: CRS reset
This bit is set and cleared by software.
0: No effect
1: Reset the CRS.
Bit 23 Reserved, must be kept at reset value.
Bit 22 I2C2RST: I2C2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the I2C2.
Bit 21 I2C1RST: I2C1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the I2C1.
Bit 20 UART5RST: UART5 reset
This bit is set and cleared by software.
0: No effect
1: Reset the UART5.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 19 UART4RST: UART4 reset
This bit is set and cleared by software.
0: No effect
1: Reset the UART4.
Bit 18 USART3RST: USART3 reset
This bit is set and cleared by software.
0: No effect
1: Reset the USART3.
Bit 17 USART2RST: USART2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the USART2
Note: This bit is only available on some devices in the STM32U5 Series.Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 16:15 Reserved, must be kept at reset value.
Bit 14 SPI2RST: SPI2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SPI2.
Bits 13:6 Reserved, must be kept at reset value.
Bit 5 TIM7RST: TIM7 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM7.
Bit 4 TIM6RST: TIM6 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM6.
Bit 3 TIM5RST: TIM5 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM5.
Bit 2 TIM4RST: TIM4 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM4.
Bit 1 TIM3RST: TIM3 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM3.
Bit 0 TIM2RST: TIM2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM2.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

11.8.26

RCC APB1 peripheral reset register 2 (RCC_APB1RSTR2)
Address offset: 0x078
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

UCPD1
RST

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

4

3

2

1

0

Res.

I2C4RS
T

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

Res.

FDCAN
1RST

rw

Res.

Res.

Res.

Res.

Res.

Res.

7

I2C6RS I2C5RS LPTIM2
T
T
RST

rw

rw

rw

rw

Res.

Res.

rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 UCPD1RST: UCPD1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the UCPD1.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 22:10 Reserved, must be kept at reset value.
Bit 9 FDCAN1RST: FDCAN1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the FDCAN1.
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 I2C6RST: I2C6 reset
This bit is set and cleared by software
0: No effect
1: Reset the I2C6.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 6 I2C5RST: I2C5 reset
This bit is set and cleared by software
0: No effect
1: Reset the I2C5.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 5 LPTIM2RST: LPTIM2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPTIM2.
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 I2C4RST: I2C4 reset
This bit is set and cleared by software
0: No effect
1: Reset the I2C4.
Bit 0 Reserved, must be kept at reset value.

11.8.27

RCC APB2 peripheral reset register (RCC_APB2RSTR)
Address offset: 0x07C
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

31

30

29

28

27

26

DSIRS LTDCR
T
ST

25

24

GFXTI USBRS
MRST
T

23

22

21

Res.

SAI2R
ST

SAI1R
ST

20

19

18

17

16

Res.

Res.

TIM17
RST

TIM16
RST

TIM15
RST

Res.

Res.

Res.

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

SPI1R
ST

TIM1R
ST

Res.

Res.

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

Res.

USART TIM8R
1RST
ST
rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 DSIRST: DSI reset
This bit is set and cleared by software.
0: No effect
1: Reset the DSI.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 26 LTDCRST: LTDC reset
This bit is set and cleared by software.
0: No effect
1: Reset the LTDC.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 25 GFXTIMRST: GFXTIM reset
This bit is set and cleared by software.
0: No effect
1: Reset the GFXTIM.
Note: .This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 24 USBRST: USB reset
This bit is set and cleared by software.
0: No effect
1: Reset the USB.
Note: This bit is only available on STM32U535/545 devices, it is reserved on other devices in
the STM32U5 Series. If not present, consider this bit as reserved and keep it at reset
value.
Bit 23 Reserved, must be kept at reset value.
Bit 22 SAI2RST: SAI2 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SAI2.
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 21 SAI1RST: SAI1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SAI1.
Bits 20:19 Reserved, must be kept at reset value.
Bit 18 TIM17RST: TIM17 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM17.
Bit 17 TIM16RST: TIM16 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM16.
Bit 16 TIM15RST: TIM15 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM15.
Bit 15 Reserved, must be kept at reset value.
Bit 14 USART1RST: USART1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the USART1.
Bit 13 TIM8RST: TIM8 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM8.
Bit 12 SPI1RST: SPI1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SPI1.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 11 TIM1RST: TIM1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the TIM1.
Bits 10:0 Reserved, must be kept at reset value.

11.8.28

RCC APB3 peripheral reset register (RCC_APB3RSTR)
Address offset: 0x080
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

VREFR
ST

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

3

2

1

0

Res.

SYSCF
GRST

Res.

rw

COMP OPAMP LPTIM4 LPTIM3 LPTIM1
RST
RST
RST
RST
RST
rw

rw

rw

rw

Res.

Res.

Res.

I2C3RS LPUAR
T
T1RST

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bit 20 VREFRST: VREFBUF reset
This bit is set and cleared by software.
0: No effect
1: Reset the VREFBUF.
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 COMPRST: COMP reset
This bit is set and cleared by software.
0: No effect
1: Reset the COMP.
Bit 14 OPAMPRST: OPAMP reset
This bit is set and cleared by software.
0: No effect
1: Reset the OPAMP.
Bit 13 LPTIM4RST: LPTIM4 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPTIM4.
Bit 12 LPTIM3RST: LPTIM3 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPTIM3.
Bit 11 LPTIM1RST: LPTIM1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPTIM1.

<!-- pagebreak -->

RM0456 Rev 6

rw

SPI3R
ST
rw

4
Res.

Res.

rw

RM0456

Reset and clock control (RCC)

Bits 10:8 Reserved, must be kept at reset value.
Bit 7 I2C3RST: I2C3 reset
This bit is set and cleared by software.
0: No effect
1: Reset the I2C3.
Bit 6 LPUART1RST: LPUART1 reset
This bit is set and cleared by software.
0: No effect
1: Reset the LPUART1.
Bit 5 SPI3RST: SPI3 reset
This bit is set and cleared by software.
0: No effect
1: Reset the SPI3.
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 SYSCFGRST: SYSCFG reset
This bit is set and cleared by software.
0: No effect
1: Reset the SYSCFG.
Bit 0 Reserved, must be kept at reset value.

11.8.29

RCC AHB1 peripheral clock enable register (RCC_AHB1ENR)
Address offset: 0x088
Reset value: 0xD000 0100 (for STM32U535/545/575/585)
Reset value: 0xD020 0100 (for STM32U59x/5Ax/5Fx/5Gx)
Access: no wait state; word, half-word, and byte access

Note:

31

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.
30

SRAM1 DCAC
EN
HE1EN
rw

rw

15

14

JPEGE
N
rw

Res.

29

28

27

26

25

24

23

22

Res.

BKPSR
AMEN

Res.

Res.

Res.

GTZC1
EN

Res.

Res.

13

12

11

10

9

8

7

6

Res.

CRCE
N

Res.

FLASH
EN

rw

rw

rw

Res.

Res.

Res.

rw

Res.

21

20

19

18

17

16

DCAC GPU2D GFXM DMA2D RAMC
TSCEN
HE2EN
EN
MUEN
EN
FGEN
rw

rw

rw

rw

rw

5

4

3

2

1

Res.

Res.

rw
0

MDF1E FMACE CORDI
N
N
CEN
rw

rw

rw

GPDM
A1EN
rw

Bit 31 SRAM1EN: SRAM1 clock enable
This bit is set and reset by software.
0: SRAM1 clock disabled
1: SRAM1 clock enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 30 DCACHE1EN: DCACHE1 clock enable
This bit is set and reset by software.
0: DCACHE1 clock disabled
1: DCACHE1 clock enabled
Note: DCACHE1 clock must be enabled when external memories are accessed through
OCTOSPI1, OCTOSPI2, HSPI1 or FSMC, even if the DCACHE1 is bypassed.
Bit 29 Reserved, must be kept at reset value.
Bit 28 BKPSRAMEN: BKPSRAM clock enable
This bit is set and reset by software.
0: BKPSRAM clock disabled
1: BKPSRAM clock enabled
Bits 27:25 Reserved, must be kept at reset value.
Bit 24 GTZC1EN: GTZC1 clock enable
This bit is set and reset by software.
0: GTZC1 clock disabled
1: GTZC1 clock enabled
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 DCACHE2EN: DCACHE2 clock enable
This bit is set and reset by software.
0: DCACHE2 clock disabled
1: DCACHE2 clock enabled
Note: DCACHE2 clock must be enabled to access memories, even if the DCACHE2 is
bypassed.
This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 20 GPU2DEN: GPU2D clock enable
This bit is set and cleared by software.
0: GPU2D clock disabled
1: GPU2D clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 GFXMMUEN: GFXMMU clock enable
This bit is set and cleared by software.
0: GFXMMU clock disabled
1: GFXMMU clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 DMA2DEN: DMA2D clock enable
This bit is set and cleared by software.
0: DMA2D clock disabled
1: DMA2D clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 17 RAMCFGEN: RAMCFG clock enable
This bit is set and cleared by software.
0: RAMCFG clock disabled
1: RAMCFG clock enabled
Bit 16 TSCEN: Touch sensing controller clock enable
This bit is set and cleared by software.
0: TSC clock disabled
1: TSC clock enabled
Bit 15 JPEGEN: JPEG clock enable
This bit is set and cleared by software.
0: JPEG clock disabled
1: JPEG clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 14:13 Reserved, must be kept at reset value.
Bit 12 CRCEN: CRC clock enable
This bit is set and cleared by software.
0: CRC clock disabled
1: CRC clock enabled
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 FLASHEN: FLASH clock enable
This bit is set and cleared by software. This bit can be disabled only when the flash memory
is in power-down mode.
0: FLASH clock disabled
1: FLASH clock enabled
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 MDF1EN: MDF1 clock enable
This bit is set and reset by software.
0: MDF1 clock disabled
1: MDF1 clock enabled
Bit 2 FMACEN: FMAC clock enable
This bit is set and reset by software.
0: FMAC clock disabled
1: FMAC clock enabled
Bit 1 CORDICEN: CORDIC clock enable
This bit is set and cleared by software.
0: CORDIC clock disabled
1: CORDIC clock enabled
Bit 0 GPDMA1EN: GPDMA1 clock enable
This bit is set and cleared by software.
0: GPDMA1 clock disabled
1: GPDMA1 clock enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.30

RM0456

RCC AHB2 peripheral clock enable register 1 (RCC_AHB2ENR1)
Address offset: 0x08C
Reset value: 0x4000 0000 (for STM32U535/545)
Reset value: 0xC000 0000 (for STM32U575/585/59x/5Ax/5Fx/5Gx)
Access: no wait state, word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.

31

30

SRAM3 SRAM2
EN
EN
rw

rw

15

14

OTGH
SPHYE
N

OTGE
N

rw

rw

29

28

27

Res.

SDMM
C2EN

SDMM
C1EN

rw

rw

13

12

Res.

DCMI_
PSSIE
N
rw

26

25

Res.

Res.

11

10

9

Res.

ADC12
EN
rw

24

23

OTFDE OTFDE
C2EN C1EN
rw

rw

8

7

22
Res.

6

21

20

19

18

17

16

OCTOS SAESE
RNGE HASHE
PKAEN
AESEN
PIMEN
N
N
N
rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

GPIOJ GPIOIE GPIOH GPIOG GPIOF GPIOE GPIOD GPIOC GPIOB GPIOA
EN
N
EN
EN
EN
EN
EN
EN
EN
EN
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

Bit 31 SRAM3EN: SRAM3 clock enable
This bit is set and reset by software.
0: SRAM3 clock disabled
1: SRAM3 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 30 SRAM2EN: SRAM2 clock enable
This bit is set and reset by software.
0: SRAM2 clock disabled
1: SRAM2 clock enabled
Bit 29 Reserved, must be kept at reset value.
Bit 28 SDMMC2EN: SDMMC2 clock enable
This bit is set and cleared by software.
0: SDMMC2 clock disabled
1: SDMMC2 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 27 SDMMC1EN: SDMMC1 clock enable
This bit is set and cleared by software.
0: SDMMC1 clock disabled
1: SDMMC1 clock enabled
Bits 26:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 24 OTFDEC2EN: OTFDEC2 clock enable
This bit is set and cleared by software.
0: OTFDEC2 clock disabled
1: OTFDEC2 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 23 OTFDEC1EN: OTFDEC1 clock enable
This bit is set and cleared by software.
0: OTFDEC1 clock disabled
1: OTFDEC1 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 22 Reserved, must be kept at reset value.
Bit 21 OCTOSPIMEN: OCTOSPIM clock enable
This bit is set and cleared by software.
0: OCTOSPIM clock disabled
1: OCTOSPIM clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 20 SAESEN: SAES clock enable
This bit is set and cleared by software.
0: SAES clock disabled
1: SAES clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 PKAEN: PKA clock enable
This bit is set and cleared by software.
0: PKA clock disabled
1: PKA clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 RNGEN: RNG clock enable
This bit is set and cleared by software.
0: RNG clock disabled
1: RNG clock enabled
Bit 17 HASHEN: HASH clock enable
This bit is set and cleared by software
0: HASH clock disabled
1: HASH clock enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 16 AESEN: AES clock enable
This bit is set and cleared by software.
0: AES clock disabled
1: AES clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 15 OTGHSPHYEN: OTG_HS PHY clock enable
This bit is set and cleared by software.
0: OTG_HS PHY clock disabled
1: OTG_HS PHY clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 14 OTGEN: OTG_FS or OTG_HS clock enable
This bit is set and cleared by software.
0: OTG_FS or OTG_HS clock disabled
1: OTG_FS or OTG_HS clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 13 Reserved, must be kept at reset value.
Bit 12 DCMI_PSSIEN: DCMI and PSSI clock enable
This bit is set and cleared by software.
0: DCMI and PSSI clock disabled
1: DCMI and PSSI clock enabled
Bit 11 Reserved, must be kept at reset value.
Bit 10 ADC12EN: ADC1 and ADC2 clock enable
This bit is set and cleared by software.
0: ADC1 and ADC2 clock disabled
1: ADC1 and ADC2 clock enabled
Note: This bit impacts ADC1 in STM32U535/545/575/585, and ADC1/ADC2
in STM32U59x/5Ax/5Fx/5Gx.
Bit 9 GPIOJEN: I/O port J clock enable
This bit is set and cleared by software.
0: I/O port J clock disabled
1: I/O port J clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 8 GPIOIEN: I/O port I clock enable
This bit is set and cleared by software.
0: I/O port I clock disabled
1: I/O port I clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 7 GPIOHEN: I/O port H clock enable
This bit is set and cleared by software.
0: I/O port H clock disabled
1: I/O port H clock enabled
Bit 6 GPIOGEN: I/O port G clock enable
This bit is set and cleared by software.
0: I/O port G clock disabled
1: I/O port G clock enabled
Bit 5 GPIOFEN: I/O port F clock enable
This bit is set and cleared by software.
0: I/O port F clock disabled
1: I/O port F clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 4 GPIOEEN: I/O port E clock enable
This bit is set and cleared by software.
0: I/O port E clock disabled
1: I/O port E clock enabled
Bit 3 GPIODEN: I/O port D clock enable
This bit is set and cleared by software.
0: I/O port D clock disabled
1: I/O port D clock enabled
Bit 2 GPIOCEN: I/O port C clock enable
This bit is set and cleared by software.
0: I/O port C clock disabled
1: I/O port C clock enabled
Bit 1 GPIOBEN: I/O port B clock enable
This bit is set and cleared by software.
0: I/O port B clock disabled
1: I/O port B clock enabled
Bit 0 GPIOAEN: I/O port A clock enable
This bit is set and cleared by software.
0: I/O port A clock disabled
1: I/O port A clock enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.31

RM0456

RCC AHB2 peripheral clock enable register 2 (RCC_AHB2ENR2)
Address offset: 0x090
Reset value: 0x0000 0000 (for STM32U535/545/575/585)
Reset value: 0x8000 0000 (for STM32U59x/5Ax)
Reset value: 0xC000 0000 (for STM32U5Fx/5Gx)
Access: no wait state; word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is not
supported.

31

30

SRAM5 SRAM6
EN
EN

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

HSPI1
EN

Res.

Res.

Res.

OCTOS
PI2EN

Res.

Res.

Res.

OCTOS
PI1EN

Res.

Res.

Res.

FSMCE
N

rw

rw

rw

rw

Bit 31 SRAM5EN: SRAM5 clock enable
This bit is set and reset by software.
0: SRAM5 clock disabled
1: SRAM5 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 30 SRAM6EN: SRAM6 clock enable
This bit is set and reset by software.
0: SRAM6 clock disabled
1: SRAM6 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 29:13 Reserved, must be kept at reset value.
Bit 12 HSPI1EN: HSPI1 clock enable
This bit is set and cleared by software.
0: HSPI1 clock disabled
1: HSPI1 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 11:9 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 8 OCTOSPI2EN: OCTOSPI2 clock enable
This bit is set and cleared by software.
0: OCTOSPI2 clock disabled
1: OCTOSPI2 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 OCTOSPI1EN: OCTOSPI1 clock enable
This bit is set and cleared by software.
0: OCTOSPI1 clock disabled
1: OCTOSPI1 clock enabled
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 FSMCEN: FSMC clock enable
This bit is set and cleared by software.
0: FSMC clock disabled
1: FSMC clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

11.8.32

RCC AHB3 peripheral clock enable register (RCC_AHB3ENR)
Address offset: 0x094
Reset value: 0x8000 0000
Access: no wait state; word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.

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

SRAM4
EN

Res.

Res.

Res.

Res.

Res.

Res.

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

GTZC2
EN

Res.

Res.

Res.

Res.

Res.

PWRE
N

Res.

LPGPI
O1EN

rw

rw

ADF1E LPDMA
N
1EN
rw

rw

DAC1E ADC4E
N
N
rw

rw

rw

rw

Bit 31 SRAM4EN: SRAM4 clock enable
This bit is set and reset by software.
0: SRAM4 clock disabled
1: SRAM4 clock enabled
Bits 30:13 Reserved, must be kept at reset value.
Bit 12 GTZC2EN: GTZC2 clock enable
This bit is set and cleared by software.
0: GTZC2 clock disabled
1: GTZC2 clock enabled

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 11 Reserved, must be kept at reset value.
Bit 10 ADF1EN: ADF1 clock enable
This bit is set and cleared by software.
0: ADF1 clock disabled
1: ADF1 clock enabled
Bit 9 LPDMA1EN: LPDMA1 clock enable
This bit is set and cleared by software.
0: LPDMA1 clock disabled
1: LPDMA1 clock enabled
Bits 8:7 Reserved, must be kept at reset value.
Bit 6 DAC1EN: DAC1 clock enable
This bit is set and cleared by software.
0: DAC1 clock disabled
1: DAC1 clock enabled
Bit 5 ADC4EN: ADC4 clock enable
This bit is set and cleared by software.
0: ADC4 clock disabled
1: ADC4 clock enabled
Bits 4:3 Reserved, must be kept at reset value.
Bit 2 PWREN: PWR clock enable
This bit is set and cleared by software.
0: PWR clock disabled
1: PWR clock enabled
Bit 1 Reserved, must be kept at reset value.
Bit 0 LPGPIO1EN: LPGPIO1 enable
This bit is set and cleared by software.
0: LPGPIO1 clock disabled
1: LPGPIO1 clock enabled

11.8.33

RCC APB1 peripheral clock enable register 1 (RCC_APB1ENR1)
Address offset: 0x09C
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.

31

30

29

28

27

26

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

Res.

SPI2E
N

Res.

WWDG
EN

rw

<!-- pagebreak -->

Res.

Res.

25

24

USART
CRSEN
6EN
rw

rw

9

8

Res.

Res.

23
Res.

7
Res.

rs

RM0456 Rev 6

22

21

20

19

18

17

UART5 UART4 USART USART
I2C2EN I2C1EN
EN
EN
3EN
2EN

16
Res.

rw

rw

rw

rw

rw

6

5

4

3

2

rw
1

0

Res.

TIM7E
N

TIM6E
N

TIM5E
N

TIM4E
N

TIM3E
N

TIM2E
N

rw

rw

rw

rw

rw

rw

RM0456

Reset and clock control (RCC)

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 USART6EN: USART6 clock enable
This bit is set and cleared by software.
0: USART6 clock disabled
1: USART6 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 24 CRSEN: CRS clock enable
This bit is set and cleared by software.
0: CRS clock disabled
1: CRS clock enabled
Bit 23 Reserved, must be kept at reset value.
Bit 22 I2C2EN: I2C2 clock enable
This bit is set and cleared by software.
0: I2C2 clock disabled
1: I2C2 clock enabled
Bit 21 I2C1EN: I2C1 clock enable
This bit is set and cleared by software.
0: I2C1 clock disabled
1: I2C1 clock enabled
Bit 20 UART5EN: UART5 clock enable
This bit is set and cleared by software.
0: UART5 clock disabled
1: UART5 clock enabled
Bit 19 UART4EN: UART4 clock enable
This bit is set and cleared by software.
0: UART4 clock disabled
1: UART4 clock enabled
Bit 18 USART3EN: USART3 clock enable
This bit is set and cleared by software.
0: USART3 clock disabled
1: USART3 clock enabled
Bit 17 USART2EN: USART2 clock enable
This bit is set and cleared by software.
0: USART2 clock disabled
1: USART2 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 16:15 Reserved, must be kept at reset value.
Bit 14 SPI2EN: SPI2 clock enable
This bit is set and cleared by software.
0: SPI2 clock disabled
1: SPI2 clock enabled
Bits 13:12 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 11 WWDGEN: WWDG clock enable
This bit is set by software to enable the window watchdog clock. It is reset by hardware
system reset. This bit can also be set by hardware if the WWDG_SW option bit is reset.
0: WWDG clock disabled
1: WWDG clock enabled
Bits 10:6 Reserved, must be kept at reset value.
Bit 5 TIM7EN: TIM7 clock enable
This bit is set and cleared by software.
0: TIM7 clock disabled
1: TIM7 clock enabled
Bit 4 TIM6EN: TIM6 clock enable
This bit is set and cleared by software.
0: TIM6 clock disabled
1: TIM6 clock enabled
Bit 3 TIM5EN: TIM5 clock enable
This bit is set and cleared by software.
0: TIM5 clock disabled
1: TIM5 clock enabled
Bit 2 TIM4EN: TIM4 clock enable
This bit is set and cleared by software.
0: TIM4 clock disabled
1: TIM4 clock enabled
Bit 1 TIM3EN: TIM3 clock enable
This bit is set and cleared by software.
0: TIM3 clock disabled
1: TIM3 clock enabled
Bit 0 TIM2EN: TIM2 clock enable
This bit is set and cleared by software.
0: TIM2 clock disabled
1: TIM2 clock enabled

11.8.34

RCC APB1 peripheral clock enable register 2 (RCC_APB1ENR2)
Address offset: 0x0A0
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.

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

UCPD1
EN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

4

3

2

1

0

LPTIM2
EN

Res.

Res.

Res.

I2C4EN

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

Res.

Res.

Res.

Res.

Res.

Res.

FDCAN
1EN

Res.

rw

rw

<!-- pagebreak -->

7

I2C6EN I2C5EN
rw

RM0456 Rev 6

rw

rw

rw

RM0456

Reset and clock control (RCC)

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 UCPD1EN: UCPD1 clock enable
This bit is set and cleared by software.
0: UCPD1 clock disabled
1: UCPD1 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 22:10 Reserved, must be kept at reset value.
Bit 9 FDCAN1EN: FDCAN1 clock enable
This bit is set and cleared by software.
0: FDCAN1 clock disabled
1: FDCAN1 clock enabled
Note: This bit is only available on some devices in the STM32U5 series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and kept at reset value.
Bit 8 Reserved, must be kept at reset value.
Bit 7 I2C6EN: I2C6 clock enable
This bit is set and cleared by software.
0: I2C6 clock disabled
1: I2C6 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 6 I2C5EN: I2C5 clock enable
This bit is set and cleared by software.
0: I2C5 clock disabled
1: I2C5 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 5 LPTIM2EN: LPTIM2 clock enable
This bit is set and cleared by software.
0: LPTIM2 clock disabled
1: LPTIM2 clock enabled
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 I2C4EN: I2C4 clock enable
This bit is set and cleared by software
0: I2C4 clock disabled
1: I2C4 clock enabled
Bit 0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.35

RM0456

RCC APB2 peripheral clock enable register (RCC_APB2ENR)
Address offset: 0x0A4
Reset value: 0x0000 0000
Access: word, half-word, and byte access

Note:

When the peripheral clock is not active, read or write access to peripheral registers is
not supported.

31

30

29

28

27

26

LTDCE
DSIEN
N

25

24

GFXTI
USBEN
MEN

23

22

21

Res.

SAI2E
N

SAI1E
N

16

rw

rw

rw

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

rw

rw

rw

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

SPI1E
N

TIM1E
N

Res.

Res.

Res.

Res.

Res.

rw

rw

rw

17

5

Res.

rw

18

TIM17E TIM16E TIM15E
N
N
N

Res.

Res.

USART TIM8E
1EN
N

19

Res.

Res.

Res.

20

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 DSIEN: DSI clock enable
This bit is set and cleared by software.
0: DSI clock disabled
1: DSI clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 26 LTDCEN: LTDC clock enable
This bit is set and cleared by software.
0: LTDC clock disabled
1: LTDC clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 25 GFXTIMEN: GFXTIM clock enable
This bit is set and cleared by software.
0: GFXTIM clock disabled
1: GFXTIM clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 24 USBEN: USB clock enable
This bit is set and cleared by software.
0: USB clock disabled
1: USB clock enabled
Note: This bit is only available on STM32U535/545 devices, it is reserved on other devices in
the STM32U5 Series. If not present, consider this bit as reserved and keep it at reset
value.
Bit 23 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 22 SAI2EN: SAI2 clock enable
This bit is set and cleared by software.
0: SAI2 clock disabled
1: SAI2 clock enabled
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral.
Bit 21 SAI1EN: SAI1 clock enable
This bit is set and cleared by software.
0: SAI1 clock disabled
1: SAI1 clock enabled
Bits 20:19 Reserved, must be kept at reset value.
Bit 18 TIM17EN: TIM17 clock enable
This bit is set and cleared by software.
0: TIM17 clock disabled
1: TIM17 clock enabled
Bit 17 TIM16EN: TIM16 clock enable
This bit is set and cleared by software.
0: TIM16 clock disabled
1: TIM16 clock enabled
Bit 16 TIM15EN: TIM15 clock enable
This bit is set and cleared by software.
0: TIM15 clock disabled
1: TIM15 clock enabled
Bit 15 Reserved, must be kept at reset value.
Bit 14 USART1EN: USART1clock enable
This bit is set and cleared by software.
0: USART1 clock disabled
1: USART1 clock enabled
Bit 13 TIM8EN: TIM8 clock enable
This bit is set and cleared by software.
0: TIM8 clock disabled
1: TIM8 clock enabled
Bit 12 SPI1EN: SPI1 clock enable
This bit is set and cleared by software.
0: SPI1 clock disabled
1: SPI1 clock enabled
Bit 11 TIM1EN: TIM1 clock enable
This bit is set and cleared by software.
0: TIM1 clock disabled
1: TIM1 clock enabled
Bits 10:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

11.8.36

RM0456

RCC APB3 peripheral clock enable register (RCC_APB3ENR)
Address offset: 0x0A8
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

Res.

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

COMP OPAMP LPTIM4 LPTIM3 LPTIM1
EN
EN
EN
EN
EN
rw

rw

rw

rw

Res.

Res.

Res.

LPUAR
I2C3EN
T1EN

rw

rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 RTCAPBEN: RTC and TAMP APB clock enable
This bit is set and cleared by software.
0: RTC and TAMP APB clock disabled
1: RTC and TAMP APB clock enabled
Bit 20 VREFEN: VREFBUF clock enable
This bit is set and cleared by software.
0: VREFBUF clock disabled
1: VREFBUF clock enabled
Bits 19:16 Reserved, must be kept at reset value.
Bit 15 COMPEN: COMP clock enable
This bit is set and cleared by software.
0: COMP clock disabled
1: COMP clock enabled
Bit 14 OPAMPEN: OPAMP clock enable
This bit is set and cleared by software.
0: OPAMP clock disabled
1: OPAMP clock enabled
Bit 13 LPTIM4EN: LPTIM4 clock enable
This bit is set and cleared by software.
0: LPTIM4 clock disabled
1: LPTIM4 clock enabled
Bit 12 LPTIM3EN: LPTIM3 clock enable
This bit is set and cleared by software.
0: LPTIM3 clock disabled
1: LPTIM3 clock enabled
Bit 11 LPTIM1EN: LPTIM1 clock enable
This bit is set and cleared by software.
0: LPTIM1 clock disabled
1: LPTIM1 clock enabled
Bits 10:8 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

21

20

RTCAP VREFE
BEN
N
rw

rw

5

4

SPI3E
N
rw

Res.

19

18

17

16

Res.

Res.

Res.

Res.

3

2

1

0

Res.

SYSCF
GEN

Res.

Res.

rw

RM0456

Reset and clock control (RCC)

Bit 7 I2C3EN: I2C3 clock enable
This bit is set and cleared by software.
0: I2C3 clock disabled
1: I2C3 clock enabled
Bit 6 LPUART1EN: LPUART1 clock enable
This bit is set and cleared by software.
0: LPUART1 clock disabled
1: LPUART1 clock enabled
Bit 5 SPI3EN: SPI3 clock enable
This bit is set and cleared by software.
0: SPI3 clock disabled
1: SPI3 clock enabled
Bits 4:2 Reserved, must be kept at reset value.
Bit 1 SYSCFGEN: SYSCFG clock enable
This bit is set and cleared by software.
0: SYSCFG clock disabled
1: SYSCFG clock enabled
Bit 0 Reserved, must be kept at reset value.

11.8.37

RCC AHB1 peripheral clock enable in Sleep and Stop modes register
(RCC_AHB1SMENR)
Address offset: 0x0B0
Reset value: 0xFFFF FFFF
Access: no wait state, word, half-word, and byte access
This register only configures the clock gating, not the clock source itself. When a bit is set
in Stop mode, the corresponding peripheral clock is enabled only when a peripheral (this
one or another) requests the AHB or APB clock (refer to Section 11.4.24).

31

30

29

28

DCAC ICACH BKPSR
SRAM1
HE1SM ESME AMSM
SMEN
EN
N
EN
rw

rw

rw

rw

15

14

13

12

JPEGS
MEN
rw

Res.

Res.

CRCS
MEN
rw

27

26

25

Res.

Res.

Res.

11

10

9

24
GTZC1
SMEN

23

22

Res.

Res.

7

6

rw

Res.

Res.

Res.

8
FLASH
SMEN

Res.

rw

Res.

21

20

19

18

17

16

DCAC
GFXM
RAMC
GPU2D
DMA2D
TSCSM
HE2SM
MUSM
FGSM
SMEN
SMEN
EN
EN
EN
EN
rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

Res.

Res.

CORDI GPDM
MDF1S FMACS
CSME A1SME
MEN
MEN
N
N
rw

rw

rw

rw

Bit 31 SRAM1SMEN: SRAM1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: SRAM1 clocks disabled by the clock gating during Sleep and Stop modes
1: SRAM1 clocks enabled by the clock gating during Sleep and Stop modes

RM0456 Rev 6

<!-- pagebreak -->

609

Reset and clock control (RCC)

RM0456

Bit 30 DCACHE1SMEN: DCACHE1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DCACHE1 clocks disabled by the clock gating during Sleep and Stop modes
1: DCACHE1 clocks enabled by the clock gating during Sleep and Stop modes
Bit 29 ICACHESMEN: ICACHE clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: ICACHE clocks disabled by the clock gating during Sleep and Stop modes
1: ICACHE clocks enabled by the clock gating during Sleep and Stop modes
Bit 28 BKPSRAMSMEN: BKPSRAM clock enable during Sleep and Stop modes
This bit is set and cleared by software
0: BKPSRAM clocks disabled by the clock gating during Sleep and Stop modes
1: BKPSRAM clocks enabled by the clock gating during Sleep and Stop modes
Bits 27:25 Reserved, must be kept at reset value.
Bit 24 GTZC1SMEN: GTZC1 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GTZC1 clocks disabled by the clock gating during Sleep and Stop modes
1: GTZC1 clocks enabled by the clock gating during Sleep and Stop modes
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 DCACHE2SMEN: DCACHE2 clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DCACHE2 clocks disabled by the clock gating during Sleep and Stop modes
1: DCACHE2 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 20 GPU2DSMEN: GPU2D clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GPU2D clocks disabled by the clock gating during Sleep and Stop modes
1: GPU2D clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 19 GFXMMUSMEN: GFXMMU clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GFXMMU clocks disabled by the clock gating during Sleep and Stop modes
1: GFXMMU clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bit 18 DMA2DSMEN: DMA2D clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: DMA2D clocks disabled by the clock gating during Sleep and Stop modes
1: DMA2D clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Reset and clock control (RCC)

Bit 17 RAMCFGSMEN: RAMCFG clock enable during Sleep and Stop modes
This bit is set and cleared by software.
0: RAMCFG clocks disabled by the clock gating during Sleep and Stop modes
1: RAMCFG clocks enabled by the clock gating during Sleep and Stop modes
Bit 16 TSCSMEN: TSC clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: TSC clocks disabled by the clock gating during Sleep and Stop modes
1: TSC clocks enabled by the clock gating during Sleep and Stop modes
Bit 15 JPEGSMEN: JPEG clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: JPEG clocks disabled by the clock gating during Sleep and Stop modes
1: JPEG clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit is only available on some devices in the STM32U5 Series. Refer to the device
datasheet for availability of its associated peripheral. If not present, consider this bit as
reserved and keep it at reset value.
Bits 14:13 Reserved, must be kept at reset value.
Bit 12 CRCSMEN: CRC clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: CRC clocks disabled by the clock gating during Sleep and Stop modes
1: CRC clocks enabled by the clock gating during Sleep and Stop modes
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 FLASHSMEN: FLASH clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: FLASH clocks disabled by the clock gating during Sleep and Stop modes
1: FLASH clocks enabled by the clock gating during Sleep and Stop modes
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 MDF1SMEN: MDF1 clocks enable during Sleep and Stop modes.
This bit is set and cleared by software.
0: MDF1 clocks disabled by the clock gating during Sleep and Stop modes
1: MDF1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.
Bit 2 FMACSMEN: FMAC clocks enable during Sleep and Stop modes.
This bit is set and cleared by software.
0: FMAC clocks disabled by the clock gating during Sleep and Stop modes
1: FMAC clocks enabled by the clock gating during Sleep and Stop modes
Bit 1 CORDICSMEN: CORDIC clocks enable during Sleep and Stop modes
This bit is set and cleared by software during Sleep mode.
0: CORDIC clocks disabled by the clock gating during Sleep and Stop modes
1: CORDIC clocks enabled by the clock gating during Sleep and Stop modes
Bit 0 GPDMA1SMEN: GPDMA1 clocks enable during Sleep and Stop modes
This bit is set and cleared by software.
0: GPDMA1 clocks disabled by the clock gating during Sleep and Stop modes
1: GPDMA1 clocks enabled by the clock gating during Sleep and Stop modes
Note: This bit must be set to allow the peripheral to wake up from Stop modes.

RM0456 Rev 6

<!-- pagebreak -->

