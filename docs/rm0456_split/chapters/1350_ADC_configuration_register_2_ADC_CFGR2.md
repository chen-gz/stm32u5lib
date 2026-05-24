1377

Analog-to-digital converter (ADC12)

RM0456

Bits 9:5 EXTSEL[4:0]: External trigger selection for regular group
These bits select the external event used to trigger the start of conversion of a regular group:
00000: adc_ext_trg0
00001: adc_ext_trg1
...
Refer to the ADC external trigger for regular channels in Section 33.4.2: ADC pins and internal
signals for details on trigger mapping.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no
regular conversion is ongoing).
Bit 4 Reserved, must be kept at reset value.
Bits 3:2 RES[1:0]: Data resolution
These bits are written by software to select the resolution of the conversion.
00: 14 bits
01: 12 bits
10: 10 bits
11: 8bits
Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bits 1:0 DMNGT[1:0]: Data management configuration
This bit is set and cleared by software to select how the ADC interface output data are managed.
00: Regular conversion data stored in DR only
01: DMA One -shot mode selected
10: MDF mode selected
11: DMA circular mode selected
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

33.6.5

ADC configuration register 2 (ADC_CFGR2)
Address offset: 0x10
Reset value: 0x0000 0000

31

30

29

28

LSHIFT[3:0]

27

26

LFTRI
G

Res.

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

BULB

Res.

Res.

SMPTR SWTRI
IG
G
rw

rw

<!-- pagebreak -->

rw

10

25

24

22

21

20

19

18

17

16

OSR[9:0]
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

ROVSM TROVS
rw

23

rw

OVSS[3:0]
rw

rw

RM0456 Rev 6

rw

rw

JOVSE ROVSE
rw

rw

RM0456

Analog-to-digital converter (ADC12)

Bits 31:28 LSHIFT[3:0]: Left shift factor
This bitfield is set and cleared by software to define the left shifting applied to the final result with or
without oversampling.
0000: No left shift
0001: 1-bit left shift
0010: 2-bit left shift
0011: 3-bit left shift
0100: 4-bit left shift
0101: 5-bit left shift
0110: 6-bit left shift
0111: 7-bit left shift
1000: 8-bit left shift
1001: 9-bit left shift
1010: 10-bit left shift
1011: 11-bit left shift
1100: 12-bit left shift
1101: 13-bit left shift
1110: 14-bit left shift
1111: 15-bit left shift
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bit 27 LFTRIG: Low-frequency trigger
This bit is set and cleared by software
0: Low-frequency trigger mode disabled
1: Low-frequency trigger mode enabled
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bit 26 Reserved, must be kept at reset value.
Bits 25:16 OSR[9:0]: Oversampling ratio
This bitfield is set and cleared by software to define the oversampling ratio.
0: 1x (no oversampling)
1: 2x
2: 3x
...
1023: 1024x
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bit 15 SMPTRIG: Sampling time control trigger mode
This bit is set and cleared by software to enable the sampling time control trigger mode.
0: Sampling time control trigger mode disabled
1: Sampling time control trigger mode enabled
The sampling time starts on the trigger rising edge, and the conversion on the trigger falling edge.
EXTEN[1:0] bits must be set to 01. BULB bit must not be set when the SMPTRIG bit is set.
When EXTEN[1:0] bits is set to 00, set SWTRIG to start the sampling and clear SWTRIG bit to start
the conversion.
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bit 14 SWTRIG: Software trigger bit for sampling time control trigger mode
This bit is set and cleared by software to enable the bulb sampling mode.
0: Software trigger starts the conversion for sampling time control trigger mode
1: Software trigger starts the sampling for sampling time control trigger mode.
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bit 13 BULB: Bulb sampling mode
This bit is set and cleared by software to select the bulb sampling mode.
0: Bulb sampling mode disabled
1: Bulb sampling mode enabled. The sampling period starts just after the previous end of the
conversion.
SMPTRIG bit must not be set when the BULB bit is set.
The very first ADC conversion is performed with the sampling time specified in SMPx bits.
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bits 12:11 Reserved, must be kept at reset value.
Bit 10 ROVSM: Regular oversampling mode
This bit is set and cleared by software to select the regular oversampling mode.
0: Continued mode: When injected conversions are triggered, the oversampling is temporary
stopped and continued after the injection sequence (oversampling buffer is maintained during
injected sequence)
1: Resumed mode: When injected conversions are triggered, the current oversampling is aborted
and resumed from start after the injection sequence (oversampling buffer is zeroed by injected
sequence start)
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bit 9 TROVS: Triggered regular oversampling
This bit is set and cleared by software to enable triggered oversampling
0: All oversampled conversions for a channel are done consecutively following a trigger
1: Each oversampled conversion for a channel needs a new trigger
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no
conversion is ongoing).
Bits 8:5 OVSS[3:0]: Oversampling right shift
This bitfield is set and cleared by software to define the right shifting applied to the raw oversampling
result.
0000: No right shift
0001: 1-bit right shift
0010: 2-bit right shift
0011: 3-bit right shift
0100: 4-bit right shift
0101: 5-bit right shift
0110: 6-bit right shift
0111: 7-bit right shift
1000: 8-bit right shift
1001: 9-bit right shift
1010: 10-bit right shift
1011: 11-bit right shift
Others: Reserved, must not be used.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no
conversion is ongoing).

<!-- pagebreak -->

