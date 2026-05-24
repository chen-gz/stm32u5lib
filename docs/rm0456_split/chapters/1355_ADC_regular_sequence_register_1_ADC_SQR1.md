19

RM0456 Rev 6

rw

rw

rw

rw

rw

rw

RM0456

Analog-to-digital converter (ADC12)

Bits 19:0 PCSEL[19:0]: Channel i (VINP[i]) preselection
These bits are written by software to preselect the input channel I/O instance to be
converted.
0: Input channel i (VINP[i]) is not preselected for conversion, the ADC conversion of this
channel shows a wrong result.
1: Input channel i (VINP[i]) is preselected for conversion
Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0
(which ensures that no conversion is ongoing).

33.6.9

ADC regular sequence register 1 (ADC_SQR1)
Address offset: 0x30
Reset value: 0x0000 0000

31

30

29

Res.

Res.

Res.

15

14

13

28

27

rw

rw

12

11

rw

rw

25

24

rw

rw

rw

10

9

8

SQ4[4:0]

SQ2[3:0]
rw

26

22

21

rw

rw

7

6

rw

rw

rw

20

18

rw

rw

rw

5

4

3

2

Res.

Res.

SQ3[4:0]

SQ1[4:0]
rw

19

Res.

Res.
rw

23

rw

17

16

Res.

SQ2[4]
rw

1

0

rw

rw

L[3:0]
rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:24 SQ4[4:0]: 4th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 4th in the
regular conversion sequence.
Bit 23 Reserved, must be kept at reset value.
Bits 22:18 SQ3[4:0]: 3rd conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 3rd in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 17 Reserved, must be kept at reset value.
Bits 16:12 SQ2[4:0]: 2nd conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 2nd in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 11 Reserved, must be kept at reset value.
Bits 10:6 SQ1[4:0]: 1st conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 1st in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bits 5:4 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 3:0 L[3:0]: Regular channel sequence length
These bits are written by software to define the total number of conversions in the regular
channel conversion sequence.
0000: 1 conversion
0001: 2 conversions
...
1111: 16 conversions
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).

33.6.10

ADC regular sequence register 2 (ADC_SQR2)
Address offset: 0x34
Reset value: 0x0000 0000

31

30

29

Res.

Res.

Res.

15

14

13

28

27

rw

rw

12

11

rw

rw

25

24

rw

rw

rw

10

9

8

SQ9[4:0]

SQ7[3:0]
rw

26

22

21

rw

rw

6

5

Res.

Res.
rw

23

7

rw

rw

19

18

rw

rw

rw

4

3

2

SQ8[4:0]

SQ6[4:0]
rw

20

Res.
rw

rw

17

16

Res.

SQ7[4]
rw

1

0

rw

rw

SQ5[4:0]
rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:24 SQ9[4:0]: 9th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 9th in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 23 Reserved, must be kept at reset value.
Bits 22:18 SQ8[4:0]: 8th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 8th in the
regular conversion sequence
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 17 Reserved, must be kept at reset value.
Bits 16:12 SQ7[4:0]: 7th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 7th in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 11 Reserved, must be kept at reset value.
Bits 10:6 SQ6[4:0]: 6th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 6th in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 5 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bits 4:0 SQ5[4:0]: 5th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 5th in the
regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).

33.6.11

ADC regular sequence register 3 (ADC_SQR3)
Address offset: 0x38
Reset value: 0x0000 0000

31

30

29

Res.

Res.

Res.

15

14

13

28

27

rw

rw

12

11

rw

rw

25

24

rw

rw

rw

10

9

8

SQ14[4:0]

SQ12[3:0]
rw

26

22

21

rw

rw

6

5

Res.

Res.
rw

23

7

rw

rw

19

18

rw

rw

rw

4

3

2

SQ13[4:0]

SQ11[4:0]
rw

20

Res.
rw

rw

17

16

Res.

SQ12[4]
rw

1

0

rw

rw

