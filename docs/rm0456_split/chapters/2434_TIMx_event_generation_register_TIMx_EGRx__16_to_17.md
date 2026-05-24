RM0456 Rev 6

RM0456

Graphic timer (GFXTIM)

Event and interrupt generation
The watchdog can generate two events:
•

an alarm when the watchdog down-counter reaches 0
The watchdog counter is automatically stopped, and WDGAF (alarm flag) is set
in GFXTIM_ISR. An interrupt is generated if WDGAIE (alarm interrupt enable) is set
in GFXTIM_IER.

•

a pre-alarm when the watchdog counter matches the pre-alarm register value
in GFXTIM_WDGPAR (watchdog pre-alarm register)
WDGPF (pre-alarm flag) is set in GFXTIM_ISR, and an interrupt is generated
if WDGPIE (pre-alarm interrupt enable) is set in GFXTIM_IER.

59.4

GFXTIM interrupts
An interrupt can be produced on the following events:
•

absolute frame counter compare event

•

absolute frame counter overflow

•

absolute line counter compare events

•

absolute line counter overflow

•

relative frame counter reload events

•

external tearing effect

•

combined events

•

watchdog alarm event

•

watchdog pre-alarm event

Separate interrupt enable bits are available for flexibility.
Table 620. Graphic timer interrupt requests
Interrupt
acronym

GFXTIM

Event flag

Enable control
bit

Interrupt clear
method

Absolute frame counter overflow

AFCOF

AFCOIE

write 1 in CAFCOF

Absolute frame counter compare 1

AFCC1F

AFCC1IE

write 1 in CAFCC1F

Absolute line counter overflow

ALCOF

ALCOIE

write 1 in CALCOF

Absolute line counter compare 1

ALCC1F

ALCC1IE

write 1 in CALCC1F

Absolute line counter compare 2

ALCC2F

ALCC2IE

write 1 in CALCC2F

Relative frame counter 1 reload

RFC1RF

RFC1RIE

write 1 in CRFC1RF

Relative frame counter 2 reload

RFC2RF

RFC2RIE

write 1 in CRFC2RF

External tearing effect

TEF

TEIE

write 1 in CTEF

Event 1

EV1F

EV1IE

write 1 in CEV1F

Event 2

EV2F

EV2IE

write 1 in CEV2F

Event 3

EV3F

EV3IE

write 1 in CEV3F

Event 4

EV4F

EV4IE

write 1 in CEV4F

Interrupt event

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Table 620. Graphic timer interrupt requests (continued)
Interrupt
acronym
GFXTIMW

Event flag

Enable control
bit

Interrupt clear
method

Watchdog alarm

WDGAF

WDGAIE

write 1 in CWDGAF

Watchdog pre-alarm

WDGPF

WDGPIE

write 1 in CWDGPF

Interrupt event

59.5

GFXTIM registers

59.5.1

GFXTIM configuration register (GFXTIM_CR)
Address offset: 0x000
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

Res.

Res.

Res.

Res.

Res.

Res.

SYNCS[1:0]

Res.

Res.

Res.

TEPOL

Res.

Res.

rw

rw

17

LCCOE FCCOE

rw

Bits 31:18 Reserved, must be kept at reset value.
Bit 17 LCCOE: Line-clock calibration output enable
This bit enables the line-clock output.
0: Line-clock output disabled
1: Line-clock output enabled
Bit 16 FCCOE: Frame-clock calibration output enable
This bit enables the frame-clock output.
0: Frame-clock output disabled
1: Frame-clock output enabled
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:8 SYNCS[1:0]: Synchronization source
This bitfield selects the synchronization signals (HSYNC and VSYNC) sources.
00: gfxtim_hsync[0] and gfxtim_vsync[0] selected
01: gfxtim_hsync[1] and gfxtim_vsync[1] selected
10: gfxtim_hsync[2] and gfxtim_vsync[2] selected
11: gfxtim_hsync[3] and gfxtim_vsync[3] selected
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 TEPOL: Tearing--effect polarity
This bit selects the tearing-effect polarity.
0: Tearing effect active on rising edge
1: Tearing effect active on falling edge
Bits 3:2 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

16

rw

rw

1

0

TES[1:0]
rw

rw

RM0456

Graphic timer (GFXTIM)

Bits 1:0 TES[1:0]: Tearing source
This bitfield selects the tearing-effect source.
00: TE input pad selected
01: gfxtim_ite selected
10: HSYNC input selected by SYNCS[1:0]
11: VSYNC input selected by SYNCS[1:0]

59.5.2

GFXTIM clock generator configuration register (GFXTIM_CGCR)
Address offset: 0x004
Reset value: 0x0000 0000

31

30

Res.

15

29

28

FCCHRS[2:0]
rw

rw

rw

14

13

12

Res.

LCCHRS[2:0]
rw

rw

rw

27

26

25

24

23

Res.

Res.

Res.

FCCFR

Res.
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

Res.

Res.

Res.

LCCFR

Res.

Res.

Res.

LCCCS

Res.

w

w

22

21

20

FCCCS[2:0]

19

18

Res.

rw

17

16

FCS[2:0]
rw

rw

rw

2

1

0

LCS[2:0]
rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bits 30:28 FCCHRS[2:0]: Frame clock counter hardware reload source
This bitfield configures the hardware reload source for the frame clock counter.
000: No hardware reload
001: Line- -clock counter underflow
010: HSYNC rising edge
011: HSYNC falling edge
100: VSYNC rising edge
101: VSYNC falling edge
110: TE rising edge
111: TE falling edge
Bits 27:25 Reserved, must be kept at reset value.
Bit 24 FCCFR: Frame clock counter force reload
This bit forces frame clock counter reload
0: No effect
1: Frame clock counter reload forced
Bit 23 Reserved, must be kept at reset value.
Bits 22:20 FCCCS[2:0]: Frame clock counter clock source
This bitfield configures the clock source for the frame clock counter.
000: Frame clock counter disabled
001: Line clock counter underflow
010: HSYNC rising edge
011: HSYNC falling edge
100: VSYNC rising edge
101: VSYNC falling edge
110: TE rising edge
111: TE falling edge
Bit 19 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2562

Graphic timer (GFXTIM)

RM0456

Bits 18:16 FCS[2:0]: Frame clock source
This bitfield configures the frame clock source.
000: Line clock counter underflow
001: Frame clock counter underflow
010: HSYNC rising edge
011: HSYNC falling edge
100: VSYNC rising edge
101: VSYNC falling edge
110: TE rising edge
111: TE falling edge
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 LCCHRS[2:0]: Line clock counter hardware reload source
This bitfield configures the hardware reload source for the line clock counter.
000: No hardware reload
001: Frame clock counter underflow
010: HSYNC rising edge
011: HSYNC falling edge
100: VSYNC rising edge
101: VSYNC falling edge
110: TE rising edge
111: TE falling edge
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 LCCFR: Line clock counter force reload
This bit forces line clock counter reload.
0: No effect
1: Line clock counter reload forced
Bits 7:5 Reserved, must be kept at reset value.
Bit 4 LCCCS: Line clock counter clock source
This bit configures the clock source for the line clock counter.
0: Line clock counter disabled
1: System clock selected
Bit 3 Reserved, must be kept at reset value.
Bits 2:0 LCS[2:0]: Line clock source
This bitfield configures the line clock source.
000: Line clock counter underflow
001: Frame clock counter underflow
010: HSYNC rising edge
011: HSYNC falling edge
100: VSYNC rising edge
101: VSYNC falling edge
110: TE rising edge
111: TE falling edge

<!-- pagebreak -->

