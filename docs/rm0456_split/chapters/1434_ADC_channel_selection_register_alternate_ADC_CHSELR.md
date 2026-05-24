1443

Analog-to-digital converter (ADC4)

34.7.9

RM0456

ADC channel selection register [alternate] (ADC_CHSELR)
Address offset: 0x28
Reset value: 0x0000 0000
The same register can be used in two different modes:
– Each ADC_CHSELR bit enables an input (CHSELRMOD = 0 in ADC_CFGR1). Refer to the current
section.
– ADC_CHSELR is able to sequence up to 8 channels (CHSELRMOD = 1 in ADC_CFGR1). Refer to
next section.

CHSELRMOD = 0 in ADC_CFGR1
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

23

22

21

20

19

18

17

16

CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL
23
22
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

CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL CHSEL
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

rw

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:0 CHSELx: Channel x selection (x = 23 to 0)
These bits are written by software and define which channels are part of the sequence of channels
to be converted.
0: Input Channel-x is not selected for conversion
1: Input Channel-x is selected for conversion
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).

34.7.10

ADC channel selection register [alternate] (ADC_CHSELR)
Address offset: 0x28
Reset value: 0x0000 0000
The same register can be used in two different modes:
– Each ADC_CHSELR bit enables an input (CHSELRMOD = 0 in ADC_CFGR1). Refer to the current
previous section.
– ADC_CHSELR is able to sequence up to 8 channels (CHSELRMOD = 1 in ADC_CFGR1). Refer to
this section.

CHSELRMOD = 1 in ADC_CFGR1
31

30

29

28

27

SQ8[3:0]

26

25

24

23

SQ7[3:0]

22

21

20

19

SQ6[3:0]

18

17

16

SQ5[3:0]

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

SQ4[3:0]
rw

rw

<!-- pagebreak -->

rw

SQ3[3:0]
rw

rw

rw

rw

SQ2[3:0]
rw

rw

RM0456 Rev 6

rw

rw

SQ1[3:0]
rw

rw

rw

rw

rw

RM0456

Analog-to-digital converter (ADC4)

Bits 31:28 SQ8[3:0]: 8th conversion of the sequence
These bits are programmed by software with the channel number assigned to the 8th conversion of
the sequence. 0b1111 indicates the end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
0000: CH0
0001: CH1
...
1010: CH10
1011: CH11
1100: CH12
1101: CH13
1110: CH14
1111: No channel selected (End of sequence)
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 27:24 SQ7[3:0]: 7th conversion of the sequence
These bits are programmed by software with the channel number assigned to the 7th conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 23:20 SQ6[3:0]: 6th conversion of the sequence
These bits are programmed by software with the channel number assigned to the 6th conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 19:16 SQ5[3:0]: 5th conversion of the sequence
These bits are programmed by software with the channel number assigned to the 5th conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 15:12 SQ4[3:0]: 4th conversion of the sequence
These bits are programmed by software with the channel number assigned to the 4th conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1(which ensures that no conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bits 11:8 SQ3[3:0]: 3rd conversion of the sequence
These bits are programmed by software with the channel number assigned to the 3rd conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 7:4 SQ2[3:0]: 2nd conversion of the sequence
These bits are programmed by software with the channel number assigned to the 2nd conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).
Bits 3:0 SQ1[3:0]: 1st conversion of the sequence
These bits are programmed by software with the channel number assigned to the 1st conversion of
the sequence. 0b1111 indicates end of the sequence.
When 0b1111 (end of sequence) is programmed to the lower sequence channels, these bits are
ignored.
Refer to SQ8[3:0] for a definition of channel selection.
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing).

34.7.11

ADC watchdog threshold register (ADC_AWD3TR)
Address offset: 0x2C
Reset value: 0x0FFF 0000

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

HT3[11:0]
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