SQ10[4:0]
rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:24 SQ14[4:0]: 14th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 14th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 23 Reserved, must be kept at reset value.
Bits 22:18 SQ13[4:0]: 13th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 13th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 17 Reserved, must be kept at reset value.
Bits 16:12 SQ12[4:0]: 12th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 12th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 11 Reserved, must be kept at reset value.
Bits 10:6 SQ11[4:0]: 11th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 11th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 5 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 4:0 SQ10[4:0]: 10th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 10th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).

33.6.12

ADC regular sequence register 4 (ADC_SQR4)
Address offset: 0x3C
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

15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

SQ16[4:0]
rw

rw

rw

Res.
rw

rw

SQ15[4:0]
rw

rw

rw

Bits 31:11 Reserved, must be kept at reset value.
Bits 10:6 SQ16[4:0]: 16th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 16th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).
Bit 5 Reserved, must be kept at reset value.
Bits 4:0 SQ15[4:0]: 15th conversion in regular sequence
These bits are written by software with the channel number (0..19) assigned as the 15th in
the regular conversion sequence.
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures
that no regular conversion is ongoing).

33.6.13

ADC regular data register (ADC_DR)
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

RDATA[31:16]
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

RDATA[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:0 RDATA[31:0]: Regular data converted
These bits are read-only. They contain the conversion result from the last converted regular channel.
The data are left- or right-aligned as described in Section 33.4.27: Data management.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

33.6.14

ADC injected sequence register (ADC_JSQR)
Address offset: 0x4C
Reset value: 0x0000 0000

31

30

29

28

27

JSQ4[4:0]
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

JSQ2[0]

Res.
rw

rw

rw

26

25

24

Res.

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

rw

rw

rw

rw

JEXTEN[1:0]
rw

rw

20

19

Res.

10

JSQ1[4:0]
rw

23
JSQ3[4:0]

4

18

17

rw

rw

rw

rw

3

2

1

0

rw

rw

rw

JEXTSEL[4:0]
rw

16

JSQ2[4:1]

JL[1:0]
rw

Bits 31:27 JSQ4[4:0]: 4th conversion in the injected sequence
These bits are written by software with the channel number (0..19) assigned as the 4th in the
injected conversion sequence.
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.
Bit 26 Reserved, must be kept at reset value.
Bits 25:21 JSQ3[4:0]: 3rd conversion in the injected sequence
These bits are written by software with the channel number (0..19) assigned as the 3rd in the
injected conversion sequence.
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.
Bit 20 Reserved, must be kept at reset value.
Bits 19:15 JSQ2[4:0]: 2nd conversion in the injected sequence
These bits are written by software with the channel number (0..19) assigned as the 2nd in
the injected conversion sequence.
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.
Bit 14 Reserved, must be kept at reset value.
Bits 13:9 JSQ1[4:0]: 1st conversion in the injected sequence
These bits are written by software with the channel number (0..19) assigned as the 1st in the
injected conversion sequence.
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.
Bits 8:7 JEXTEN[1:0]: External trigger enable and polarity selection for injected channels
These bits are set and cleared by software to select the external trigger polarity and enable
the trigger of an injected group.
00: Hardware trigger detection disabled (conversions can be launched by software)
01: Hardware trigger detection on the rising edge
10: Hardware trigger detection on the falling edge
11: Hardware trigger detection on both the rising and falling edges
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 6:2 JEXTSEL[4:0]: External trigger selection for injected group
These bits select the external event used to trigger the start of conversion of an injected
group:
00000: adc_jext_trg0
00001: adc_jext_trg1
...
Refer to the ADC external trigger for injected channels in Section 33.4.2: ADC pins and
internal signals for details on trigger mapping.
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.
Bits 1:0 JL[1:0]: Injected channel sequence length
These bits are written by software to define the total number of conversions in the injected
channel conversion sequence.
00: 1 conversion
01: 2 conversions
10: 3 conversions
11: 4 conversions
Note: The software is allowed to write these bits only when JADSTART = 0 (which ensures
that no injected conversion is ongoing.

33.6.15

ADC offset y register (ADC_OFRy)
Address offset: 0x60 + 0x04 * (y - 1), (y = 1 to 4)
Reset value: 0x0000 0000

31

30

29

28

27

OFFSET_CH[4:0]

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

OFFSET[23:16]

SSAT

USAT

POSOFF

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

rw

rw

rw

rw

rw

rw

rw

OFFSET[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:27 OFFSET_CH[4:0]: Channel selection for the data offset y
These bits are written by software to define the channel to which the offset programmed into
OFFSETy[25:0] bits applies.
Note: The software is allowed to write these bits only when ADSTART = 0 and
JADSTART = 0 (which ensures that no conversion is ongoing).
If OFFSETy_EN bit is set, it is not allowed to select the same channel in different
ADC_OFRy registers.
Bit 26 SSAT: Signed saturation enable
This bit is written by software to enable or disable the Signed saturation feature.
(see Section : Data register, data alignment and offset (ADC_DR, ADC_JDRy, OFFSETy,
OFFSETy_CH, OVSS, LSHIFT, USAT, SSAT) for details).
0: Offset is subtracted maintaining data integrity and extending converted data size (9-bit
and 15-bit signed format).
1: Offset is subtracted and result is saturated to maintain converted data size.
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0
(which ensures that no conversion is ongoing).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bit 25 USAT: Unsigned saturation enable
This bit is written by software to enable or disable the unsigned saturation feature.
0: Offset is subtracted maintaining data integrity and keeping converted data size
1: Offset is subtracted and result is saturated to maintain converted data size.
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0
(which ensures that no conversion is ongoing).
Bit 24 POSOFF: offset sign
This bit is set and cleared by software to enable the positive offset.
0: Negative offset
1: Positive offset
Note: The software is allowed to write these bits only when ADSTART = 0 and
JADSTART = 0 (which ensures that no conversion is ongoing).
Bits 23:0 OFFSET[23:0]: Data offset y for the channel programmed into OFFSETy_CH[4:0] bits
These bits are written by software to define the offset y to be subtracted from the raw
converted data when converting a channel (regular or injected). The channel to which the
data offset y applies must be programmed to the OFFSETy_CH[4:0] bits. The conversion
result can be read from in the ADC_DR (regular conversion) or from in the ADC_JDRyi
registers (injected conversion).
When OFFSETy[21:0] bitfield is reset, the offset compensation is disabled.
Note: The software is allowed to write these bits only when ADSTART = 0 and
JADSTART = 0 (which ensures that no conversion is ongoing).
If several offsets (OFFSETy) point to the same channel, only the offset with the lowest y
value is considered for the subtraction.
For example, if OFFSET1_CH[4:0] = 4 and OFFSET2_CH[4:0] = 4, this is
OFFSET1[25:0] that is subtracted when converting channel 4.

33.6.16

ADC gain compensation register (ADC_GCOMP)
Address offset: 0x70
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

GCOMP

Res.

Res.

Res.

Res.

Res.

Res.

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

GCOMPCOEFF[13:0]
rw

rw

Bit 31 GCOMP: Gain compensation mode
This bit is set and cleared by software to enable the gain compensation mode.
0: Regular ADC operating mode
1: Gain compensation enabled and applied on all channels
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that
no conversion is ongoing).
Bits 30:14 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 13:0 GCOMPCOEFF[13:0]: Gain compensation coefficient
These bits are set and cleared by software to program the gain compensation coefficient.
00 1000 0000 0000: gain factor of 0.5
...
01 0000 0000 0000: gain factor of 1
10 0000 0000 0000: gain factor of 2
11 0000 0000 0000: gain factor of 3
...
The coefficient is divided by 4096 to get the gain factor ranging from 0 to 3.999756.
Note: This gain compensation is only applied when GCOMP bit of ADCx_CFGR2 register is
1.

33.6.17

ADC injected data register (ADC_JDRy)
Address offset: 0x80 + 0x04 * (y - 1), (y= 1 to 4)
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

JDATA[31:16]
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

r

r

r

r

r

JDATA[15:0]
r

Bits 31:0 JDATA[31:0]: Injected data
These bits are read-only. They contain the conversion result from injected channel y. The
data are left -or right-aligned as described in Section 33.4.27: Data management.

33.6.18

ADC analog watchdog 2 configuration register
(ADC_AWD2CR)
Address offset: 0xA0
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

rw

rw

rw

rw

rw

rw

rw

AWD2CH[19:16]

AWD2CH[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bits 19:0 AWD2CH[19:0]: Analog watchdog 2 channel selection
These bits are set and cleared by software. They enable and select the input channels to be guarded
by the analog watchdog 2.
AWD2CH[i] = 0: ADC analog input channel-i is not monitored by AWD2
AWD2CH[i] = 1: ADC analog input channel-i is monitored by AWD2
When AWD2CH[19:0] = 000..0, the analog Watchdog 2 is disabled
Note: The channels selected by AWD2CH must be also selected into the SQRi or JSQRi registers.
Software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

33.6.19

ADC analog watchdog 3 configuration register
(ADC_AWD3CR)
Address offset: 0xA4
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

rw

rw

rw

rw

rw

rw

rw

AWD3CH[19:16]

AWD3CH[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 AWD3CH[19:0]: Analog watchdog 3 channel selection
These bits are set and cleared by software. They enable and select the input channels to be guarded
by the analog watchdog 3.
AWD3CH[i] = 0: ADC analog input channel-i is not monitored by AWD3
AWD3CH[i] = 1: ADC analog input channel-i is monitored by AWD3
When AWD3CH[19:0] = 000..0, the analog Watchdog 3 is disabled
Note: The channels selected by AWD3CH must be also selected into the SQRi or JSQRi registers.
The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

33.6.20

ADC watchdog threshold register 1 (ADC_LTR1)
Address offset: 0xA8
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

15

14

13

12

11

10

9

24

23

22

21

20

19

18

17

16

LTR1[24:16]
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

rw

rw

rw

rw

rw

rw

rw

LTR1[15:0]
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

1377

Analog-to-digital converter (ADC12)

RM0456

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 LTR1[24:0]: Analog watchdog 1 lower threshold
These bits are written by software to define the lower threshold for the analog watchdog 1.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

33.6.21

ADC watchdog threshold register 1 (ADC_HTR1)
Address offset: 0xAC
Reset value: 0x01FF FFFF

31

30

29

AWDFILT1[2:0]
rw

rw

rw

15

14

13

28

27

26

25

Res.

Res.

Res.

Res.

12

11

10

9

24

23

22

21

20

19

18

17

16

HTR1[24:16]
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

rw

rw

rw

rw

rw

rw

rw

HTR1[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:29 AWDFILT1[2:0]: Analog watchdog filtering parameter
This bit is set and cleared by software.
000: No filtering
001: two consecutive detection generates an AWDx flag or an interrupt
...
111: Eight consecutive detection generates an AWDx flag or an interrupt
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bits 28:25 Reserved, must be kept at reset value.
Bits 24:0 HTR1[24:0]: Analog watchdog 1 higher threshold
These bits are written by software to define the higher threshold for the analog watchdog 1.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

33.6.22

ADC watchdog lower threshold register 2 (ADC_LTR2)
Address offset: 0xB0
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

15

14

13

12

11

10

9

24

23

22

21

20

19

18

17

16

LTR2[24:16]
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

rw

rw

rw

rw

rw

rw

rw

LTR2[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bits 24:0 LTR2[24:0]: Analog watchdog 2 lower threshold
These bits are written by software to define the lower threshold for the analog watchdog 2.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

33.6.23

ADC watchdog higher threshold register 2 (ADC_HTR2)
Address offset: 0xB4
Reset value: 0x01FF FFFF

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

24

23

22

21

20

19

18

17

16

HTR2[24:16]

9

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

rw

rw

rw

rw

rw

rw

rw

HTR2[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 HTR2[24:0]: Analog watchdog 2 higher threshold
These bits are written by software to define the higher threshold for the analog watchdog 2.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

33.6.24

ADC watchdog lower threshold register 3 (ADC_LTR3)
Address offset: 0xB8
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

15

14

13

12

11

10

24

23

22

21

rw

rw

rw

rw

8

7

6

rw

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

LTR3[24:16]

9

LTR3[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 LTR3[24:0]: Analog watchdog 3 lower threshold
These bits are written by software to define the lower threshold for the analog watchdog 3.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

33.6.25

RM0456

ADC watchdog higher threshold register 3 (ADC_HTR3)
Address offset: 0xBC
Reset value: 0x01FF FFFF

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

24

23

22

21

20

19

18

17

16

HTR3[24:16]
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

rw

rw

rw

rw

rw

rw

rw

HTR3[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 HTR3[24:0]: Analog watchdog 3 higher threshold
These bits are written by software to define the higher threshold for the analog watchdog 3.
Refer to Section 33.4.30: Analog window watchdog (AWD1EN, JAWD1EN, AWD1SGL, AWD1CH,
AWD2CH, AWD3CH, AWD_HTRy, AWD_LTRy, AWDy).

33.6.26

ADC differential mode selection register (ADC_DIFSEL)
Address offset: 0xC0
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

rw

rw

rw

rw

rw

rw

rw

DIFSEL[19:16]

DIFSEL[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:20 Reserved, must be kept at reset value.
Bits 19:0 DIFSEL[19:0]: Differential mode for channels 19 to 0
These bits are set and cleared by software. They allow selecting if a channel is configured as singleended or differential mode.
DIFSEL[i] = 0: ADC analog input channel-i is configured in single-ended mode
DIFSEL[i] = 1: ADC analog input channel-i is configured in differential mode
Note: The software is allowed to write these bits only when the ADC is disabled (ADCAL = 0,
JADSTART = 0, JADSTP = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

33.6.27

ADC user control register (ADC_CALFACT)
Address offset: 0xC4
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

25

24

CAPTURE LATCH
_COEF _COEF
rw

rw

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

VALIDITY

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

I_APB_DATA[7:0]
r

r

r

r

r

I_APB_ADDR[7:0]
r

r

r

r

r

r

r

r

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 CAPTURE_COEF: Calibration factor capture enable bit
This bit enables the internal calibration factor capture.
0: Calibration factor not captured
1: Calibration factor available in CALFACT[31:0] bits, the calibration factor index being defined by
CALINDEX[3:0] bits
Bit 24 LATCH_COEF: Calibration factor latch enable bit
This bit latches the calibration factor in the CALFACT[31:0] bits.
0: No effect
1: Calibration factor latched in the analog block on LATCH_COEF bit transition from 0 to 1. Prior to
latching the calibration factor, CALFACT[31:0] bits must be programmed with the content of
CALINDEX[3:0] bits.
Bits 23:17 Reserved, must be kept at reset value.
Bit 16 VALIDITY: Delayed write access status bit
This bit indicates the communication status between the ADC digital and analog blocks.
0: Operation still in progress
1: Operation complete
Bits 15:8 I_APB_DATA[7:0]: Delayed write access data
This bitfield contains the data that are being written during delayed write accesses.
Bits 7:0 I_APB_ADDR[7:0]: Delayed write access address
This bitfield contains the address that is being written during delayed write accesses.

33.6.28

ADC calibration factor register (ADC_CALFACT2)
Address offset: 0xC8
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

CALFACT[31:16]
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

rw

rw

rw

rw

rw

rw

rw

CALFACT[15:0]
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

