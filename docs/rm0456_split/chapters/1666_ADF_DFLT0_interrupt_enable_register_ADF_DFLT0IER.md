1674

Audio digital filter (ADF)

40.8.9

RM0456

ADF DFLT0 interrupt enable register (ADF_DFLT0IER)
Address offset: 0x0AC
Reset value: 0x0000 0000
This register is used for allowing or not the events to generate an interrupt.

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

DOVRI
E

FTHIE

rw

rw

Res.

Res.

SDLVLI SDDET RFOVR CKABI
E
IE
IE
E
rw

rw

rw

rw

SATIE

Res.

Res.

Res.

rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 SDLVLIE: SAD sound-level value ready enable
This bit is set and cleared by software.
0: Sound-level-ready interrupt disabled
1: Sound-level-ready interrupt enabled
Bit 12 SDDETIE: Sound activity detection interrupt enable
This bit is set and cleared by software.
0: Sound-trigger interrupt disabled
1: Sound-trigger interrupt enabled
Bit 11 RFOVRIE: Reshape filter overrun interrupt enable
This bit is set and cleared by software.
0: Reshape filter overrun interrupt disabled
1: Reshape filter overrun interrupt enabled
Bit 10 CKABIE: Clock absence detection interrupt enable
This bit is set and cleared by software.
0: Clock absence interrupt disabled
1: Clock absence interrupt enabled
Bit 9 SATIE: Saturation detection interrupt enable
This bit is set and cleared by software.
0: Saturation interrupt disabled
1: Saturation interrupt enabled
Bits 8:2 Reserved, must be kept at reset value.
Bit 1 DOVRIE: Data overflow interrupt enable
This bit is set and cleared by software.
0: Data overflow interrupt disabled
1: Data overflow interrupt enabled
Bit 0 FTHIE: RXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: RXFIFO threshold interrupt disabled
1: RXFIFO threshold interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

Res.

Res.

Res.

RM0456

Audio digital filter (ADF)

40.8.10

ADF DFLT0 interrupt status register 0 (ADF_DFLT0ISR)
Address offset: 0x0B0
Reset value: 0x0000 0000
This register contains the status flags for the digital filter path.

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

SDLVL SDDET RFOVR
CKABF
F
F
F

SATF

Res.

Res.

Res.

Res.

Res.

RXNEF

Res.

DOVRF

FTHF

rc_w1

rc_w1

rc_w1

r

rc_w1

rc_w1

rc_w1

r

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 SDLVLF: Sound level value ready flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that new sound level value is not ready. Write 0 has no effect.
1: Read 1 means that new sound level value is ready. Write 1 clears this flag.
Bit 12 SDDETF: Sound activity detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no sound activity is detected. Write 0 has no effect.
1: Read 1 means that sound activity is detected. Write 1 clears this flag.
Bit 11 RFOVRF: Reshape filter overrun detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no reshape filter overrun is detected. Write 0 has no effect.
1: Read 1 means that reshape filter overrun is detected. Write 1 clears this flag.
Bit 10 CKABF: Clock absence detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no clock absence is detected. Write 0 has no effect.
1: Read 1 means that a clock absence is detected. Write 1 clears this flag.
Bit 9 SATF: Saturation detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no saturation is detected. Write 0 has no effect.
1: Read 1 means that a saturation is detected. Write 1 clears this flag.
Bits 8:4 Reserved, must be kept at reset value.
Bit 3 RXNEF: RXFIFO not empty flag
This bit is set and cleared by hardware according to the RXFIFO level.
0: RXFIFO empty
1: RXFIFO not empty
Bit 2 Reserved, must be kept at reset value.
Bit 1 DOVRF: Data overflow flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no overflow is detected. Write 0 has no effect.
1: Read 1 means that an overflow is detected; Write 1 clears this flag.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Bit 0 FTHF: RXFIFO threshold flag
This bit is set by hardware, and cleared by the hardware when the RXFIFO level is lower than
the threshold.
0: RXFIFO threshold not reached
1: RXFIFO threshold reached

