RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)
As illustrated in Figure 778, if FLTEN = 0, any mismatch between the TAMP_OUTy output
and the associated TAMP_INx input when the latter is sampled generates a tamper. This is
the case in all three examples (a), (b) and (c).
If FLTEN = 1, example (a) does not generate a tamper, since only one mismatch is detected
in four consecutive comparisons. In example (b), a tamper is generated since two
successive mismatches are detected. Example (c) also generates a tamper, since two
mismatches occur in four consecutive comparisons, even though the mismatches do not
occur on successive samples.
Setting FLTEN = 1 avoids unwanted detection of tampers due to glitches, bounce or
transitory states on the TAMP_INx inputs, by ignoring single pulses which are shorter than
one period of CK_ATPRE, programmed in the ATCKSEL field of the TAMP_ATCR1 register.
The minimum filtered pulse width is listed in Table 648 for each possible setting of
ATCKSEL, assuming fRTCCLK = 32.768 kHz.
Table 648. Active tamper filtered pulse duration
ATCKSEL[3:0]

CK_ATPRE frequency

Minimum filtered pulse width (ms)

0x0

fRTCCLK

0.030

0x1

fRTCCLK/2

0.061

0x2

fRTCCLK/4

0.122

0x3

fRTCCLK/8

0.244

0x4

fRTCCLK/16

0.488

0x5

fRTCCLK/32

0.977

0x6

fRTCCLK/64

1.953

0x7

fRTCCLK/128

3.906

(1)

fRTCCLK/2048

62.500(2)

0xB

1. These values are supported only when the active tamper prescaler extension is supported. Refer to
Section 64.3: TAMP implementation.
2. This setting requires that (PREDIV_A+1) = 128 and (PREDIV_S+1) is a multiple of 16.

Note:

Multiple pulses which are shorter than one CK_ATPRE period may nevertheless cause a
tamper if they result in two mismatches in four consecutive comparisons.

Caution:

Entering RTC initialization mode stops CK_ATPRE and CK_ATPER clocks when
ATCKSEL[3] = 1. Therefore, TAMP_OUTy pin stops toggling until INIT mode exit.
Refer to section Section : Calendar initialization and configuration.
Refer also to RTC alarm A subsecond register (RTC_ALRMASSR), RTC alarm B
subsecond register (RTC_ALRMBSSR), RTC alarm A binary mode register
(RTC_ALRABINR) and RTC alarm B binary mode register (RTC_ALRBBINR) in case
RTC binary mode is used in conjunction with ATCKSEL[3] = 1.

Caution:

Caution: The active tamper detection is no more functional in case of calendar overflow
when ATCKSEL[3] = 1. It is mandatory to enable the internal tamper 5 on calendar overflow
to ensure tamper protection.
The pseudo-random generator must be initialized with a seed. This is done by writing
consecutively four 32-bit random values in the TAMP_ATSEEDR register. Programming the
seed automatically sends it to the PRNG. As long as the new seed is transferred and

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

RM0456

elaborated by the PRNG, the SEEDF bit is set in the TAMP_ATOR and it is not allowed to
switch off the TAMP APB clock. The duration of the elaboration is up to 184 APB clock
cycles after the forth seed is written. Consequently, after writing a new seed, the user must
wait until SEEDF is cleared before entering low-power modes.
The active tamper outputs are activated only after the first seed is written and the
elaboration is completed. Then new seeds can be written and elaborated during active
tamper activity.

Active tamper initialization
Here is the software procedure to initialize the active tampers after system reset:
Read INITS in TAMP_ATOR register.
•
If INITS = 0x0 (initialization was not done):

•

a)

Write TAMP_ATCR to configure Active tamper clock, filter and output sharing if
any, and active mode.

b)

Write TAMP_CR1 to enable tampers (all the needed tampers must be enabled in
the same write access).

c)

Write SEED by writing four times in the TAMP_ATSEEDR.

d)

Wait until SEEDF = 0 in TAMP_ATOR. Backup registers are then protected by
active tamper.

If INITS = 0x1 (initialization already done):
No initialization. To increase randomness a new SEED should be provided regularly.
When a new SEED is provided, wait until SEEDF = 0 before entering a low-power
mode which switches off the TAMP APB clock.

•

<!-- pagebreak -->

In case the tampers are disabled by software, and re-enabled afterwards, the SEED
must be written after enabling tampers:
a)