LT3[11:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bits 27:16 HT3[11:0]: Analog watchdog 3 higher threshold
These bits are written by software to define the higher threshold for the analog watchdog.
Refer to Section 34.4.25: Analog window watchdog.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:0 LT3[11:0]: Analog watchdog 3lower threshold
These bits are written by software to define the lower threshold for the analog watchdog.
Refer to Section 34.4.25: Analog window watchdog.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

34.7.12

ADC data register (ADC_DR)
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

r

r

r

r

r

r

r

DATA[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:0 DATA[15:0]: Converted data
These bits are read-only. They contain the conversion result from the last converted channel. The data
are left- or right-aligned as shown in Figure 291: Data alignment and resolution (oversampling
disabled: OVSE = 0).
Just after a calibration is complete, DATA[6:0] contains the calibration factor.

34.7.13

ADC power register (ADC_PWRR)
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

DPD

AUTOF
F

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

VREFS
VREFP
ECSM
ROT
P
rw

RM0456 Rev 6

rw

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 VREFSECSMP: VREF+ second sample bit
This bit is set and cleared by software. It is used to enable/disable the second VREF+ protection
when multiple ADCs are working simultaneously and a clock divider of 1 is used.
0: VREF+ second sample disabled
1: VREF+ second sample enabled
Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no
conversion is ongoing).
Bit 2 VREFPROT: VREF+ protection bit
This bit is set and cleared by software. It is used to enable/disable VREF+ protection when multiple
ADCs are working simultaneously and a clock divider is used.
0: VREF+ protection disabled
1: VREF+ protection enabled
Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no
conversion is ongoing).
Bit 1 DPD: Deep-power-down mode bit
This bit is set and cleared by software. It is used to enable/disable Deep-power-down mode in
autonomous mode when the ADC is not used.
0: Deep-power-down mode disabled
1: Deep-power-down mode enabled
Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no
conversion is ongoing).
Setting DPD in auto-off mode automatically disables the LDO.
Bit 0 AUTOFF: Auto-off mode bit
This bit is set and cleared by software. it is used to enable/disable the auto-off mode.
0: Auto-off mode disabled
1: Auto-off mode enabled
Note: The software is allowed to write this bit only when ADEN bit is cleared to 0 (this ensures that no
conversion is ongoing).

34.7.14

ADC Analog Watchdog 2 Configuration register (ADC_AWD2CR)
Address offset: 0xA0
Reset value: 0x0000 0000

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25
Res.

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

AWD2
CH23

AWD2
CH22

AWD2
CH21

AWD2
CH20

AWD2
CH19

AWD2
CH18

AWD2
CH17

AWD2
CH16

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

AWD2
CH15

AWD2
CH14

AWD2
CH13

AWD2
CH12

AWD2
CH11

AWD2
CH10

AWD2
CH9

AWD2
CH8

AWD2
CH7

AWD2
CH6

AWD2
CH5

AWD2
CH4

AWD2
CH3

AWD2
CH2

AWD2
CH1

AWD2
CH0

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

Bits 31:24 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

Bits 23:0 AWD2CHx: Analog watchdog channel selection (x = 23 to 0)
These bits are set and cleared by software. They enable and select the input channels to be guarded
by analog watchdog 2 (AWD2).
0: ADC analog channel-x is not monitored by AWD2
1: ADC analog channel-x is monitored by AWD2
Note: The channels selected through ADC_AWD2CR must be also configured into the
ADC_CHSELR registers. Refer to SQ8[3:0] for a definition of channel selection. The software is
allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which
ensures that no conversion is ongoing).

34.7.15

ADC Analog Watchdog 3 Configuration register (ADC_AWD3CR)
Address offset: 0xA4
Reset value: 0x0000 0000

31
Res.

30
Res.

29
Res.

28
Res.

27

26

Res.

Res.

25
Res.

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

AWD3
CH23

AWD3
CH22

AWD3
CH21

AWD3
CH20

AWD3
CH19

AWD3
CH18

AWD3
CH17

AWD3
CH16

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

AWD3
CH15

AWD3
CH14

AWD3
CH13

AWD3
CH12

AWD3
CH11

AWD3
CH10

AWD3
CH9

AWD3
CH8

