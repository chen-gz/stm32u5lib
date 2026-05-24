1377

Analog-to-digital converter (ADC12)

33.7.2

RM0456

ADC system control register (ADC12_CCR)
Address offset: 0x08
Reset value: 0x0000 0000
The address offset is relative to the master ADC base address + 0x300.
ADC12_CCR is common to ADC1 and ADC2. ADC2 is not available on all devices (refer to
Section 33.3: ADC implementation).

31

30

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

Res.

Res.

15

14

13

12

11

10

9

DAMDF[1:0]

Res.

Res.
rw

rw

rw

rw

24

22

21

20

19

18

PRESC[3:0]

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

Res.

Res.

Res.
rw

rw

DELAY[3:0]
rw

23

VSENSE
VREFEN
VBATEN
SEL

rw

17

16

Res.

Res.

1

0

rw

rw

DUAL[4:0]
rw

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 VBATEN: VBAT enable
This bit is set and cleared by software to control the VBAT channel.
0: VBAT channel disabled
1: VBAT channel enabled
Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0,
JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bit 23 VSENSESEL: Temperature sensor voltage selection
This bit is set and cleared by software to control the temperature sensor channel.
0: Temperature sensor channel disabled
1: Temperature sensor channel enabled
Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0,
JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bit 22 VREFEN: VREFINT enable
This bit is set and cleared by software to enable/disable the VREFINT buffer.
0: VREFINT channel disabled
1: VREFINT channel enabled
Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0,
JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bits 21:18 PRESC[3:0]: ADC prescaler
These bits are set and cleared by software to select the frequency of the ADC clock. The
clock is common to all ADCs.
0000: input ADC clock not divided
0001: input ADC clock divided by 2
0010: input ADC clock divided by 4
0011: input ADC clock divided by 6
0100: input ADC clock divided by 8
0101: input ADC clock divided by 10
0110: input ADC clock divided by 12
0111: input ADC clock divided by 16
1000: input ADC clock divided by 32
1001: input ADC clock divided by 64
1010: input ADC clock divided by 128
1011: input ADC clock divided by 256
Others: Reserved, must not be used
Note: The software is allowed to write this bit only when the ADCs are disabled (ADCAL = 0,
JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 17:16 Reserved, must be kept at reset value.
Bits 15:14 DAMDF[1:0]: Dual ADC mode data format
This bit-field is set and cleared by software. It specifies the data format in the common data
register ADC12_CDR.
00: Dual ADC mode without data packing (ADC12_CDR and ADC12_CDR2 registers not
used).
01: Reserved.
10: Data formatting mode for 32 down to 10-bit resolution
11: Data formatting mode for 8-bit resolution
Note: This register is available only the devices that support dual mode (see Section 33.3:
ADC implementation).
The software is allowed to write these bits only when the ADCs are disabled
(ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 13:12 Reserved, must be kept at reset value.
Bits 11:8 DELAY[3:0]: Delay between the end of the master ADC sampling phase and the beginning of
the slave ADC sampling phase.
These bits are set and cleared by software. These bits are used in dual interleaved modes.
Refer to Table 321 for the value of ADC resolution versus DELAY bits values.
Note: This register is available only the devices that support dual mode (see Section 33.3:
ADC implementation).
The software is allowed to write these bits only when the ADCs are disabled
(ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 7:5 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 4:0 DUAL[4:0]: Dual ADC mode selection
These bits are written by software to select the operating mode.
All the ADCs are independent:
00000: Independent mode
The configurations 00001 to 01001 correspond to the following operating modes: dual mode,
master and slave ADCs working together:
00001: Combined regular simultaneous + injected simultaneous mode
00010: Combined regular simultaneous + alternate trigger mode
00011: Combined interleaved mode + injected simultaneous mode
00100: Reserved.
00101: Injected simultaneous mode only
00110: Regular simultaneous mode only
00111: Interleaved mode only
01001: Alternate trigger mode only
All other combinations are reserved and must not be programmed
Note: This register is available only the devices that support dual mode (see Section 33.3:
ADC implementation).
The software is allowed to write these bits only when the ADCs are disabled
(ADCAL = 0, JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

Table 321. DELAY bits versus ADC resolution

<!-- pagebreak -->