Write TAMP_CR1 to enable tampers (all the needed tampers must be enabled in
the same write access).

b)

Write SEED by writing four times in the TAMP_ATSEEDR.

c)

Wait until SEEDF = 0 in TAMP_ATOR. Backup registers are then protected by
active tamper.

RM0456 Rev 6

RM0456

64.5

Tamper and backup registers (TAMP)

TAMP low-power modes
Table 649. Effect of low-power modes on TAMP
Mode

Description

Sleep

No effect.
TAMP interrupts cause the device to exit the Sleep mode.

Stop

No effect, except for level detection with filtering and active tamper modes which remain
active only when the clock source is LSE or LSI. Some internal tampers sources are not
functional, refer to Table 100: Functionalities depending on the working mode.
TAMP interrupts cause the device to exit the Stop mode.

Standby

No effect, except for level detection with filtering and active tamper modes which remain
active only when the clock source is LSE or LSI. Some internal tampers sources are not
functional, refer to Table 100: Functionalities depending on the working mode.
TAMP interrupts cause the device to exit the Standby mode.

No effect, except for level detection with filtering and active tamper modes which remain
active only when the clock source is LSE. Some internal tampers sources are not
Shutdown functional, refer to Table 100: Functionalities depending on the working mode.
TAMP interrupts cause the device to exit the Shutdown mode.

Table 650. TAMP pins functionality over modes

64.6

Pin name

Functional in all low-power modes

Functional in VBAT mode

TAMP_IN[8:1]

Yes

Yes

TAMP_OUT[8:1]

Yes

Yes

TAMP interrupts
The interrupt channel is set in the masked interrupt status register or in the secure masked
interrupt status register depending on its security mode configuration (TAMPSEC).
Table 651. Interrupt requests
Interrupt acronym

TAMP

Interrupt
event

Event
flag(1)

Enable
control bit

Interrupt
clear
method

Exit from
low-power
modes

Tamper x(2)

TAMPxF

TAMPxIE

Write 1 in
CTAMPxF

Yes(3)

Internal
tamper y(2)

ITAMPyF

ITAMPyIE

Write 1 in
CITAMPyF

Yes(3)

1. The event flags are in the TAMP_SR register.
2. The number of tampers and internal tampers events depend on products.
3. Refer to Table 649: Effect of low-power modes on TAMP for more details about available features in the
low-power modes.

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

64.7

RM0456

TAMP registers
Refer to Section 1.2 of the reference manual for a list of abbreviations used in register
descriptions. The peripheral registers can be accessed by words (32-bit).

64.7.1

TAMP control register 1 (TAMP_CR1)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x00
Backup domain reset value: 0x0000 0000
System reset: not affected

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

Res.

Res.

28

27

26

ITAMP1 ITAMP1 ITAMP1
3E
2E
1E
rw

rw

rw

12

11

10

Res.

Res.

Res.

25
Res.

9
Res.

24

23

ITA- ITAMP8 ITAMP9E
E
MP7E

21

20

ITAMP6 ITAMP5
E
E

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

Res.

19
Res.

3

18

17

16

ITAMP3 ITAMP2 ITAMP1
E
E
E
rw

rw

rw

2

1

0

TAMP8 TAMP7 TAMP6 TAMP5 TAMP4 TAMP3 TAMP2 TAMP1
E
E
E
E
E
E
E
E
rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 Reserved, must be kept at reset value.
Bit 29 Reserved, must be kept at reset value.
Bit 28 ITAMP13E: Internal tamper 13 enable
0: Internal tamper 13 disabled.
1: Internal tamper 13 enabled.
Bit 27 ITAMP12E: Internal tamper 12 enable
0: Internal tamper 12 disabled.
1: Internal tamper 12 enabled.
Bit 26 ITAMP11E: Internal tamper 11 enable
0: Internal tamper 11 disabled.
1: Internal tamper 11 enabled.
Bit 25 Reserved, must be kept at reset value.
Bit 24 ITAMP9E: Internal tamper 9 enable
0: Internal tamper 9 disabled.
1: Internal tamper 9 enabled.
Bit 23 ITAMP8E: Internal tamper 8 enable
0: Internal tamper 8 disabled.
1: Internal tamper 8 enabled.
Bit 22 ITAMP7E: Internal tamper 7 enable
0: Internal tamper 7 disabled.
1: Internal tamper 7 enabled

<!-- pagebreak -->

22

RM0456 Rev 6