AWD3
CH7

AWD3
CH6

AWD3
CH5

AWD3
CH4

AWD3
CH3

AWD3
CH2

AWD3
CH1

AWD3
CH0

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

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:0 AWD3CHx: Analog watchdog channel selection (x = 23 to 0)
These bits are set and cleared by software. They enable and select the input channels to be guarded
by analog watchdog 3 (AWD3).
0: ADC analog channel-x is not monitored by AWD3
1: ADC analog channel-x is monitored by AWD3
Note: The channels selected through ADC_AWD3CR must be also configured into the
ADC_CHSELR registers. Refer to SQ8[3:0] for a definition of channel selection. The software is
allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1 (which
ensures that no conversion is ongoing).

34.7.16

ADC Calibration factor (ADC_CALFACT)
Address offset: 0xC4
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

rw

CALFACT[6:0]
rw

RM0456 Rev 6

rw

rw

rw

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bits 31:7 Reserved, must be kept at reset value.
Bits 6:0 CALFACT[6:0]: Calibration factor
These bits are written by hardware or by software.
– Once a calibration is complete, they are updated by hardware with the calibration factors.
– Software can write these bits with a new calibration factor. If the new calibration factor is different
from the current one stored into the analog ADC, it is then applied once a new calibration is
launched.
– Just after a calibration is complete, DATA[6:0] contains the calibration factor.
Note: Software can write these bits only when ADEN = 1 (ADC is enabled and no calibration is
ongoing and no conversion is ongoing).

34.7.17

ADC option register (ADC_OR)
Address offset: 0xD0
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

Res.

CHN21
SEL

Res.

Res.

Res.

Res.

Res.

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

Bits 31:1 Reserved, must be kept at reset value.
Bit 0 CHN21SEL: Channel 21 selection bit
This bit is set and cleared by software. It is used to select the internal source connected to ADC input
channel 21:
0: dac1_out1 selected
1: dac1_out2 selected
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 = 0 (which ensures that no conversion is ongoing).

34.7.18

ADC common configuration register (ADC_CCR)
Address offset: 0x308
Reset value: 0x0000 0000

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

<!-- pagebreak -->

24

23

22

21

VBAT VSENSE VREF
EN
SEL
EN

RM0456 Rev 6

20

19

18

17

16

Res.

Res.

2

1

0

Res.

Res.

Res.

PRESC[3:0]

RM0456

Analog-to-digital converter (ADC4)

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 VBATEN: VBAT enable
This bit is set and cleared by software to enable/disable the VBAT channel.
0: VBAT channel disabled
1: VBAT channel enabled
Note: The software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP
to 1 (which ensures that no conversion is ongoing)
Bit 23 VSENSESEL: Temperature sensor selection
This bit is set and cleared by software to enable/disable the temperature sensor.
0: Temperature sensor disabled
1: Temperature sensor enabled
Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1
(which ensures that no conversion is ongoing).
Bit 22 VREFEN: VREFINT enable
This bit is set and cleared by software to enable/disable the VREFINT buffer.
0: VREFINT disabled
1: VREFINT enabled
Note: Software is allowed to write this bit only when ADSTART is cleared to 0 by writing ADSTP to 1
(which ensures that no conversion is ongoing).
Bits 21:18 PRESC[3:0]: ADC prescaler
Set and cleared by software to select the frequency of the clock to the ADC.
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
Other: Reserved
Note: Software is allowed to write these bits only when the ADC is disabled (ADCAL = 0,
ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).
Bits 17:0 Reserved, must be kept at reset value.

34.7.19

ADC register map
The following table summarizes the ADC registers.

3

2

1

0

EOC

EOSMP

ADRDY

0

4

0

EOS

0

5

0

OVR

14
Res.

6

15
Res.

0

Res.

16
Res.

7

17
Res.

Res.

18
Res.

8

19
Res.

AWD1

20
Res.

9

21
Res.

AWD2

22
Res.

AWD3

23
Res.

11

24
Res.

10

25
Res.

Res.

26
Res.

12

