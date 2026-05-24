RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

34.7.4

ADC configuration register 1 (ADC_CFGR1)
Address offset: 0x0C
Reset value: 0x0000 0000

31

30

29

Res.

28

27

26

AWD1CH[4:0]
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

Res.

WAIT CONT OVRMOD
rw

rw

rw

EXTEN[1:0]
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

Res.

Res.

AWD1
EN

AWD1
SGL

CHSEL
RMOD

Res.

Res.

Res.

Res.

DISCEN

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

ALIGN

SCAN
DIR

DMAC
FG

DMAEN

rw

rw

rw

rw

Res.

EXTSEL[2:0]
rw

rw

rw

rw

RES[1:0]
rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:26 AWD1CH[4:0]: Analog watchdog channel selection
These bits are set and cleared by software. They select the input channel to be guarded by the
analog watchdog.
00000: ADC analog input Channel 0 monitored by AWD
00001: ADC analog input Channel 1 monitored by AWD
.....
10001: ADC analog input Channel 17 monitored by AWD
10110: ADC analog input Channel 22 monitored by AWD
10111: ADC analog input Channel 23 monitored by AWD
Others: Reserved
Note: The channel selected by the AWDCH[4:0] bits must be also set into the CHSELR register.
The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bits 25:24

Reserved, must be kept at reset value.

Bit 23 AWD1EN: Analog watchdog enable
This bit is set and cleared by software.
0: Analog watchdog 1 disabled
1: Analog watchdog 1 enabled
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 22 AWD1SGL: Enable the watchdog on a single channel or on all channels
This bit is set and cleared by software to enable the analog watchdog on the channel identified
by the AWDCH[4:0] bits or on all the channels
0: Analog watchdog 1 enabled on all channels
1: Analog watchdog 1 enabled on a single channel
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 21 CHSELRMOD: Mode selection of the ADC_CHSELR register
This bit is set and cleared by software to control the ADC_CHSELR feature:
0: Each bit of the ADC_CHSELR register enables an input
1: ADC_CHSELR register is able to sequence up to 8 channels
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bits 20:17 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1443

Analog-to-digital converter (ADC4)

RM0456

Bit 16 DISCEN: Discontinuous mode
This bit is set and cleared by software to enable/disable discontinuous mode.
0: Discontinuous mode disabled
1: Discontinuous mode enabled
Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is
forbidden to set both bits DISCEN = 1 and CONT = 1.
The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 15 Reserved, must be kept at reset value.
Bit 14 WAIT: Wait conversion mode
This bit is set and cleared by software to enable/disable wait conversion mode..
0: Wait conversion mode off
1: Wait conversion mode on
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 13 CONT: Single / continuous conversion mode
This bit is set and cleared by software. If it is set, conversion takes place continuously until it is
cleared.
0: Single conversion mode
1: Continuous conversion mode
Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is
forbidden to set both bits DISCEN = 1 and CONT = 1.
The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 12 OVRMOD: Overrun management mode
This bit is set and cleared by software and configure the way data overruns are managed.
0: ADC_DR register is preserved with the old data when an overrun is detected.
1: ADC_DR register is overwritten with the last conversion result when an overrun is detected.
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bits 11:10 EXTEN[1:0]: External trigger enable and polarity selection
These bits are set and cleared by software to select the external trigger polarity and enable the
trigger.
00: Hardware trigger detection disabled (conversions can be started by software)
01: Hardware trigger detection on the rising edge
10: Hardware trigger detection on the falling edge
11: Hardware trigger detection on both the rising and falling edges
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 9 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC4)

Bits 8:6 EXTSEL[2:0]: External trigger selection
These bits select the external event used to trigger the start of conversion (refer to table ADC
interconnection in Section 34.4.2: ADC pins and internal signals for details):
000: adc_trg0
001: adc_trg1
010: adc_trg2
011: adc_trg3
100: adc_trg4
101: adc_trg5
110: adc_trg6
111: adc_trg7
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 5 ALIGN: Data alignment
This bit is set and cleared by software to select right or left alignment. Refer to Figure 291: Data
alignment and resolution (oversampling disabled: OVSE = 0)
0: Right alignment
1: Left alignment
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bit 4 SCANDIR: Scan sequence direction
This bit is set and cleared by software to select the direction in which the channels is scanned in
the sequence. It is effective only if CHSELRMOD bit is cleared to 0.
0: Upward scan (from CHSEL0 to CHSEL23)
1: Backward scan (from CHSEL23 to CHSEL0)
Note: The software is allowed to write this bit only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).
Bits 3:2 RES[1:0]: Data resolution
These bits are written by software to select the resolution of the conversion.
00: 12 bits
01: 10 bits
10: 8 bits
11: 6 bits
Note: The software is allowed to write these bits only when ADSTART bit is cleared to 0 by writing
ADSTP to 1 (this ensures that no conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