rw

rw

rw

rw

rw

rw

rw

RM0456

Tamper and backup registers (TAMP)

Bit 21 ITAMP6E: Internal tamper 6 enable
0: Internal tamper 6 disabled.
1: Internal tamper 6 enabled.
Bit 20 ITAMP5E: Internal tamper 5 enable
0: Internal tamper 5 disabled.
1: Internal tamper 5 enabled.
Bit 19 Reserved, must be kept at reset value.
Bit 18 ITAMP3E: Internal tamper 3 enable
0: Internal tamper 3 disabled.
1: Internal tamper 3 enabled.
Bit 17 ITAMP2E: Internal tamper 2 enable
0: Internal tamper 2 disabled.
1: Internal tamper 2 enabled.
Bit 16 ITAMP1E: Internal tamper 1 enable
0: Internal tamper 1 disabled.
1: Internal tamper 1 enabled.
Bits 15:8 Reserved, must be kept at reset value.
Bit 7 TAMP8E: Tamper detection on TAMP_IN8 enable(1)
0: Tamper detection on TAMP_IN8 is disabled.
1: Tamper detection on TAMP_IN8 is enabled.
Bit 6 TAMP7E: Tamper detection on TAMP_IN7 enable(1)
0: Tamper detection on TAMP_IN7 is disabled.
1: Tamper detection on TAMP_IN7 is enabled.
Bit 5 TAMP6E: Tamper detection on TAMP_IN6 enable(1)
0: Tamper detection on TAMP_IN6 is disabled.
1: Tamper detection on TAMP_IN6 is enabled.
Bit 4 TAMP5E: Tamper detection on TAMP_IN5 enable(1)
0: Tamper detection on TAMP_IN5 is disabled.
1: Tamper detection on TAMP_IN5 is enabled.
Bit 3 TAMP4E: Tamper detection on TAMP_IN4 enable(1)
0: Tamper detection on TAMP_IN4 is disabled.
1: Tamper detection on TAMP_IN4 is enabled.
Bit 2 TAMP3E: Tamper detection on TAMP_IN3 enable(1)
0: Tamper detection on TAMP_IN3 is disabled.
1: Tamper detection on TAMP_IN3 is enabled.
Bit 1 TAMP2E: Tamper detection on TAMP_IN2 enable(1)
0: Tamper detection on TAMP_IN2 is disabled.
1: Tamper detection on TAMP_IN2 is enabled.
Bit 0 TAMP1E: Tamper detection on TAMP_IN1 enable(1)
0: Tamper detection on TAMP_IN1 is disabled.
1: Tamper detection on TAMP_IN1 is enabled.
1. Tamper detection mode (selected with TAMP_FLTCR, TAMP_ATCR1, TAMP_ATCR2 registers and TAMPxTRG bits in
TAMP_CR2), must be configured before enabling the tamper detection.

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

64.7.2

RM0456

TAMP control register 2 (TAMP_CR2)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x04
Backup domain reset value: 0x0000 0000
System reset: not affected

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

TAMP8 TAMP7 TAMP6 TAMP5 TAMP4 TAMP3 TAMP2 TAMP1
BK
BK
TRG
TRG
TRG
TRG
TRG
TRG
TRG
TRG ERASE BLOCK
rw

rw

rw

rw

rw

rw

rw

rw

w

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

21

20

19

Res.

Res.

Res.

5

4

3

18

17

rw

rw

rw

2

1

0

TAMP8 TAMP7 TAMP6 TAMP5 TAMP4 TAMP3 TAMP2 TAMP1
NOER NOER NOER NOER NOER NOER NOER NOER
rw

rw

rw

rw

rw

rw

rw

Bit 31 TAMP8TRG: Active level for tamper 8 input (active mode disabled)
0: If TAMPFLT ≠ 00 tamper 8 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 8 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 8 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 8 input falling edge triggers a tamper detection event.
Bit 30 TAMP7TRG: Active level for tamper 7 input (active mode disabled)
0: If TAMPFLT ≠ 00 tamper 7 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 7 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 7 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 7 input falling edge triggers a tamper detection event.
Bit 29 TAMP6TRG: Active level for tamper 6 input (active mode disabled)
0: If TAMPFLT ≠ 00 tamper 6 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 6 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 6 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 6 input falling edge triggers a tamper detection event.
Bit 28 TAMP5TRG: Active level for tamper 5 input (active mode disabled)
0: If TAMPFLT ≠ 00 tamper 5 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 5 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 5 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 5 input falling edge triggers a tamper detection event.
Bit 27 TAMP4TRG: Active level for tamper 4 input (active mode disabled)
0: If TAMPFLT ≠ 00 tamper 4 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 4 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 4 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 4 input falling edge triggers a tamper detection event.

