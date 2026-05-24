RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bit 1 ADDIS: ADC disable command
This bit is set by software to disable the ADC (ADDIS command) and put it into power-down state
(OFF state).
It is cleared by hardware once the ADC is effectively disabled (ADEN is also cleared by hardware at
this time).
0: no ADDIS command ongoing
1: Write 1 to disable the ADC. Read 1 means that an ADDIS command is in progress.
Note: The software is allowed to set ADDIS only when ADEN = 1 and both ADSTART = 0 and
JADSTART = 0 (which ensures that no conversion is ongoing)
Bit 0 ADEN: ADC enable control
This bit is set by software to enable the ADC. The ADC is effectively ready to operate once the flag
ADRDY has been set.
It is cleared by hardware when the ADC is disabled, after the execution of the ADDIS command.
0: ADC is disabled (OFF state)
1: Write 1 to enable the ADC.
Note: The software is allowed to set ADEN only when all bits of ADC_CR registers are 0 (ADCAL = 0,
JADSTART = 0, ADSTART = 0, ADSTP = 0, ADDIS = 0 and ADEN = 0) except for bit
ADVREGEN which must be 1 (and the software must have wait for the startup time of the
voltage regulator)

33.6.4

ADC configuration register (ADC_CFGR1)
Address offset: 0x0C
Reset value: 0x8000 0000

31

30

29

Res.

28

27

26

rw

15

14

Res.

AUT
DLY
rw

24

23

22

JAWD1 AWD1E AWD1S
JAUTO
EN
N
GL

AWD1CH[4:0]
rw

25

rw

rw

rw

rw

rw

rw

rw

13

12

11

10

9

8

7

6

CONT

OVR
MOD

EXTEN[1:0]

rw

rw

rw

rw

21

20

Res.

JDISCE
N

5

EXTSEL[4:0]
rw

rw

rw

rw

18

17

16
DISCE
N

DISCNUM[2:0]

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
rw

19

RES[1:0]

DMNGT[1:0]

rw

rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:26 AWD1CH[4:0]: Analog watchdog 1 channel selection
These bits are set and cleared by software. They select the input channel to be guarded by the
analog watchdog.
00000: ADC analog input channel-0 monitored by AWD1
00001: ADC analog input channel-1 monitored by AWD1
.....
10011: ADC analog input channel-19 monitored by AWD1
Others: Reserved, must not be used
Note: The channel selected by AWD1CH must be also selected into the SQRi or JSQRi registers.
Software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

1377

Analog-to-digital converter (ADC12)

RM0456

Bit 25 JAUTO: Automatic injected group conversion
This bit is set and cleared by software to enable/disable automatic injected group conversion after
regular group conversion.
0: Automatic injected group conversion disabled
1: Automatic injected group conversion enabled
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no regular nor injected conversion is ongoing).
Bit 24 JAWD1EN: Analog watchdog 1 enable on injected channels
This bit is set and cleared by software
0: Analog watchdog 1 disabled on injected channels
1: Analog watchdog 1 enabled on injected channels
Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no
injected conversion is ongoing).
Bit 23 AWD1EN: Analog watchdog 1 enable on regular channels
This bit is set and cleared by software
0: Analog watchdog 1 disabled on regular channels
1: Analog watchdog 1 enabled on regular channels
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 22 AWD1SGL: Enable the watchdog 1 on a single channel or on all channels
This bit is set and cleared by software to enable the analog watchdog on the channel identified by
the AWD1CH[4:0] bits or on all the channels
0: Analog watchdog 1 enabled on all channels
1: Analog watchdog 1 enabled on a single channel
Note: The software is allowed to write these bits only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bit 21 Reserved, must be kept at reset value.
Bit 20 JDISCEN: Discontinuous mode on injected channels
This bit is set and cleared by software to enable/disable discontinuous mode on the injected
channels of a group.
0: Discontinuous mode on injected channels disabled
1: Discontinuous mode on injected channels enabled
Note: The software is allowed to write this bit only when JADSTART = 0 (which ensures that no
injected conversion is ongoing).
It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the
bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set.
Bits 19:17 DISCNUM[2:0]: Discontinuous mode channel count
These bits are written by software to define the number of regular channels to be converted in
discontinuous mode, after receiving an external trigger.
000: 1 channel
001: 2 channels
...
111: 8 channels
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no
regular conversion is ongoing).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Analog-to-digital converter (ADC12)

Bit 16 DISCEN: Discontinuous mode for regular channels
This bit is set and cleared by software to enable/disable discontinuous mode for regular channels.
0: Discontinuous mode for regular channels disabled
1: Discontinuous mode for regular channels enabled
Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden
to set both DISCEN = 1 and CONT = 1.
It is not possible to use both auto-injected mode and discontinuous mode simultaneously: the
bits DISCEN and JDISCEN must be kept cleared by software when JAUTO is set.
The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 15 Reserved, must be kept at reset value.
Bit 14 AUTDLY: Delayed conversion mode
This bit is set and cleared by software to enable/disable the auto-delayed conversion mode..
0: Auto-delayed conversion mode off
1: Auto-delayed conversion mode on
Note: The software is allowed to write this bit only when ADSTART = 0 and JADSTART = 0 (which
ensures that no conversion is ongoing).
Bit 13 CONT: Single / continuous conversion mode for regular conversions
This bit is set and cleared by software. If it is set, regular conversion takes place continuously until it
is cleared.
0: Single conversion mode
1: Continuous conversion mode
Note: It is not possible to have both discontinuous mode and continuous mode enabled: it is forbidden
to set both DISCEN = 1 and CONT = 1.
The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bit 12 OVRMOD: Overrun Mode
This bit is set and cleared by software and configure the way data overrun is managed.
0: ADC_DR register is preserved with the old data when an overrun is detected.
1: ADC_DR register is overwritten with the last conversion result when an overrun is detected.
Note: The software is allowed to write this bit only when ADSTART = 0 (which ensures that no regular
conversion is ongoing).
Bits 11:10 EXTEN[1:0]: External trigger enable and polarity selection for regular channels
These bits are set and cleared by software to select the external trigger polarity and enable the
trigger of a regular group.
00: Hardware trigger detection disabled (conversions can be launched by software)
01: Hardware trigger detection on the rising edge
10: Hardware trigger detection on the falling edge
11: Hardware trigger detection on both the rising and falling edges
Note: The software is allowed to write these bits only when ADSTART = 0 (which ensures that no
regular conversion is ongoing).

RM0456 Rev 6

<!-- pagebreak -->