27
Res.

EOCAL

28
Res.

RM0456 Rev 6

13

29
Res.

Reset value

LDORDY

30

ADC_ISR

Res.

31

0x00

Register name

Res.

Offset

Res.

Table 339. ADC register map and reset values

0

0

0

0

0

<!-- pagebreak -->

1443

0x44

ADC_PWRR

0x48 0xBF

<!-- pagebreak -->

Reserved

RM0456 Rev 6

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

VREFSECSMP

VREFPROT

DPD

AUTOFF

Reset value

0

Res.

Reset value

Res.

Reserved
1

Res.

1
0

Res.

1
0

Res.

HT3[11:0]

Res.

1
0
0

SQ5[3:0]

Reserved
0
0

CHSEL0

0
0
CHSEL1

0
0
CHSEL2

0
0

CHSEL3

0
0

CHSEL4

0
0

CHSEL5

0
0

CHSEL6

0
0

CHSEL7

0
0

CHSEL8

0
0

CHSEL9

0
0

CHSEL10

1
Res.

SMPSEL0

CHSELRMOD

CONT
OVRMOD
EXTEN[1:0]

SQ4[3:0]

0
0
0
0

1
0

0
0
0
0

0

DMAEN

0
0

OVSS[3:0]
OVSE

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

0

0

0

0

SQ3[3:0]

0

0

0

0

0

0

0

RES[1:0]

0
SCANDIR

0

0

SMP2
[2:0]
0

0

0

0
0

Res.

0
ALIGN

EXTSEL[2:0]

DMACFG

0
0

OVSR
[2:0]
Res.

0
TOVS

Res.
Res.

3
2
1
0

EOSMPIE
ADRDYIE

0
0
0

0

0

SQ2[3:0]

ADEN

0
ADDIS

0

Res.

EOCIE

12

Res.

0
0
ADSTART

13

Res.

4

14

Res.

EOSIE

15

Res.

5

16

Res.

OVRIE

17

Res.

6

18

Res.

Res.

19

Res.

7

20

Res.

Res.

21

Res.

8

22

Res.

AWD1IE

23

Res.

9

24

Res.

AWD2IE

25

Res.

AWD3IE

26

Res.

11

27

Res.

10

28

Res.

Res.

29

Res.

0
ADSTP

Res.

Res.

Res.

Res.

Res.

30

Res.

LDORDYIE
EOCALIE
0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

WAIT

Res.

DISCEN

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

AWD1SGL
Res.

AWD1EN

Res.

Res.

Res.

Res.

ADVREGEN

31

ADC_IER

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

AWDCH[4:0]

Res.

Res.

ADCAL

Res.

Register name

Res.

SMPSEL1

1
SMPSEL2

0

SMPSEL3

0

SMPSEL4

0

Res.

0
SMPSEL5

Reserved

Res.

0

Res.

0

Res.

CHSEL13

SMPSEL8

0
SMPSEL6

SMPSEL9

0

Res.

SMPSEL10

0
SMPSEL7

SMPSEL11

0

Res.

HT1[11:0]
Res.

SMPSEL12

0

CHSEL11

CHSEL14

HT2[11:0]
Res.

SMPSEL13

0

Res.

1

CHSEL15

SMPSEL14

LFTRIG
Res.

Res.

Res.

0

CHSEL12

1

CHSEL16

SMPSEL15

0

Res.

1

CHSEL17

SMPSEL16

0

Res.

1

CHSEL18

SMPSEL17

0

Res.

1

CHSEL19

SMPSEL18.

0

Res.

1

CHSEL20

SMPSEL19.

0

Res.

1

CHSEL21

SMPSEL20.

0

Res.

1

CHSEL22

SMPSEL21.

0

Res.

1

CHSEL23

SMPSEL22

Reset value

Res.

1
0

Res.

1
0

Res.

SQ6[3:0]
1

Res.

1

Res.

Reserved

Res.

1
0
1

Res.

0
1

Res.

0
1

Res.

1

Res.

Res.