<!-- pagebreak -->

16

TAMP3 TAMP2 TAMP1
MSK
MSK
MSK

RM0456 Rev 6

rw

RM0456

Tamper and backup registers (TAMP)

Bit 26 TAMP3TRG: Active level for tamper 3 input
0: If TAMPFLT ≠ 00 tamper 3 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 3 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 3 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 3 input falling edge triggers a tamper detection event.
Bit 25 TAMP2TRG: Active level for tamper 2 input
0: If TAMPFLT ≠ 00 tamper 2 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 2 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 2 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 2 input falling edge triggers a tamper detection event.
Bit 24 TAMP1TRG: Active level for tamper 1 input
0: If TAMPFLT ≠ 00 tamper 1 input staying low triggers a tamper detection event.
If TAMPFLT = 00 tamper 1 input rising edge triggers a tamper detection event.
1: If TAMPFLT ≠ 00 tamper 1 input staying high triggers a tamper detection event.
If TAMPFLT = 00 tamper 1 input falling edge triggers a tamper detection event.
Bit 23 BKERASE: Backup registers and device secrets(1) erase
Writing ‘1’ to this bit reset the backup registers and device secrets(1). Writing 0 has no effect.
This bit is always read as 0.
Bit 22 BKBLOCK: Backup registers and device secrets(1) access blocked
0: backup registers and device secrets(1) can be accessed if no tamper flag is set
1: backup registers and device secrets(1) cannot be accessed
Bits 21:19 Reserved, must be kept at reset value.
Bit 18 TAMP3MSK: Tamper 3 mask
0: Tamper 3 event generates a trigger event and TAMP3F must be cleared by software to
allow next tamper event detection.
1: Tamper 3 event generates a trigger event. TAMP3F is masked and internally cleared by
hardware. The backup registers and device secrets(1) are not erased.
The tamper 3 interrupt must not be enabled when TAMP3MSK is set.
Bit 17 TAMP2MSK: Tamper 2 mask
0: Tamper 2 event generates a trigger event and TAMP2F must be cleared by software to
allow next tamper event detection.
1: Tamper 2 event generates a trigger event. TAMP2F is masked and internally cleared by
hardware. The backup registers and device secrets(1) are not erased.
The tamper 2 interrupt must not be enabled when TAMP2MSK is set.
Bit 16 TAMP1MSK: Tamper 1 mask
0: Tamper 1 event generates a trigger event and TAMP1F must be cleared by software to
allow next tamper event detection.
1: Tamper 1 event generates a trigger event. TAMP1F is masked and internally cleared by
hardware. The backup registers and device secrets(1) are not erased.
The tamper 1 interrupt must not be enabled when TAMP1MSK is set.
Bits 15:8 Reserved, must be kept at reset value.
Bit 7 TAMP8NOER: Tamper 8 no erase
0: Tamper 8 event detection is in confirmed mode(1).
1: Tamper 8 event detection is in potential mode(2).
Bit 6 TAMP7NOER: Tamper 7 no erase
0: Tamper 7 event detection is in confirmed mode(1).
1: Tamper 7 event detection is in potential mode(2).

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

RM0456

Bit 5 TAMP6NOER: Tamper 6 no erase
0: Tamper 6 event detection is in confirmed mode(1).
1: Tamper 6 event detection is in potential mode(2).
Bit 4 TAMP5NOER: Tamper 5 no erase
0: Tamper 5 event detection is in confirmed mode(1).
1: Tamper 5 event detection is in potential mode(2).
Bit 3 TAMP4NOER: Tamper 4 no erase
0: Tamper 4 event detection is in confirmed mode(1).
1: Tamper 4 event detection is in potential mode(2).
Bit 2 TAMP3NOER: Tamper 3 no erase
0: Tamper 3 event detection is in confirmed mode(1).
1: Tamper 3 event detection is in potential mode(2).
Bit 1 TAMP2NOER: Tamper 2 no erase
0: Tamper 2 event detection is in confirmed mode(1).
1: Tamper 2 event detection is in potential mode(2).
Bit 0 TAMP1NOER: Tamper 1 no erase
0: Tamper 1 event detection is in confirmed mode(1).
1: Tamper 1 event detection is in potential mode(2).
1. The effects of tamper detection in confirmed mode is described with tamp_confirmed and tamp_confirmed_ercfgx signals in
Table 644: TAMP interconnection.
2. The effects of tamper detection in potential mode is described with tamp_potential and tamp_potential_ercfgx signals in
Table 644: TAMP interconnection.

