RM0456 Rev 6

RM0456

35.6

Digital-to-analog converter (DAC)

DAC interrupts
Table 349. DAC interrupts

Interrupt
acronym

Interrupt
event

Event flag

Enable
Interrupt clear
control bit
method

DAC

DMA
underrun

DMAUDRx

DMAUDRI
Ex

35.7

Write
DMAUDRx = 1

Exit Sleep
mode

Exit Stop
mode

Exit Standby
mode

Yes

No

No

DAC registers
Refer to Section 1 on page 127 for a list of abbreviations used in register descriptions.
The peripheral registers have to be accessed by words (32-bit).

35.7.1

DAC control register (DAC_CR)
Address offset: 0x00
Reset value: 0x0000 0000

31

30

29

28

Res.

CEN2

DMAU
DRIE2

DMAE
N2

27

26

25

24

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

3

Res.

CEN1

DMAU
DRIE1

DMAE
N1

rw

rw

rw

MAMP2[3:0]

rw

rw

22

WAVE2[1:0]

MAMP1[3:0]
rw

23

WAVE1[1:0]
rw

rw

rw

21

20

19

18

17

16

TEN2

EN2

rw

rw

rw

2

1

0

TEN1

EN1

rw

rw

TSEL2[3] TSEL2[2] TSEL2[1] TSEL2[0]

TSEL1[3] TSEL1[2] TSEL1[1] TSEL1[0]
rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 CEN2: DAC channel2 calibration enable
This bit is set and cleared by software to enable/disable DAC channel2 calibration, it can be
written only if EN2 bit is set to 0 into DAC_CR (the calibration mode can be entered/exit only
when the DAC channel is disabled) Otherwise, the write operation is ignored.
0: DAC channel2 in normal operating mode
1: DAC channel2 in calibration mode
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 29 DMAUDRIE2: DAC channel2 DMA underrun interrupt enable
This bit is set and cleared by software.
0: DAC channel2 DMA underrun interrupt disabled
1: DAC channel2 DMA underrun interrupt enabled
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Bit 28 DMAEN2: DAC channel2 DMA enable
This bit is set and cleared by software.
0: DAC channel2 DMA mode disabled
1: DAC channel2 DMA mode enabled
Note: This bit is available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bits 27:24 MAMP2[3:0]: DAC channel2 mask/amplitude selector
These bits are written by software to select mask in wave generation mode or amplitude in
triangle generation mode.
0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1
0001: Unmask bits[1:0] of LFSR/ triangle amplitude equal to 3
0010: Unmask bits[2:0] of LFSR/ triangle amplitude equal to 7
0011: Unmask bits[3:0] of LFSR/ triangle amplitude equal to 15
0100: Unmask bits[4:0] of LFSR/ triangle amplitude equal to 31
0101: Unmask bits[5:0] of LFSR/ triangle amplitude equal to 63
0110: Unmask bits[6:0] of LFSR/ triangle amplitude equal to 127
0111: Unmask bits[7:0] of LFSR/ triangle amplitude equal to 255
1000: Unmask bits[8:0] of LFSR/ triangle amplitude equal to 511
1001: Unmask bits[9:0] of LFSR/ triangle amplitude equal to 1023
1010: Unmask bits[10:0] of LFSR/ triangle amplitude equal to 2047
≥ 1011: Unmask bits[11:0] of LFSR/ triangle amplitude equal to 4095
Note: These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bits 23:22 WAVE2[1:0]: DAC channel2 noise/triangle wave generation enable
These bits are set/reset by software.
00: wave generation disabled
01: Noise wave generation enabled
10: Triangle wave generation enabled
11: Reserved
Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled)
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bits 21:18 TSEL2[3:0]: DAC channel2 trigger selection
These bits select the external event used to trigger DAC channel2
0000: SWTRIG2
0001: dac_ch2_trg1
0010: dac_ch2_trg2
...
1111: dac_ch2_trg15
Refer to the trigger selection tables in Section 35.4.2: DAC pins and internal signals for
details on trigger configuration and mapping.
Note: Only used if bit TEN2 = 1 (DAC channel2 trigger enabled).
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)

Bit 17 TEN2: DAC channel2 trigger enable
This bit is set and cleared by software to enable/disable DAC channel2 trigger
0: DAC channel2 trigger disabled and data written into the DAC_DHR2 register are
transferred one dac_hclk clock cycle later to the DAC_DOR2 register
1: DAC channel2 trigger enabled and data from the DAC_DHR2 register are transferred
three dac_hclk clock cycles later to the DAC_DOR2 register
Note: When software trigger is selected, the transfer from the DAC_DHR2 register to the
DAC_DOR2 register takes only one dac_hclk clock cycle.
These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 16 EN2: DAC channel2 enable
This bit is set and cleared by software to enable/disable DAC channel2.
0: DAC channel2 disabled
1: DAC channel2 enabled
Note: These bits are available only on dual-channel DACs. Refer to Section 35.3: DAC
implementation.
Bit 15 Reserved, must be kept at reset value.
Bit 14 CEN1: DAC channel1 calibration enable
This bit is set and cleared by software to enable/disable DAC channel1 calibration, it can be
written only if bit EN1 = 0 into DAC_CR (the calibration mode can be entered/exit only when
the DAC channel is disabled) Otherwise, the write operation is ignored.
0: DAC channel1 in normal operating mode
1: DAC channel1 in calibration mode
Bit 13 DMAUDRIE1: DAC channel1 DMA Underrun Interrupt enable
This bit is set and cleared by software.
0: DAC channel1 DMA Underrun Interrupt disabled
1: DAC channel1 DMA Underrun Interrupt enabled
Bit 12 DMAEN1: DAC channel1 DMA enable
This bit is set and cleared by software.
0: DAC channel1 DMA mode disabled
1: DAC channel1 DMA mode enabled
Bits 11:8 MAMP1[3:0]: DAC channel1 mask/amplitude selector
These bits are written by software to select mask in wave generation mode or amplitude in
triangle generation mode.
0000: Unmask bit0 of LFSR/ triangle amplitude equal to 1
0001: Unmask bits[1:0] of LFSR/ triangle amplitude equal to 3
0010: Unmask bits[2:0] of LFSR/ triangle amplitude equal to 7
0011: Unmask bits[3:0] of LFSR/ triangle amplitude equal to 15
0100: Unmask bits[4:0] of LFSR/ triangle amplitude equal to 31
0101: Unmask bits[5:0] of LFSR/ triangle amplitude equal to 63
0110: Unmask bits[6:0] of LFSR/ triangle amplitude equal to 127
0111: Unmask bits[7:0] of LFSR/ triangle amplitude equal to 255
1000: Unmask bits[8:0] of LFSR/ triangle amplitude equal to 511
1001: Unmask bits[9:0] of LFSR/ triangle amplitude equal to 1023
1010: Unmask bits[10:0] of LFSR/ triangle amplitude equal to 2047
≥ 1011: Unmask bits[11:0] of LFSR/ triangle amplitude equal to 4095

RM0456 Rev 6

<!-- pagebreak -->