ADC_SMPR
SMPSEL23.

0

Res.

1

Res.

Res.

Res.

Res.

0

Res.

1
0

Res.

SQ7[3:0]

Res.

1
0
1

Res.

1
0
1

Res.

1
0
1

Res.

Reset value
0
1

Res.

0
0

Res.

0

Res.

0

Res.

0
1

Res.

SQ8[3:0]

Res.

Reset value

Res.

Res.

Res.

1

Res.

ADC_DR
Res.

Reset value
0

Res.

0x40
Res.

Reset value
0

Res.

0x30 0x3C
0

Res.

ADC_AWD3TR
Res.

Reset value
ADC_CHSELR
(CHSELRMOD
= 1)
Reset value

Res.

ADC_CHSELR
(CHSELRMOD =
0)
0

Res.

0x2C
ADC_AWD2TR
Res.

Reset value

Res.

0x28
ADC_CFGR1

Res.

0x28
0

Res.

0x24
ADC_AWD1TR

Res.

0x20
Reset value

Res.

0x180x1C

Res.

0x14
ADC_CFGR2

Res.

0x10

Res.

0x0C
ADC_CR

Res.

0x08

Res.

0x04

Res.

Offset

Res.

Analog-to-digital converter (ADC4)
RM0456

Table 339. ADC register map and reset values (continued)

0
0
0

0
0
0

SMP1
[2:0]

0

0
0
0

LT1[11:0]

LT2[11:0]

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
0

SQ1[3:0]

LT3[11:0]
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

Reserved

DATA[15:0]

0

0

0

0

0x308
ADC_CCR
PRESC3
PRESC2
PRESC1
PRESC0

0
0
0
0
0
0
0

RM0456 Rev 6

2
1
0

AWD2CH2
AWD2CH1
AWD2CH0

AWD3CH20
AWD3CH19
AWD3CH18
AWD3CH17
AWD3CH16

0
0
0
0
0
0
Reserved

0

AWD3CH5
AWD3CH4
AWD3CH3
AWD3CH2
AWD3CH1
AWD3CH0

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

Res.
Res.
Res.
Res.
CHN21SEL

Reserved

0

Res.

Reset value
AWD3CH6

0

Res.

0

Reset value

Res.

3

AWD2CH3

AWD3CH21

0
0

Res.

4

AWD2CH4

0

0

Res.

5

AWD2CH5

0

0

Res.

6

AWD2CH6

0

0

Res.

7

AWD2CH7

0

0

Res.

8

AWD2CH8

0

Res.

9

AWD2CH9

0

AWD3CH8

0

AWD3CH9

0

Res.

11
10

AWD2CH10

0

Res.

12
AWD2CH11

0

AWD3CH10

13
AWD2CH12

0

Res.

14
AWD2CH13

0

AWD3CH11

15
AWD2CH14

0

Res.

16
AWD2CH15

0
AWD3CH12

17
AWD2CH16

0

Res.

18
AWD2CH17

0
AWD3CH13

19
AWD2CH18

0

Res.

20
AWD2CH19

0
AWD3CH14

21
AWD2CH20

0

Res.

22
AWD2CH21

0
AWD3CH15

Res.

Res.

23
AWD2CH22

25

Res.

0

AWD3CH22

26

Res.

0
AWD3CH7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

27

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

Reserved

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

VREFEN

28

Res.

24

29

Res.

AWD2CH23

30

Res.

0

AWD3CH23

Res.

Res.

Res.

31

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

ADC_AWD2CR

VSENSESEL

Res.

Res.

Res.

Res.

Res.

Res.

Register name

VBATEN

Reset value
Res.

ADC_OR

Res.

0xD0
Res.

0xB8 0xCF
ADC_CALFACT

Res.

0xC4
Res.

0xA4 0xC0
ADC_AWD3CR

Res.

0xA4

Res.

0xA0

Res.

Offset

Res.

RM0456
Analog-to-digital converter (ADC4)

Table 339. ADC register map and reset values (continued)

CALFACT[6:0]

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

