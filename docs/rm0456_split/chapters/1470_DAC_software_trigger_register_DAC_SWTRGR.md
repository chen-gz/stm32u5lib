1485

Digital-to-analog converter (DAC)

RM0456

Bits 7:6 WAVE1[1:0]: DAC channel1 noise/triangle wave generation enable
These bits are set and cleared by software.
00: wave generation disabled
01: Noise wave generation enabled
10: Triangle wave generation enabled
11: Reserved
Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
Bits 5:2 TSEL1[3:0]: DAC channel1 trigger selection
These bits select the external event used to trigger DAC channel1
0000: SWTRIG1
0001: dac_ch1_trg1
0010: dac_ch1_trg2
...
1111: dac_ch1_trg15
Refer to the trigger selection tables in Section 35.4.2: DAC pins and internal signals for
details on trigger configuration and mapping.
Note: Only used if bit TEN1 = 1 (DAC channel1 trigger enabled).
Bit 1 TEN1: DAC channel1 trigger enable
This bit is set and cleared by software to enable/disable DAC channel1 trigger.
0: DAC channel1 trigger disabled and data written into the DAC_DHR1 register are
transferred one dac_hclk clock cycle later to the DAC_DOR1 register
1: DAC channel1 trigger enabled and data from the DAC_DHR1 register are transferred
three dac_hclk clock cycles later to the DAC_DOR1 register
Note: When software trigger is selected, the transfer from the DAC_DHR1 register to the
DAC_DOR1 register takes only one dac_hclk clock cycle.
Bit 0 EN1: DAC channel1 enable
This bit is set and cleared by software to enable/disable DAC channel1.
0: DAC channel1 disabled
1: DAC channel1 enabled

35.7.2

DAC software trigger register (DAC_SWTRGR)
Address offset: 0x04
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

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SWTRIG2 SWTRIG1
w

<!-- pagebreak -->

RM0456 Rev 6

w

RM0456

Digital-to-analog converter (DAC)

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 SWTRIG2: DAC channel2 software trigger
This bit is set by software to trigger the DAC in software trigger mode.
0: No trigger
1: Trigger
Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR2
register value has been loaded into the DAC_DOR2 register.
This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 0 SWTRIG1: DAC channel1 software trigger
This bit is set by software to trigger the DAC in software trigger mode.
0: No trigger
1: Trigger
Note: This bit is cleared by hardware (one dac_hclk clock cycle later) once the DAC_DHR1
register value has been loaded into the DAC_DOR1 register.

35.7.3

DAC channel1 12-bit right-aligned data holding register
(DAC_DHR12R1)
Address offset: 0x08
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

23

22

rw

rw

rw

rw

rw

rw

11

10

9

8

7

6

21

20

19

18

17

16

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

rw

rw

rw

rw

rw

DACC1DHRB[11:0]