64.7.3

TAMP control register 3 (TAMP_CR3)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x08
Backup domain reset value: 0x0000 0000
System reset: not affected

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

ITAMP ITAMP ITAMP
13NOE 12NOE 11NOE
R
R
R
rw

rw

rw

Res.

ITAMP9 ITAMP8 ITAMP7 ITAMP6 ITAMP5
NOER NOER NOER NOER NOER
rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 Reserved, must be kept at reset value.
Bit 14 Reserved, must be kept at reset value.
Bit 13 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

rw

Res.

ITAMP3 ITAMP2 ITAMP1
NOER NOER NOER
rw

rw

rw

RM0456

Tamper and backup registers (TAMP)

Bit 12 ITAMP13NOER: Internal tamper 13 no erase
0: Internal tamper 13 event detection is in confirmed mode(1).
1: Internal tamper 13 event detection is in potential mode(2).
Bit 11 ITAMP12NOER: Internal tamper 12 no erase
0: Internal tamper 12 event detection is in confirmed mode(1).
1: Internal tamper 12 event detection is in potential mode(2).
Bit 10 ITAMP11NOER: Internal tamper 11 no erase
0: Internal tamper 11 event detection is in confirmed mode(1).
1: Internal tamper 11 event detection is in potential mode(2).
Bit 9 Reserved, must be kept at reset value.
Bit 8 ITAMP9NOER: Internal tamper 9 no erase
0: Internal tamper 9 event detection is in confirmed mode(1).
1: Internal tamper 9 event detection is in potential mode(2).
Bit 7 ITAMP8NOER: Internal tamper 8 no erase
0: Internal tamper 8 event detection is in confirmed mode(1).
1: Internal tamper 8 event detection is in potential mode(2).
Bit 6 ITAMP7NOER: Internal tamper 7 no erase
0: Internal tamper 7 event detection is in confirmed mode(1).
1: Internal tamper 7 event detection is in potential mode(2).
Bit 5 ITAMP6NOER: Internal tamper 6 no erase
0: Internal tamper 6 event detection is in confirmed mode(1).
1: Internal tamper 6 event detection is in potential mode(2).
Bit 4 ITAMP5NOER: Internal tamper 5 no erase
0: Internal tamper 5 event detection is in confirmed mode(1).
1: Internal tamper 5 event detection is in potential mode(2).
Bit 3 Reserved, must be kept at reset value.
Bit 2 ITAMP3NOER: Internal tamper 3 no erase
0: Internal tamper 3 event detection is in confirmed mode(1).
1: Internal tamper 3 event detection is in potential mode(2).
Bit 1 ITAMP2NOER: Internal tamper 2 no erase
0: Internal tamper 2 event detection is in confirmed mode(1).
1: Internal tamper 2 event detection is in potential mode(2).
Bit 0 ITAMP1NOER: Internal tamper 1 no erase
0: Internal tamper 1 event detection is in confirmed mode(1).
1: Internal tamper 1 event detection is in potential mode(2).
1. The effects of internal tamper detection in confirmed mode is described with tamp_confirmed and tamp_confirmed_ercfgx
signals in Table 644: TAMP interconnection
2. The effects of internal tamper detection in potential mode is described with tamp_potential and tamp_potential_ercfgx
signals in Table 644: TAMP interconnection.

64.7.4

TAMP filter control register (TAMP_FLTCR)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

RM0456

privilege protection modes.
Address offset: 0x0C
Backup domain reset value: 0x0000 0000
System reset: not affected
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

TAMP
PUDIS

Res.

Res.

Res.

Res.

Res.

Res.

Res.

rw

TAMPPRCH
[1:0]

TAMPFLT
[1:0]

rw

rw

rw

rw

TAMPFREQ
[2:0]
rw

rw

rw