40.8.11

ADF SAD control register (ADF_SADCR)
Address offset: 0x0B8
Reset value: 0x0000 0000
This register is used for the configuration and the control of the sound activity detection.

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

SADAC
TIVE

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
15
Res.

Res.

SADMOD[1:0]
rw

rw

Res.

HYSTE
N

FRSIZE[2:0]
rw

rw

rw

rw

Res.

SADST[1:0]
r

r

DETCF
G
rw

DATCAP[1:0]
rw

rw

SADEN
rw

Bit 31 SADACTIVE: SAD Active flag
This bit is set and cleared by hardware. It is used to check if the SAD is effectively enabled
(active) or not. The protected fields and registers of this function can only be updated when
the SADACTIVE is set to 0 (see Section 40.4.13: Register protection for details).
The delay between a transition on SADEN and a transition on SADACTIVE is two periods of
AHB clock and two periods of adf_proc_ck.
0: SAD not active and can be configured if needed
1: SAD active and protected fields cannot be configured.
Bits 30:14 Reserved, must be kept at reset value.
Bits 13:12 SADMOD[1:0]: SAD working mode
This bitfield is set and cleared by software. It is used to define the way the SAD works.
00: Threshold value computed according to the estimated ambient noise
The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this
mode, the SAD works like a voice activity detector.
01: Threshold value equal to ANMIN[12:0], multiplied by the gain selected by SNTHR[3:0]
The SAD triggers when the sound level (SDLVL) is bigger than the defined threshold. In this
mode, the SAD works like a sound detector.
1x: Threshold value given by 4 x ANMIN[12:0]
The SAD triggers when the estimated ambient noise (ANLVL), multiplied by the gain selected
by SNTHR[3:0] is bigger than the defined threshold. In this mode, the SAD is working like an
ambient noise estimator. Hysteresis function cannot be used in this mode.
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 11 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Audio digital filter (ADF)

Bits 10:8 FRSIZE[2:0]: Frame size
This bitfield is set and cleared by software. it is used to define the size of one frame and also
to define how many samples are taken into account to compute the short-term signal level.
000: 8 PCM samples used to compute the short-term signal level
001: 16 PCM samples used to compute the short-term signal level
010: 32 PCM samples used to compute the short-term signal level
011: 64 PCM samples used to compute the short-term signal level
100: 128 PCM samples used to compute the short-term signal level
101: 256 PCM samples used to compute the short-term signal level
11x: 512 PCM samples used to compute the short-term signal level
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 7 HYSTEN: Hysteresis enable
This bit is set and cleared by software. It is used to enable/disable the hysteresis function
(see Table 396 for details). This bit must be kept to 0 when SADMOD[1:0] = 1x.
0: Hysteresis function disabled. THRH is always used.
1: Hysteresis function enabled. THRH is used for MONITOR to DETECT transition and THRL
is used for DETECT to MONITOR transition.
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 6 Reserved, must be kept at reset value.
Bits 5:4 SADST[1:0]: SAD state
This bitfield is set and cleared by hardware. It indicates the SAD state and is meaningful only
when SADEN = 1. The SAD state can be:
- LEARN when the SAD is in learning phase or in SDLVL computation mode
- MONITOR when the SAD is in monitoring phase
- DETECT when the SAD detects a sound
00: SAD in LEARN state
01: SAD in MONITOR state
11: SAD in DETECT state
Bit 3 DETCFG: Sound trigger event configuration
This bit is set and cleared by software. It is used to define if the sddet_evt event is generated
only when the SAD enters to MONITOR state or when the SAD enters or exits the DETECT
state.
0: sddet_evt generated when SAD enters the MONITOR state
1: sddet_evt generated when SAD enters or exits the DETECT state
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bits 2:1 DATCAP[1:0]: Data capture mode
This bitfield is set and cleared by software. It is used to define in which conditions, the
samples provided by DLFT0 are stored into the memory.
00: Samples from DFLT0 not transfered into the memory
01: Samples from DFLT0 transfered into the memory when SAD is in DETECT state
1x: Samples from DFLT0 transfered into memory when SAD and DFLT0 are enabled
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 0 SADEN: Sound activity detector enable
This bit is set and cleared by software. It is used to enable/disable the SAD.
0: SAD disabled and SAD state reset
1: SAD enabled

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