DACC1DHR[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 DACC1DHRB[11:0]: DAC channel1 12-bit right-aligned data B
These bits are written by software. They specify 12-bit data for DAC channel1 when the
DAC operates in double data mode.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DACC1DHR[11:0]: DAC channel1 12-bit right-aligned data
These bits are written by software. They specify 12-bit data for DAC channel1.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

35.7.4

RM0456

DAC channel1 12-bit left aligned data holding register
(DAC_DHR12L1)
Address offset: 0x0C
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

DACC1DHRB[11:0]
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

DACC1DHR[11:0]
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

Res.

Res.

Res.

rw

Bits 31:20 DACC1DHRB[11:0]: DAC channel1 12-bit left-aligned data B
These bits are written by software. They specify 12-bit data for DAC channel1 when the DAC
operates in double data mode.
Bits 19:16 Reserved, must be kept at reset value.
Bits 15:4 DACC1DHR[11:0]: DAC channel1 12-bit left-aligned data
These bits are written by software.
They specify 12-bit data for DAC channel1.
Bits 3:0 Reserved, must be kept at reset value.

35.7.5

DAC channel1 8-bit right aligned data holding register
(DAC_DHR8R1)
Address offset: 0x10
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

DACC1DHRB[7:0]
rw

rw

DACC1DHR[7:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 DACC1DHRB[7:0]: DAC channel1 8-bit right-aligned data
These bits are written by software. They specify 8-bit data for DAC channel1 when the DAC
operates in double data mode.
Bits 7:0 DACC1DHR[7:0]: DAC channel1 8-bit right-aligned data
These bits are written by software. They specify 8-bit data for DAC channel1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

35.7.6

DAC channel2 12-bit right aligned data holding register
(DAC_DHR12R2)
This register is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Address offset: 0x14
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

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

DACC2DHRB[11:0]
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

DACC2DHR[11:0]
rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 DACC2DHRB[11:0]: DAC channel2 12-bit right-aligned data
These bits are written by software. They specify 12-bit data for DAC channel2 when the DAC
operates in DMA double data mode.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DACC2DHR[11:0]: DAC channel2 12-bit right-aligned data
These bits are written by software. They specify 12-bit data for DAC channel2.

35.7.7

DAC channel2 12-bit left aligned data holding register
(DAC_DHR12L2)
This register is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Address offset: 0x18
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

DACC2DHRB[11:0]
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

DACC2DHR[11:0]
rw

rw

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

Res.

Res.

Res.

Bits 31:20 DACC2DHRB[11:0]: DAC channel2 12-bit left-aligned data B
These bits are written by software. They specify 12-bit data for DAC channel2 when the
DAC operates in double data mode.
Bits 19:16 Reserved, must be kept at reset value.
Bits 15:4 DACC2DHR[11:0]: DAC channel2 12-bit left-aligned data
These bits are written by software which specify 12-bit data for DAC channel2.
Bits 3:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

35.7.8

RM0456

DAC channel2 8-bit right-aligned data holding register
(DAC_DHR8R2)
This register is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Address offset: 0x1C
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

DACC2DHRB[7:0]
rw

rw

DACC2DHR[7:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 DACC2DHRB[7:0]: DAC channel2 8-bit right-aligned data
These bits are written by software. They specify 8-bit data for DAC channel2 when the DAC
operates in double data mode.
Bits 7:0 DACC2DHR[7:0]: DAC channel2 8-bit right-aligned data
These bits are written by software which specifies 8-bit data for DAC channel2.

35.7.9

Dual DAC 12-bit right-aligned data holding register
(DAC_DHR12RD)
Address offset: 0x20
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

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

DACC2DHR[11:0]
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

rw

rw

rw

rw

rw

DACC1DHR[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 DACC2DHR[11:0]: DAC channel2 12-bit right-aligned data
These bits are written by software which specifies 12-bit data for DAC channel2.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DACC1DHR[11:0]: DAC channel1 12-bit right-aligned data
These bits are written by software which specifies 12-bit data for DAC channel1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

35.7.10

Dual DAC 12-bit left aligned data holding register
(DAC_DHR12LD)
Address offset: 0x24
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

DACC2DHR[11:0]
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

DACC1DHR[11:0]
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

Res.

Res.

Res.

rw

Bits 31:20 DACC2DHR[11:0]: DAC channel2 12-bit left-aligned data
These bits are written by software which specifies 12-bit data for DAC channel2.
Bits 19:16 Reserved, must be kept at reset value.
Bits 15:4 DACC1DHR[11:0]: DAC channel1 12-bit left-aligned data
These bits are written by software which specifies 12-bit data for DAC channel1.
Bits 3:0 Reserved, must be kept at reset value.

35.7.11

Dual DAC 8-bit right aligned data holding register
(DAC_DHR8RD)
Address offset: 0x28
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

rw

rw

rw

DACC2DHR[7:0]
rw

rw

rw

rw

rw

DACC1DHR[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:8 DACC2DHR[7:0]: DAC channel2 8-bit right-aligned data
These bits are written by software which specifies 8-bit data for DAC channel2.
Bits 7:0 DACC1DHR[7:0]: DAC channel1 8-bit right-aligned data
These bits are written by software which specifies 8-bit data for DAC channel1.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

35.7.12

RM0456

DAC channel1 data output register (DAC_DOR1)
Address offset: 0x2C
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

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

DACC1DORB[11:0]
r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

r

r

r

17

16

DACC1DOR[11:0]
r

r

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 DACC1DORB[11:0]: DAC channel1 data output
These bits are read-only. They contain data output for DAC channel1 B.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DACC1DOR[11:0]: DAC channel1 data output
These bits are read-only, they contain data output for DAC channel1.

35.7.13

DAC channel2 data output register (DAC_DOR2)
This register is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Address offset: 0x30
Reset value: 0x0000 0000

31

30

29

28

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

Res.

Res.

27

26

25

24

23

22

21

20

19

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

DACC2DORB[11:0]

DACC2DOR[11:0]
r

r

r

r

r

r

r

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 DACC2DORB[11:0]: DAC channel2 data output
These bits are read-only. They contain data output for DAC channel2 B.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 DACC2DOR[11:0]: DAC channel2 data output
These bits are read-only, they contain data output for DAC channel2.

<!-- pagebreak -->

18

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

35.7.14

DAC status register (DAC_SR)
Address offset: 0x34
Reset value: 0x0000 0000

31

30

CAL_
BWST2
FLAG2

29

28

27

DMAU DORST DAC2R
DR2
AT2
DY

r

r

rc_w1

r

r

15

14

13

12

11

BWST1

CAL_
FLAG1

r

r

DMAU DORST DAC1R
DR1
AT1
DY
rc_w1

r

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

r

Bit 31 BWST2: DAC channel2 busy writing sample time flag
This bit is systematically set just after sample and hold mode enable. It is set each time the
software writes the register DAC_SHSR2, It is cleared by hardware when the write operation
of DAC_SHSR2 is complete. (It takes about 3 LSI/LSE periods of synchronization).
0:There is no write operation of DAC_SHSR2 ongoing: DAC_SHSR2 can be written
1:There is a write operation of DAC_SHSR2 ongoing: DAC_SHSR2 cannot be written
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 30 CAL_FLAG2: DAC channel2 calibration offset status
This bit is set and cleared by hardware
0: calibration trimming value is lower than the offset correction value
1: calibration trimming value is equal or greater than the offset correction value
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 29 DMAUDR2: DAC channel2 DMA underrun flag
This bit is set by hardware and cleared by software (by writing it to 1).
0: No DMA underrun error condition occurred for DAC channel2
1: DMA underrun error condition occurred for DAC channel2 (the currently selected trigger is
driving DAC channel2 conversion at a frequency higher than the DMA service capability
rate).
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 28 DORSTAT2: DAC channel2 output register status bit
This bit is set and cleared by hardware. It is applicable only when the DAC operates in
double data mode.
0: DOR[11:0] is used actual DAC output
1: DORB[11:0] is used actual DAC output
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 27 DAC2RDY: DAC channel2 ready status bit
This bit is set and cleared by hardware.
0: DAC channel2 is not yet ready to accept the trigger nor output data
1: DAC channel2 is ready to accept the trigger or output data
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Bits 26:16 Reserved, must be kept at reset value.
Bit 15 BWST1: DAC channel1 busy writing sample time flag
This bit is systematically set just after sample and hold mode enable and is set each time the
software writes the register DAC_SHSR1, It is cleared by hardware when the write operation of
DAC_SHSR1 is complete. (It takes about 3 LSI/LSE periods of synchronization).
0:There is no write operation of DAC_SHSR1 ongoing: DAC_SHSR1 can be written
1:There is a write operation of DAC_SHSR1 ongoing: DAC_SHSR1 cannot be written
Bit 14 CAL_FLAG1: DAC channel1 calibration offset status
This bit is set and cleared by hardware
0: calibration trimming value is lower than the offset correction value
1: calibration trimming value is equal or greater than the offset correction value
Bit 13 DMAUDR1: DAC channel1 DMA underrun flag
This bit is set by hardware and cleared by software (by writing it to 1).
0: No DMA underrun error condition occurred for DAC channel1
1: DMA underrun error condition occurred for DAC channel1 (the currently selected trigger is
driving DAC channel1 conversion at a frequency higher than the DMA service capability rate)
Bit 12 DORSTAT1: DAC channel1 output register status bit
This bit is set and cleared by hardware. It is applicable only when the DAC operates in
double data mode.
0: DOR[11:0] is used actual DAC output
1: DORB[11:0] is used actual DAC output
Bit 11 DAC1RDY: DAC channel1 ready status bit
This bit is set and cleared by hardware.
0: DAC channel1 is not yet ready to accept the trigger nor output data
1: DAC channel1 is ready to accept the trigger or output data
Bits 10:0 Reserved, must be kept at reset value.

35.7.15

DAC calibration control register (DAC_CCR)
Address offset: 0x38
Reset value: 0x00XX 00XX

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

20

19

rw

rw

4

3

18

17

16

rw

rw

rw

2

1

0

rw

rw

OTRIM2[4:0]

OTRIM1[4:0]
rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 OTRIM2[4:0]: DAC channel2 offset trimming value
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bits 15:5 Reserved, must be kept at reset value.
Bits 4:0 OTRIM1[4:0]: DAC channel1 offset trimming value

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

35.7.16

DAC mode control register (DAC_MCR)
Address offset: 0x3C
Reset value: 0x0000 0000

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

HFSEL[1:0]

Res.

Res.

Res.

Res.

rw

rw

25

24

DMA
SINFO
DOUBLE
RMAT2
2
rw

rw

9

8

DMA
SINFO
DOUBLE
RMAT1
1
rw

23

22

21

20

19

Res.

Res.

Res.

Res.

Res.

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

rw

18

17

16

MODE2[2:0]
rw

rw

rw

2

1

0

MODE1[2:0]
rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 SINFORMAT2: Enable signed format for DAC channel2
This bit is set and cleared by software.
0: Input data is in unsigned format
1: Input data is in signed format (2’s complement). The MSB bit represents the sign.
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 24 DMADOUBLE2: DAC channel2 DMA double data mode
This bit is set and cleared by software.
0: DMA normal mode selected
1: DMA double data mode selected
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bits 23:19 Reserved, must be kept at reset value.
Bits 18:16 MODE2[2:0]: DAC channel2 mode
These bits can be written only when the DAC is disabled and not in the calibration mode
(when bit EN2 = 0 and bit CEN2 = 0 in the DAC_CR register). If EN2 = 1 or CEN2 = 1 the
write operation is ignored.
They can be set and cleared by software to select the DAC channel2 mode:
– DAC channel2 in normal mode
000: DAC channel2 is connected to external pin with Buffer enabled
001: DAC channel2 is connected to external pin and to on chip peripherals with buffer
enabled
010: DAC channel2 is connected to external pin with buffer disabled
011: DAC channel2 is connected to on chip peripherals with Buffer disabled
– DAC channel2 in sample and hold mode
100: DAC channel2 is connected to external pin with Buffer enabled
101: DAC channel2 is connected to external pin and to on chip peripherals with Buffer
enabled
110: DAC channel2 is connected to external pin and to on chip peripherals with Buffer
disabled
111: DAC channel2 is connected to on chip peripherals with Buffer disabled
Note: This register can be modified only when EN2 = 0.
Refer to Section 35.3: DAC implementation for the availability of DAC channel2.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Bits 15:14 HFSEL[1:0]: High frequency interface mode selection
00: High frequency interface mode disabled
01: High frequency interface mode enabled for AHB clock frequency > 80 MHz
10: High frequency interface mode enabled for AHB clock frequency >160 MHz
11: Reserved
Bits 13:10 Reserved, must be kept at reset value.
Bit 9 SINFORMAT1: Enable signed format for DAC channel1
This bit is set and cleared by software.
0: Input data is in unsigned format
1: Input data is in signed format (2’s complement). The MSB bit represents the sign.
Bit 8 DMADOUBLE1: DAC channel1 DMA double data mode
This bit is set and cleared by software.
0: DMA normal mode selected
1: DMA double data mode selected
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 MODE1[2:0]: DAC channel1 mode
These bits can be written only when the DAC is disabled and not in the calibration mode
(when bit EN1 = 0 and bit CEN1 = 0 in the DAC_CR register). If EN1 = 1 or CEN1 = 1 the
write operation is ignored.
They can be set and cleared by software to select the DAC channel1 mode:
– DAC channel1 in normal mode
000: DAC channel1 is connected to external pin with Buffer enabled
001: DAC channel1 is connected to external pin and to on chip peripherals with Buffer
enabled
010: DAC channel1 is connected to external pin with Buffer disabled
011: DAC channel1 is connected to on chip peripherals with Buffer disabled
– DAC channel1 in sample & hold mode
100: DAC channel1 is connected to external pin with Buffer enabled
101: DAC channel1 is connected to external pin and to on chip peripherals with Buffer
enabled
110: DAC channel1 is connected to external pin and to on chip peripherals with Buffer
disabled
111: DAC channel1 is connected to on chip peripherals with Buffer disabled
Note: This register can be modified only when EN1 = 0.

35.7.17

DAC channel1 sample and hold sample time register
(DAC_SHSR1)
Address offset: 0x40
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

Res.

Res.

Res.

Res.

Res.

Res.

TSAMPLE1[9:0]
rw

<!-- pagebreak -->

rw

rw

RM0456 Rev 6

rw

rw

rw

RM0456

Digital-to-analog converter (DAC)

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 TSAMPLE1[9:0]: DAC channel1 sample time (only valid in sample and hold mode)
These bits can be written when the DAC channel1 is disabled or also during normal operation.
in the latter case, the write can be done only when BWST1 of DAC_SR register is low, If
BWST1 = 1, the write operation is ignored.

Note:

It represents the number of LSI/LSE clocks to perform a sample phase. Sampling time =
(TSAMPLE1[9:0] + 1) x LSI/LSE clock period.

35.7.18

DAC channel2 sample and hold sample time register
(DAC_SHSR2)
This register is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Address offset: 0x44
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

Res.

Res.

Res.

Res.

Res.

Res.

TSAMPLE2[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:10 Reserved, must be kept at reset value.
Bits 9:0 TSAMPLE2[9:0]: DAC channel2 sample time (only valid in sample and hold mode)
These bits can be written when the DAC channel2 is disabled or also during normal
operation. in the latter case, the write can be done only when BWST2 of DAC_SR register is
low, if BWST2 = 1, the write operation is ignored.

Note:

It represents the number of LSI/LSE clocks to perform a sample phase. Sampling time =
(TSAMPLE1[9:0] + 1) x LSI/LSE clock period.

35.7.19

DAC sample and hold time register (DAC_SHHR)
Address offset: 0x48
Reset value: 0x0001 0001

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

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

rw

rw

rw

rw

rw

9

8

7

6

5

20

19

18

17

16

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

rw

rw

rw

rw

THOLD2[9:0]

THOLD1[9:0]
rw

rw

rw

RM0456 Rev 6

rw

rw

rw

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:16 THOLD2[9:0]: DAC channel2 hold time (only valid in sample and hold mode).
Hold time = (THOLD[9:0]) x LSI/LSE clock period
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Note: This register can be modified only when EN2 = 0.
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:0 THOLD1[9:0]: DAC channel1 hold time (only valid in sample and hold mode)
Hold time = (THOLD[9:0]) x LSI/LSE clock period
Note: This register can be modified only when EN1 = 0.

Note:

These bits can be written only when the DAC channel is disabled and in normal operating
mode (when bit ENx = 0 and bit CENx = 0 in the DAC_CR register). If ENx = 1 or CENx = 1
the write operation is ignored.

35.7.20

DAC sample and hold refresh time register (DAC_SHRR)
Address offset: 0x4C
Reset value: 0x0001 0001

31

30

29

28

27

26

25

24

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

Res.

Res.

23

22

21

20

19

18

17

16

TREFRESH2[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

7

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

rw

rw

TREFRESH1[7:0]
rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 TREFRESH2[7:0]: DAC channel2 refresh time (only valid in sample and hold mode)
Refresh time = (TREFRESH[7:0]) x LSI/LSE clock period
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Note: This register can be modified only when EN2 = 0.
Bits 15:8 Reserved, must be kept at reset value.
Bits 7:0 TREFRESH1[7:0]: DAC channel1 refresh time (only valid in sample and hold mode)
Refresh time = (TREFRESH[7:0]) x LSI/LSE clock period
Note: This register can be modified only when EN1 = 0.

Note:

<!-- pagebreak -->

These bits can be written only when the DAC channel is disabled and in normal operating
mode (when bit ENx = 0 and bit CENx = 0 in the DAC_CR register). If ENx = 1 or CENx = 1
the write operation is ignored.

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

35.7.21

DAC autonomous mode control register (DAC_AUTOCR)
Address offset: 0x54
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

18

17

16

AUTO
MODE

Res.

Res.

Res.

Res.

Res.

Res.

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

Res.

rw

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 AUTOMODE: DAC autonomous mode
This bit is set and cleared by software.
0: DAC autonomous mode disabled
1: DAC autonomous mode enabled
Bits 21:0 Reserved, must be kept at reset value.

35.7.22

DAC register map
Table 350 summarizes the DAC registers.

Res.

Res.

0

0

0

0

0

0

0

0

0

0

DAC_
DHR8R1

Res.

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

DACC1DHRB[7:0]
0

RM0456 Rev 6

0

DACC1DHR[11:0]
0

Reset
value

Res.

0

0

0

0

0

0

0

0

0

0

0
Res.

0

DACC1DHR[11:0]

Res.

0

0

0

Res.

0

0

0

Res.

0

0

Res.

Reset
value

Res.

DACC1DHRB[11:0]

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

0

Res.

Res.

Res.

Res.

Res.

DAC_
DHR12L1

0

Res.

0x10

0

Res.

0x0C

Reset
value

DACC1DHRB[11:0]

Res.

0x08

DAC_
DHR12R1

Res.

Reset
value

SWTRIG1

0

SWTRIG2

0

Res.

WAVE1[1:0]

1

Res.

EN1

Res.

2

Res.

MAMP1[3:0]

TEN1

Res.

TSEL1[0]

0

TSEL1[3:1]

Res.

0

3

0

4

0

5

0

6

0

7

0

8

0

9

0

11

Res.

10

Res.

0

Res.

Res.

12

Res.

0

Res.

Res.

13

Res.

WAVE2[2:0]

DMAEN1

Res.

0

Res.

Res.

14

0

DMAUDRIE1

0

Res.

0

15

0

CEN1

16

0

Res.

17

EN2

0

Res.

18

TEN2

0

Res.

19

TSEL2[0]

0

Res.

20

0

Res.

21

0

Res.

22

0

Res.

23

0

Res.

24

0

Res.

DAC_
SWTRGR

25

28

0

Res.

0x04

26

29

DMAEN2

0

MAMP2[3:0]

30

DMAUDRIE2

TSEL2[3:1]

Reset
value

27

31

DAC_CR
0x00

Res.

Register
name
reset value

CEN2

Offset

Res.

Table 350. DAC register map and reset values

0

0

DACC1DHR[7:0]
0

0

0

0

0

0

0

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

2

1

0

Res.

4

3

Res.

Res.

0

0

0

0

Res.

Res.

0

0

0

0

0

0

0

0

0

0

Res.

0

Res.

0

0

0

0

0

0

0

0

0

0

0

0

0

DACC1DHR[7:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

Res.

0

OTRIM1[0]

0

0

Res.

0

Res.

0

OTRIM1[1]

0

OTRIM1[2]

0

Res.

0

OTRIM1[3]

0

X

X

X

X

X

Res.

HFSEL[0]

Res.

5

6

7

8

9

12

11

13

Res.
Res.

Res.
HFSEL[1]

0

Res.

Res.
Res.

0

Res.

Res.

0

0

Res.

Res.

OTRIM2[0]

0

OTRIM2[1]

0

OTRIM2[2]

0

0

0

DACC2DOR[11:0]

0

0

0

DACC1DOR[11:0]

0

MODE2
[2:0]

0

OTRIM1[4]

Res.

Res.

0

0

OTRIM2[3]

Res.

0

0

OTRIM2[4]

Res.

10

14

0

0

0

MODE1
[2:0]

Res.

Res.

Res.

16

Res.

15

0

Res.

Res.

0

Res.

Res.

Res.
0

X X

Res.

0

0

X

RM0456 Rev 6

0

DACC2DHR[7:0]

X

0

0

Res.

Res.
Res.
Res.
Res.

Res.

0

X

0

0

Res.

Res.
Res.
Res.

Res.

0

Res.

DMADOUBLE2

0

0

Res.

Res.

Res.

Res.

Res.
SINFORMAT2

Res.

Res.

Res.

Res.

Res.

Res.

Reset
value

0

Res.

Res.

Res.

Res.

0

Res.

0

Res.

0

DAC2RDY

0

Res.

0

DORSTAT2

0

DAC_CCR

0

0

0

0

<!-- pagebreak -->

0

0

0

Reset
value

0

0

0

0x3C

0

0

0

DAC_MCR

0

0

Res.

Res.

0

0

Res.

Res.

0

0

Res.

Res.

0

0

0

0x38

0

0

Reset
value

DAC_SR

0

DACC2DORB[11:0]

Res.

Reset
value

0x34

0

Res.

Res.

DAC_
DOR2

0

0

Res.

Res.
Res.

Reset
value

DACC1DORB[11:0]

0

Res.

Res.
Res.

DAC_
DOR1

0

Res.

Res.

0

Reset
value

0

DACC2DHR[7:0]

SINFORMAT1

DAC_
DHR8RD

0

DMADOUBLE1

0

0

Res.

0

0

Res.

0

0

0

Res.

0

DAC1RDY

0

Res.

0

0

DACC1DHR[11:0]

Res.

0

0

Res.

0

Res.

0

Res.

0

0

Res.

Res.

0

0

Res.

0

0

0

Res.

Res.

Reset
value

Res.

DACC2DHR[11:0]

0

0

DACC1DHR[11:0]

DORSTAT1

Res.

0

0

Res.

Res.

0

0

Res.

Res.

0

0
Res.

Res.

0

0

Res.

Res.

0

0

Res.

Res.

0

0

DMAUDR1

Res.
Res.

DAC_
DHR12LD

0

0

Res.

Res.
Res.

0

0

DACC2DHRB[7:0]

Res.

Res.

Reset
value

DACC2DHR[11:0]

0

Res.

Res.

Res.

DAC_
DHR12RD

Res.

Reset
value

0

Res.

DAC_
DHR8R2

0

Res.

0

BWST1
CAL_FLAG1

0

Res.

0

Res.

0

Res.

0

0

DACC2DHR[11:0]

Res.

0

0

DACC2DHR[11:0]
0

Res.

0

0

Res.

0

0

Res.

0

0
Res.

0

0

0
Res.

0

Res.

0

Res.

17

18

19

20

21

22

23

28

25

29

Res.

26

30

Res.

27

31

Res.

24

0

0

DMAUDR2

0x30

0

0

Res.

0x2C

0

Reset
value

Res.

0x28

0

DACC2DHRB[11:0]

Res.

0x24

DAC_
DHR12L2

Res.

0x20

0

Res.

0x1C

Reset
value

DACC2DHRB[11:0]

BWST2
CAL_FLAG2

0x18

DAC_
DHR12R2

Res.

0x14

Register
name
reset value

Res.

Offset

Res.

Table 350. DAC register map and reset values (continued)

0

0

0

RM0456

Digital-to-analog converter (DAC)

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset
value

Res.

0

0

0

0

0

0

0

0

0

0

0

0

0

1

TREFRESH1[7:0]
0

0

0

0

0

0

0

1

Res.

0

Res.

0

Res.

Res.

Res.

Res.

0

THOLD1[9:0]

Res.

Res.

Res.

Res.
Res.

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

AUTOMODE

Res.

Res.

Res.

Res.

Res.

DAC_
AUTOCR

0

1

Reserved

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

Res.

TREFRESH2[7:0]
0

Res.

Res.
1
Res.

0

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
0

0

22

Res.

0

1

23

Res.

0

2

24

Res.

0

3

25

Res.
Res.
Res.

0

4

26

Res.
Res.
Res.

Reset
value

0

5

27

Res.
Res.
Res.
Res.

0

6

28

Res.
Res.
Res.
Res.

0

7

29

Res.
Res.
Res.
Res.

THOLD2[9:0]

8

30

Res.
Res.
Res.
Res.

DAC_
SHRR

9

31

Res.

Res.

Res.
Res.

Res.

Reset
value

Res.

0x54

DAC_
SHHR

0

TSAMPLE2[9:0]
0

Res.

0x50

Reset
value

Res.

0x4C

DAC_
SHSR2

0

TSAMPLE1[9:0]
0

Res.

0x48

Reset
value

Res.

0x44

DAC_
SHSR1

Res.

0x40

Register
name
reset value

Res.

Offset

Res.

Table 350. DAC register map and reset values (continued)

0

0x580x60

Reserved

Res.

0x640x68

Reserved

Res.

Refer to Section 2.3 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