Bits 31:8 Reserved, must be kept at reset value.
Bit 7 TAMPPUDIS: TAMP_INx pull-up disable
This bit determines if each of the TAMPx pins are precharged before each sample.
0: Precharge TAMP_INx pins before sampling (enable internal pull-up)
1: Disable precharge of TAMP_INx pins.
Bits 6:5 TAMPPRCH[1:0]: TAMP_INx precharge duration
These bit determines the duration of time during which the pull-up/is activated before each
sample. TAMPPRCH is valid for each of the TAMP_INx inputs.
0x0: 1 RTCCLK cycle
0x1: 2 RTCCLK cycles
0x2: 4 RTCCLK cycles
0x3: 8 RTCCLK cycles
Bits 4:3 TAMPFLT[1:0]: TAMP_INx filter count
These bits determines the number of consecutive samples at the specified level
(TAMP*TRG) needed to activate a tamper event. TAMPFLT is valid for each of the
TAMP_INx inputs.
0x0: Tamper event is activated on edge of TAMP_INx input transitions to the active level (no
internal pull-up on TAMP_INx input).
0x1: Tamper event is activated after 2 consecutive samples at the active level.
0x2: Tamper event is activated after 4 consecutive samples at the active level.
0x3: Tamper event is activated after 8 consecutive samples at the active level.
Bits 2:0 TAMPFREQ[2:0]: Tamper sampling frequency
Determines the frequency at which each of the TAMP_INx inputs are sampled.
0x0: RTCCLK / 32768 (1 Hz when RTCCLK = 32768 Hz)
0x1: RTCCLK / 16384 (2 Hz when RTCCLK = 32768 Hz)
0x2: RTCCLK / 8192 (4 Hz when RTCCLK = 32768 Hz)
0x3: RTCCLK / 4096 (8 Hz when RTCCLK = 32768 Hz)
0x4: RTCCLK / 2048 (16 Hz when RTCCLK = 32768 Hz)
0x5: RTCCLK / 1024 (32 Hz when RTCCLK = 32768 Hz)
0x6: RTCCLK / 512 (64 Hz when RTCCLK = 32768 Hz)
0x7: RTCCLK / 256 (128 Hz when RTCCLK = 32768 Hz)

Note:

<!-- pagebreak -->

This register concerns only the tamper inputs in passive mode.

RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)

64.7.5

TAMP active tamper control register 1 (TAMP_ATCR1)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.
Address offset: 0x10
Backup domain reset value: 0x0007 0000
System reset: not affected

31

30

ATO
FLTEN
SHARE
rw

rw

15

14

29

28

27

Res.

Res.

Res.

13

12

11

26

25

24

ATPER[2:0]
rw

rw

rw

10

9

8

ATOSEL4[1:0]

ATOSEL3[1:0]

ATOSEL2[1:0]

ATOSEL1[1:0]

rw

rw

rw

rw

rw

rw

rw

23

22

21

20

Res.

Res.

Res.

Res.

7

6

5

4

19

18

17

16

ATCKSEL[3:0]
rw

rw

rw

rw

3

2

1

0

TAMP8 TAMP7 TAMP6 TAMP5 TAMP4 TAMP3 TAMP2 TAMP1
AM
AM
AM
AM
AM
AM
AM
AM

rw

rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 FLTEN: Active tamper filter enable
0: Active tamper filtering disable
1: Active tamper filtering enable: a tamper event is detected when 2 comparison mismatches
occur out of 4 consecutive samples.
Bit 30 ATOSHARE: Active tamper output sharing
0: Each active tamper input TAMP_INi is compared with its dedicated output TAMP_OUTi
1: Each active tamper input TAMP_INi is compared with TAMPOUTSELi defined by ATOSELi
bits.
Bits 29:27 Reserved, must be kept at reset value.
Bits 26:24 ATPER[2:0]: Active tamper output change period
The tamper output is changed every CK_ATPER = (2ATPER x CK_ATPRE) cycles. Refer to
Table 647: Minimum ATPER value.
Bits 23:20 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

2687

Tamper and backup registers (TAMP)

RM0456