40.8.12

RM0456

ADF SAD configuration register (ADF_SADCFGR)
Address offset: 0x0BC
Reset value: 0x0000 0000
This register is used for the configuration of the sound activity detection.

31

30

29

Res.

Res.

Res.

15

14

13

Res.

28

rw

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

ANMIN[12:0]
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

HGOVR[2:0]
rw

27

Res.
rw

LFRNB[2:0]
rw

rw

Res.
rw

ANSLP[2:0]
rw

rw

SNTHR[3:0]
rw

rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:16 ANMIN[12:0]: Minimum noise level
This bitfield is set and cleared by software. It is used to define the minimum noise level and
then the sensitivity. It represents a positive number.
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 HGOVR[2:0]: Hangover time window
This bitfield is set and cleared by software. Once the SAD state is DETECT, this parameter is
used to define the amount of time the sound is allowed to remain below the threshold, before
switching the SAD to MONITOR state (see FRSIZE bitfield for the description of a frame).
000: SAD back to MONITOR state if sound is below threshold for 4 frames
001: SAD back to MONITOR state if sound is below threshold for 8 frames
010: SAD back to MONITOR state if sound is below threshold for 16 frames
011: SAD back to MONITOR state if sound is below threshold for 32 frames
100: SAD back to MONITOR state if sound is below threshold for 64 frames
101: SAD back to MONITOR state if sound is below threshold for 128 frames
110: SAD back to MONITOR state if sound is below threshold for 256 frames
111: SAD back to MONITOR state if sound is below threshold for 512 frames
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 11 Reserved, must be kept at reset value.
Bits 10:8 LFRNB[2:0]: Number of learning frames
This bitfield is set and cleared by software. It is used to define the number of learning frames
to perform the first estimate of the noise level.
000: 2 frames used to compute the initial noise level
001: 4 frames used to compute the initial noise level
010: 8 frames used to compute the initial noise level
011: 16 frames used to compute the initial noise level
1xx: 32 frames used to compute the initial noise level
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 7 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Audio digital filter (ADF)

Bits 6:4 ANSLP[2:0]: Ambient noise slope control
This bitfield is set and cleared by software. It is used to define the positive and negative slope
of the noise estimator, in charge of updating the ANLVL (see Ambient noise estimation
(ANLVL) for information about programming this bitfield).
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 3:0 SNTHR[3:0]: Signal to noise threshold
This bitfield is set and cleared by software. It is used to define THRH (and THRL if hysteresis
is enabled). See Table 396 for details.
0000: THRH is 3.5 dB higher than ANLVL
0001: THRH is 6.0 dB higher than ANLVL
0010: THRH is 9.5 dB higher than ANLVL
0011: THRH is 12 dB higher than ANLVL
0100: THRH is 15.6 dB higher than ANLVL
0101: THRH is 18 dB higher than ANLVL
0110: THRH is 21.6 dB higher than ANLVL
0111: THRH is 24.1 dB higher than ANLVL
1000: THRH is 27.6 dB higher than ANLVL
1001: THRH is 30.1dB higher than ANLVL
others: Reserved
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).

40.8.13

ADF SAD sound level register (ADF_SADSDLVR)
Address offset: 0x0C0
Reset value: 0x0000 0000
This register contains the short-term sound-level computed by the SAD.

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

Res.

SDLVL[14:0]
r

r

r

r

r

r

r

r

Bits 31:15 Reserved, must be kept at reset value.
Bits 14:0 SDLVL[14:0]: Short term sound level
This bitfield is set by hardware. It contains the latest sound level computed by the SAD. To
refresh this value, SDLVLF must be cleared.

RM0456 Rev 6

<!-- pagebreak -->