Bits 19:16 ATCKSEL[3:0]: Active tamper RTC asynchronous prescaler clock selection
These bits selects the RTC asynchronous prescaler stage output. The selected clock is
CK_ATPRE. ATCKSEL[3] is reserved, read only, and tied to 0 when the active tamper
prescaler extension is not implemented.
0000: RTCCLK is selected
0001: RTCCLK/2 is selected
0010: RTCCLK/4 is selected
0011: RTCCLK/8 is selected
0100: RTCCLK/16 is selected
0101: RTCCLK/32 is selected
0110: RTCCLK/64 is selected
0111: RTCCLK/128 is selected
1011: RTCCLK/2048 is selected when (PREDIV_A+1) = 128 and (PREDIV_S+1) is a
multiple of 16. This value is supported only when the active tamper prescaler
extension is supported. Refer to Section 64.3: TAMP implementation.
Others: Reserved
Note: These bits can be written only when all active tampers are disabled. The write
protection remains for up to 1.5 CK_ATPRE cycles after all the active tampers are
disable.
Bits 15:14 ATOSEL4[1:0]: Active tamper shared output 4 selection
00: TAMPOUTSEL4 = TAMP_OUT1
01: TAMPOUTSEL4 = TAMP_OUT2
10: TAMPOUTSEL4 = TAMP_OUT3
11: TAMPOUTSEL4 = TAMP_OUT4
If the TAMP_OUTx output is not available in the package pinout, the ouput selection value is
reserved and must not be used.
Bits 13:12 ATOSEL3[1:0]: Active tamper shared output 3 selection
00: TAMPOUTSEL3 = TAMP_OUT1
01: TAMPOUTSEL3 = TAMP_OUT2
10: TAMPOUTSEL3 = TAMP_OUT3
11: TAMPOUTSEL3 = TAMP_OUT4
If the TAMP_OUTx output is not available in the package pinout, the ouput selection value is
reserved and must not be used.
Bits 11:10 ATOSEL2[1:0]: Active tamper shared output 2 selection
00: TAMPOUTSEL2 = TAMP_OUT1
01: TAMPOUTSEL2 = TAMP_OUT2
10: TAMPOUTSEL2 = TAMP_OUT3
11: TAMPOUTSEL2 = TAMP_OUT4
If the TAMP_OUTx output is not available in the package pinout, the ouput selection value is
reserved and must not be used.
Bits 9:8 ATOSEL1[1:0]: Active tamper shared output 1 selection
00: TAMPOUTSEL1 = TAMP_OUT1
01: TAMPOUTSEL1 = TAMP_OUT2
10: TAMPOUTSEL1 = TAMP_OUT3
11: TAMPOUTSEL1 = TAMP_OUT4
If the TAMP_OUTx output is not available in the package pinout, the ouput selection value is
reserved and must not be used.
Bit 7 TAMP8AM: Tamper 8 active mode
0: Tamper 8 detection mode is passive.
1: Tamper 8 detection mode is active.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Tamper and backup registers (TAMP)

Bit 6 TAMP7AM: Tamper 7 active mode
0: Tamper 7 detection mode is passive.
1: Tamper 7 detection mode is active.
Bit 5 TAMP6AM: Tamper 6 active mode
0: Tamper 6 detection mode is passive.
1: Tamper 6 detection mode is active.
Bit 4 TAMP5AM: Tamper 5 active mode
0: Tamper 5 detection mode is passive.
1: Tamper 5 detection mode is active.
Bit 3 TAMP4AM: Tamper 4 active mode
0: Tamper 4 detection mode is passive.
1: Tamper 4 detection mode is active.
Bit 2 TAMP3AM: Tamper 3 active mode
0: Tamper 3 detection mode is passive.
1: Tamper 3 detection mode is active.
Bit 1 TAMP2AM: Tamper 2 active mode
0: Tamper 2 detection mode is passive.
1: Tamper 2 detection mode is active.
Bit 0 TAMP1AM: Tamper 1 active mode
0: Tamper 1 detection mode is passive.
1: Tamper 1 detection mode is active.

Note:

Changing the active tampers configuration in this register is not allowed when a TAMPxAM
bit is set, unless the corresponding TAMPxE bits are all cleared in the TAMP_CR1 register.
All tampers configured in active mode must be enabled at the same time (by setting all
related TAMPxE in the same TAMP_CR1 write).
All tampers configured in active mode must be disabled at the same time (by clearing all
related TAMPxE in the same TAMP_CR1 write).
A minimum duration of 1 CK_ATPRE period must be waited for after disabling the active
tampers and before re-enabling them.

64.7.6

TAMP active tamper seed register (TAMP_ATSEEDR)
This register can be protected against nonsecure access. Refer to Section 64.4.5: TAMP
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 64.4.7: TAMP
privilege protection modes.

RM0456 Rev 6

<!-- pagebreak -->

