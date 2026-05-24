2687

Real-time clock (RTC)

63.6.5

RM0456

RTC prescaler register (RTC_PRER)
This register must be written in initialization mode only. The initialization must be performed
in two separate write accesses. Refer to Calendar initialization and configuration on
page 2599.
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be write-protected against nonsecure access. Refer to Section 63.3.4:
RTC secure protection modes.
This register can be write-protected against unprivileged access. Refer to Section 63.3.5:
RTC privilege protection modes.
Address offset: 0x10
Backup domain reset value: 0x007F 00FF
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

Res.

22

21

20

19

18

17

16

PREDIV_A[6:0]
rw

rw

rw

rw

rw

rw

rw

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

PREDIV_S[14:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:23 Reserved, must be kept at reset value.
Bits 22:16 PREDIV_A[6:0]: Asynchronous prescaler factor
This is the asynchronous division factor:
ck_apre frequency = RTCCLK frequency/(PREDIV_A+1)
Bit 15 Reserved, must be kept at reset value.
Bits 14:0 PREDIV_S[14:0]: Synchronous prescaler factor
This is the synchronous division factor:
ck_spre frequency = ck_apre frequency/(PREDIV_S+1)

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6.6

RTC wake-up timer register (RTC_WUTR)
This register can be written only when WUTWF is set to 1 in RTC_ICSR.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x14
Backup domain reset value: 0x0000 FFFF
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

WUTOCLR[15:0]
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

rw

rw

rw

rw

rw

rw

rw

rw

WUT[15:0]
rw

Bits 31:16 WUTOCLR[15:0]: Wake-up auto-reload output clear value
When WUTOCLR[15:0] is different from 0x0000, WUTF is set by hardware when the autoreload down-counter reaches 0 and is cleared by hardware when the auto-reload
downcounter reaches WUTOCLR[15:0].
When WUTOCLR[15:0] = 0x0000, WUTF is set by hardware when the WUT down-counter
reaches 0 and is cleared by software.
Bits 15:0 WUT[15:0]: Wake-up auto-reload value bits
When the wake-up timer is enabled (WUTE set to 1), the WUTF flag is set every
(WUT[15:0] + 1) ck_wut cycles. The ck_wut period is selected through WUCKSEL[2:0] bits
of the RTC_CR register.
When WUCKSEL[2] = 1, the wake-up timer becomes 17-bits and WUCKSEL[1] effectively
becomes WUT[16] the most-significant bit to be reloaded into the timer.
The first assertion of WUTF occurs between WUT and (WUT + 2) ck_wut cycles after WUTE
is set. Setting WUT[15:0] to 0x0000 with WUCKSEL[2:0] = 011 (RTCCLK/2) is forbidden.

63.6.7

RTC control register (RTC_CR)
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

This register can be globally protected, or each bit of this register can be individually
protected against nonsecure access. Refer to Section 63.3.4: RTC secure protection
modes.
This register can be globally protected, or each bit of this register can be individually
protected against unprivileged access. Refer to Section 63.3.5: RTC privilege protection
modes.
Address offset: 0x18
Backup domain reset value: 0x0000 0000
System reset: not affected
31
OUT2
EN

30

29

28

27

TAMP TAMP
ALRBF ALRAF
ALRM_ ALRM_
CLR
CLR
TYPE
PU

rw

rw

15

14

TSIE
rw

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

TAMP
OE

TAMP
TS

ITSE

COE

OSEL[1:0]

POL

COSEL

BKP

rw

rw

rw

rw

rw

w

w

7

2

1

0

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

WUTIE

ALRB
IE

ALRA
IE

TSE

WUTE

rw

rw

rw

rw

rw

ALRBE ALRAE
rw

rw

rw

6

5

4

3

SSR
UIE

FMT

BYP
SHAD

REFCK
ON

TS
EDGE

rw

rw

rw

rw

rw

17

16

SUB1H ADD1H

WUCKSEL[2:0]
rw

rw

rw

Bit 31 OUT2EN: RTC_OUT2 output enable
With this bit set, the RTC outputs can be remapped on RTC_OUT2 as follows:
OUT2EN = 0: RTC output 2 disable
If OSEL ≠ 00 or TAMPOE = 1: TAMPALRM is output on RTC_OUT1
If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT1
OUT2EN = 1: RTC output 2 enable
If (OSEL ≠ 00 or TAMPOE = 1) and COE = 0: TAMPALRM is output on RTC_OUT2
If OSEL = 00 and TAMPOE = 0 and COE = 1: CALIB is output on RTC_OUT2
If (OSEL ≠ 00 or TAMPOE = 1) and COE = 1: CALIB is output on RTC_OUT2 and
TAMPALRM is output on RTC_OUT1.
Bit 30 TAMPALRM_TYPE: TAMPALRM output type
0: TAMPALRM is push-pull output
1: TAMPALRM is open-drain output
Bit 29 TAMPALRM_PU: TAMPALRM pull-up enable
0: No pull-up is applied on TAMPALRM output
1: A pull-up is applied on TAMPALRM output
Bit 28 ALRBFCLR: Alarm B flag automatic clear
0: Alarm B event generates a trigger event and ALRBF must be cleared by software to allow
next alarm event.
1: Alarm B event generates a trigger event. ALRBF is automatically cleared by hardware
after 1 ck_apre cycle.
Bit 27 ALRAFCLR: Alarm A flag automatic clear
0: Alarm A event generates a trigger event and ALRAF must be cleared by software to allow
next alarm event.
1: Alarm A event generates a trigger event. ALRAF is automatically cleared by hardware
after 1 ck_apre cycle.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

Bit 26 TAMPOE: Tamper detection output enable on TAMPALRM
0: The tamper flag is not routed on TAMPALRM
1: The tamper flag is routed on TAMPALRM, combined with the signal provided by OSEL and
with the polarity provided by POL.
Bit 25 TAMPTS: Activate timestamp on tamper detection event
0: Tamper detection event does not cause a RTC timestamp to be saved
1: Save RTC timestamp on tamper detection event
TAMPTS is valid even if TSE = 0 in the RTC_CR register. Timestamp flag is set up to 3
ck_apre cycles after the tamper flags.
Note: TAMPTS must be cleared before entering RTC initialization mode.
Bit 24 ITSE: timestamp on internal event enable
0: internal event timestamp disabled
1: internal event timestamp enabled
Bit 23 COE: Calibration output enable
This bit enables the CALIB output
0: Calibration output disabled
1: Calibration output enabled
Bits 22:21 OSEL[1:0]: Output selection
These bits are used to select the flag to be routed to TAMPALRM output.
00: Output disabled
01: Alarm A output enabled
10: Alarm B output enabled
11: Wake-up output enabled
Bit 20 POL: Output polarity
This bit is used to configure the polarity of TAMPALRM output.
0: The pin is high when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0]), or
when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
1: The pin is low when ALRAF/ALRBF/WUTF is asserted (depending on OSEL[1:0]), or
when a TAMPxF/ITAMPxF is asserted (if TAMPOE = 1).
Bit 19 COSEL: Calibration output selection
When COE = 1, this bit selects which signal is output on CALIB.
0: Calibration output is 512 Hz
1: Calibration output is 1 Hz
These frequencies are valid for RTCCLK at 32.768 kHz and prescalers at their default values
(PREDIV_A = 127 and PREDIV_S = 255). Refer to Section 63.3.18: Calibration clock output.
Bit 18 BKP: Backup
This bit can be written by the user to memorize whether the daylight saving time change has
been performed or not.
Bit 17 SUB1H: Subtract 1 hour (winter time change)
When this bit is set outside initialization mode, 1 hour is subtracted to the calendar time if the
current hour is not 0. This bit is always read as 0.
Setting this bit has no effect when current hour is 0.
0: No effect
1: Subtracts 1 hour to the current time. This can be used for winter time change.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bit 16 ADD1H: Add 1 hour (summer time change)
When this bit is set outside initialization mode, 1 hour is added to the calendar time. This bit
is always read as 0.
0: No effect
1: Adds 1 hour to the current time. This can be used for summer time change
Bit 15 TSIE: Timestamp interrupt enable
0: Timestamp interrupt disable
1: Timestamp interrupt enable
Bit 14 WUTIE: Wake-up timer interrupt enable
0: Wake-up timer interrupt disabled
1: Wake-up timer interrupt enabled
Bit 13 ALRBIE: Alarm B interrupt enable
0: Alarm B interrupt disable
1: Alarm B interrupt enable
Bit 12 ALRAIE: Alarm A interrupt enable
0: Alarm A interrupt disabled
1: Alarm A interrupt enabled
Bit 11 TSE: timestamp enable
0: timestamp disable
1: timestamp enable
Bit 10 WUTE: Wake-up timer enable
0: Wake-up timer disabled
1: Wake-up timer enabled
Note: When the wake-up timer is disabled, wait for WUTWF = 1 before enabling it again.
Bit 9 ALRBE: Alarm B enable
0: Alarm B disabled
1: Alarm B enabled
Bit 8 ALRAE: Alarm A enable
0: Alarm A disabled
1: Alarm A enabled
Bit 7 SSRUIE: SSR underflow interrupt enable
0: SSR underflow interrupt disabled
1: SSR underflow interrupt enabled
Bit 6 FMT: Hour format
0: 24 hour/day format
1: AM/PM hour format
Bit 5 BYPSHAD: Bypass the shadow registers
0: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken from
the shadow registers, which are updated once every two RTCCLK cycles.
1: Calendar values (when reading from RTC_SSR, RTC_TR, and RTC_DR) are taken
directly from the calendar counters.
Note: If the frequency of the APB clock is less than seven times the frequency of RTCCLK,
BYPSHAD must be set to 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

Bit 4 REFCKON: RTC_REFIN reference clock detection enable (50 or 60 Hz)
0: RTC_REFIN detection disabled
1: RTC_REFIN detection enabled
Note: BIN must be 0x00 and PREDIV_S must be 0x00FF.
Bit 3 TSEDGE: Timestamp event active edge
0: RTC_TS input rising edge generates a timestamp event
1: RTC_TS input falling edge generates a timestamp event
TSE must be reset when TSEDGE is changed to avoid unwanted TSF setting.
Bits 2:0 WUCKSEL[2:0]: ck_wut wake-up clock selection
000: RTC/16 clock is selected
001: RTC/8 clock is selected
010: RTC/4 clock is selected
011: RTC/2 clock is selected
10x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is
the clock selected by BCDU.
11x: ck_spre (usually 1 Hz) clock is selected in BCD mode. In binary or mixed mode, this is
the clock selected by BCDU. Furthermore, 216 is added to the WUT counter value.

Note:

Bits 6 and 4 of this register can be written in initialization mode only (RTC_ICSR/INITF = 1).
WUT = wake-up unit counter value. WUT = (0x0000 to 0xFFFF) + 0x10000 added when
WUCKSEL[2:1 = 11].
Bits 2 to 0 of this register can be written only when RTC_CR WUTE bit = 0 and RTC_ICSR
WUTWF bit = 1.
It is recommended not to change the hour during the calendar hour increment as it may
mask the incrementation of the calendar hour.
ADD1H and SUB1H changes are effective in the next second.

63.6.8

RTC privilege mode control register (RTC_PRIVCFGR)
This register can be written only when the APB access is privileged. This register can be
write-protected, or each bit of this register can be individually write-protected against
nonsecure access depending on the RTC_SECCFGR configuration (refer to Section 63.3.5:
RTC privilege protection modes).
Address offset: 0x1C
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

TS
PRIV

WUT
PRIV

ALRB
PRIV

ALRA
PRIV

rw

rw

rw

rw

15

14

13

PRIV

INIT
PRIV

CAL
PRIV

rw

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

RM0456 Rev 6

Res.

Res.

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 PRIV: RTC privilege protection
0: All RTC registers can be written when the APB access is privileged or unprivileged, except
the registers protected by other privilege protection bits.
1: All RTC registers can be written only when the APB access is privileged.
Bit 14 INITPRIV: Initialization privilege protection
0: RTC Initialization mode, calendar and prescalers registers can be written when the APB
access is privileged or unprivileged.
1: RTC Initialization mode, calendar and prescalers registers can be written only when the
APB access is privileged.
Bit 13 CALPRIV: Shift register, Delight saving, calibration and reference clock privilege protection
0: Shift register, Delight saving, calibration and reference clock can be written when the APB
access is privileged or unprivileged.
1: Shift register, Delight saving, calibration and reference clock can be written only when the
APB access is privileged.
Bits 12:4 Reserved, must be kept at reset value.
Bit 3 TSPRIV: Timestamp privilege protection
0: RTC Timestamp configuration and interrupt clear can be written when the APB access is
privileged or unprivileged.
1: RTC Timestamp configuration and interrupt clear can be written only when the APB
access is privileged.
Bit 2 WUTPRIV: Wake-up timer privilege protection
0: RTC wake-up timer configuration and interrupt clear can be written when the APB access
is privileged or unprivileged.
1: RTC wake-up timer configuration and interrupt clear can be written only when the APB
access is privileged.
Bit 1 ALRBPRIV: Alarm B privilege protection
0: RTC Alarm B configuration and interrupt clear can be written when the APB access is
privileged or unprivileged.
1: RTC Alarm B configuration and interrupt clear can be written only when the APB access is
privileged.
Bit 0 ALRAPRIV: Alarm A and SSR underflow privilege protection
0: RTC Alarm A and SSR underflow configuration and interrupt clear can be written when the
APB access is privileged or unprivileged.
1: RTC Alarm A and SSR underflow configuration and interrupt clear can be written only
when the APB access is privileged.

Note:

<!-- pagebreak -->

Refer to Section 63.3.5: RTC privilege protection modes for details on the read protection.

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6.9

RTC secure configuration register (RTC_SECCFGR)
This register can be written only when the APB access is secure.
This register can be globally write-protected, or each bit of this register can be individually
write-protected against unprivileged access depending on the RTC_PRIVCFGR
configuration (refer to Section 63.3.5: RTC privilege protection modes).
Address offset: 0x20
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

TS
SEC

WUT
SEC

ALRB
SEC

ALRA
SEC

rw

rw

rw

rw

15

14

13

SEC

INIT
SEC

CAL
SEC

rw

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

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 SEC: RTC global protection
0: All RTC registers can be written when the APB access is secure or nonsecure, except the
registers protected by other secure protection bits.
1: All RTC registers can be written only when the APB access is secure.
Bit 14 INITSEC: Initialization protection
0: RTC Initialization mode, calendar and prescalers registers can be written when the APB
access is secure or nonsecure.
1: RTC Initialization mode, calendar and prescalers registers can be written only when the
APB access is secure.
Bit 13 CALSEC: Shift register, daylight saving, calibration and reference clock protection
0: Shift register, daylight saving, calibration and reference clock can be written when the APB
access is secure or nonsecure.
1: Shift register, daylight saving, calibration and reference clock can be written only when the
APB access is secure.
Bits 12:4 Reserved, must be kept at reset value.
Bit 3 TSSEC: Timestamp protection
0: RTC timestamp configuration and interrupt clear can be written when the APB access is
secure or nonsecure.
1: RTC timestamp configuration and interrupt clear can be written only when the APB access
is secure.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bit 2 WUTSEC: Wake-up timer protection
0: RTC wake-up timer configuration and interrupt clear can be written when the APB access
is secure or nonsecure.
1: RTC wake-up timer configuration and interrupt clear can be written only when the APB
access is secure.
Bit 1 ALRBSEC: Alarm B protection
0: RTC alarm B configuration and interrupt clear can be written when the APB access is
secure or nonsecure.
1: RTC alarm B configuration and interrupt clear can be written only when the APB access is
secure.
Bit 0 ALRASEC: Alarm A and SSR underflow protection
0: RTC alarm A and SSR underflow configuration and interrupt clear can be written when the
APB access is secure or nonsecure.
1: RTC alarm A and SSR underflow configuration and interrupt clear can be written only
when the APB access is secure.

Note:

Refer to Section 63.3.4: RTC secure protection modes for details on the read protection.

63.6.10

RTC write protection register (RTC_WPR)
Address offset: 0x24
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
w

w

w

w

w

w

w

KEY[7:0]
w

Bits 31:8 Reserved, must be kept at reset value.
Bits 7:0 KEY[7:0]: Write protection key
This byte is written by software.
Reading this byte always returns 0x00.
Refer to RTC register write protection for a description of how to unlock RTC register write
protection.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6.11

RTC calibration register (RTC_CALR)
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be write-protected against nonsecure access. Refer to Section 63.3.4:
RTC secure protection modes.
This register can be write-protected against unprivileged access. Refer to Section 63.3.5:
RTC privilege protection modes.
Address offset: 0x28
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

CALP

CALW8

CALW
16

LPCAL

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

rw

CALM[8:0]
rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 CALP: Increase frequency of RTC by 488.5 ppm
0: No RTCCLK pulses are added.
1: One RTCCLK pulse is effectively inserted every 211 pulses (frequency increased by
488.5 ppm).
This feature is intended to be used in conjunction with CALM, which lowers the frequency of
the calendar with a fine resolution. if the input frequency is 32768 Hz, the number of
RTCCLK pulses added during a 32-second window is calculated as follows:
(512 × CALP) - CALM.
Refer to Section 63.3.16: RTC smooth digital calibration.
Bit 14 CALW8: Use an 8-second calibration cycle period
When CALW8 is set to 1, the 8-second calibration cycle period is selected.
Note: CALM[1:0] are stuck at 00 when CALW8 = 1. Refer to Section 63.3.16: RTC smooth
digital calibration.
Bit 13 CALW16: Use a 16-second calibration cycle period
When CALW16 is set to 1, the 16-second calibration cycle period is selected. This bit must
not be set to 1 if CALW8 = 1.
Note: CALM[0] is stuck at 0 when CALW16 = 1. Refer to Section 63.3.16: RTC smooth digital
calibration.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bit 12 LPCAL: RTC low-power mode
0: Calibration window is 220 RTCCLK, which is a high-consumption mode. This mode must
be set only when less than 32s calibration window is required.
1: Calibration window is 220 ck_apre, which is the required configuration for ultra-low
consumption mode.
Bits 11:9 Reserved, must be kept at reset value.
Bits 8:0 CALM[8:0]: Calibration minus
The frequency of the calendar is reduced by masking CALM out of 220 RTCCLK pulses
(32 seconds if the input frequency is 32768 Hz). This decreases the frequency of the
calendar with a resolution of 0.9537 ppm.
To increase the frequency of the calendar, this feature should be used in conjunction with
CALP. See Section 63.3.16: RTC smooth digital calibration on page 2603.

63.6.12

RTC shift control register (RTC_SHIFTR)
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x2C
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

ADD1S

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

w

w

w

w

w

w

w

w
15
Res.

SUBFS[14:0]
w

<!-- pagebreak -->

w

w

w

w

w

w

w

RM0456 Rev 6

RM0456

Real-time clock (RTC)

Bit 31 ADD1S: Add one second
0: No effect
1: Add one second to the clock/calendar
This bit is write only and is always read as zero. Writing to this bit has no effect when a shift
operation is pending (when SHPF = 1, in RTC_ICSR).
This function is intended to be used with SUBFS (see description below) in order to
effectively add a fraction of a second to the clock in an atomic operation.
Bits 30:15 Reserved, must be kept at reset value.
Bits 14:0 SUBFS[14:0]: Subtract a fraction of a second
These bits are write only and is always read as zero. Writing to this bit has no effect when a
shift operation is pending (when SHPF = 1, in RTC_ICSR).
The value which is written to SUBFS is added to the synchronous prescaler counter. Since
this counter counts down, this operation effectively subtracts from (delays) the clock by:
Delay (seconds) = SUBFS / (PREDIV_S + 1)
A fraction of a second can effectively be added to the clock (advancing the clock) when the
ADD1S function is used in conjunction with SUBFS, effectively advancing the clock by:
Advance (seconds) = (1 - (SUBFS / (PREDIV_S + 1))).
In mixed BCD-binary mode (BIN=10 or 11), the SUBFS[14:BCDU+8] must be written with 0.
Note: Writing to SUBFS causes RSF to be cleared. Software can then wait until RSF = 1 to be
sure that the shadow registers have been updated with the shifted time.

63.6.13

RTC timestamp time register (RTC_TSTR)
The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when
TSF bit is reset.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x30
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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PM
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

Res.

MNT[2:0]
r

r

MNU[3:0]
r

Res.
r

21

20

19

18

HT[1:0]

ST[2:0]
r

r

17

16

HU[3:0]

SU[3:0]

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 PM: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 HT[1:0]: Hour tens in BCD format.
Bits 19:16 HU[3:0]: Hour units in BCD format.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bit 15 Reserved, must be kept at reset value.
Bits 14:12 MNT[2:0]: Minute tens in BCD format.
Bits 11:8 MNU[3:0]: Minute units in BCD format.
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 ST[2:0]: Second tens in BCD format.
Bits 3:0 SU[3:0]: Second units in BCD format.

63.6.14

RTC timestamp date register (RTC_TSDR)
The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when
TSF bit is reset.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x34
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

r

r

r

r

r

r

r

r

r

WDU[2:0]
r

r

MT
r

r

MU[3:0]

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:13 WDU[2:0]: Week day units
Bit 12 MT: Month tens in BCD format
Bits 11:8 MU[3:0]: Month units in BCD format
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 DT[1:0]: Date tens in BCD format
Bits 3:0 DU[3:0]: Date units in BCD format

<!-- pagebreak -->

RM0456 Rev 6

DT[1:0]
r

DU[3:0]

RM0456

Real-time clock (RTC)

63.6.15

RTC timestamp subsecond register (RTC_TSSSR)
The content of this register is valid only when TSF is set to 1 in RTC_SR. It is cleared when
the TSF bit is reset.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x38
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

SS[31:16]
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

SS[15:0]
r

Bits 31:0 SS[31:0]: Subsecond value/synchronous binary counter values
SS[31:0] is the value of the synchronous prescaler counter when the timestamp event
occurred.

63.6.16

RTC alarm A register (RTC_ALRMAR)
This register can be written only when ALRAE is reset in RTC_CR register, or in initialization
mode.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x40
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

MSK4

WDSE
L

29

28

27

DT[1:0]

26

25

24

DU[3:0]

23

22

MSK3

PM

21

20

19

18

HT[1:0]

17

16

HU[3:0]

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

rw

MSK2
rw

MNT[2:0]
rw

rw

MNU[3:0]
rw

MSK1
rw

rw

RM0456 Rev 6

ST[2:0]
rw

rw

SU[3:0]

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Bit 31 MSK4: Alarm A date mask
0: Alarm A set if the date/day match
1: Date/day don’t care in alarm A comparison
Bit 30 WDSEL: Week day selection
0: DU[3:0] represents the date units
1: DU[3:0] represents the week day. DT[1:0] is don’t care.
Bits 29:28 DT[1:0]: Date tens in BCD format
Bits 27:24 DU[3:0]: Date units or day in BCD format
Bit 23 MSK3: Alarm A hours mask
0: Alarm A set if the hours match
1: Hours don’t care in alarm A comparison
Bit 22 PM: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 HT[1:0]: Hour tens in BCD format
Bits 19:16 HU[3:0]: Hour units in BCD format
Bit 15 MSK2: Alarm A minutes mask
0: Alarm A set if the minutes match
1: Minutes don’t care in alarm A comparison
Bits 14:12 MNT[2:0]: Minute tens in BCD format
Bits 11:8 MNU[3:0]: Minute units in BCD format
Bit 7 MSK1: Alarm A seconds mask
0: Alarm A set if the seconds match
1: Seconds don’t care in alarm A comparison
Bits 6:4 ST[2:0]: Second tens in BCD format.
Bits 3:0 SU[3:0]: Second units in BCD format.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6.17

RTC alarm A subsecond register (RTC_ALRMASSR)
This register can be written only when ALRAE is reset in RTC_CR register, or in initialization
mode.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x44
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

SSCLR

Res.

rw
15

29

28

27

26

25

24

MASKSS[5:0]
rw

rw

rw

rw

rw

rw

14

13

12

11

10

9

8

rw

rw

rw

rw

rw

rw

rw

Res.

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

w

rw

rw

SS[14:0]
rw

Bit 31 SSCLR: Clear synchronous counter on alarm (Binary mode only)
0: The synchronous binary counter (SS[31:0] in RTC_SSR) is free-running.
1: The synchronous binary counter (SS[31:0] in RTC_SSR) is running from 0xFFFF FFFF to
RTC_ALRABINR.SS[31:0] value and is automatically reloaded with 0xFFFF FFFF one
ck_apre cycle after reaching RTC_ALRABINR.SS[31:0].
Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11).
Bit 30 Reserved, must be kept at reset value.
Bits 29:24 MASKSS[5:0]: Mask the most-significant bits starting at this bit
0: No comparison on subseconds for Alarm A. The alarm is set when the seconds unit is
incremented (assuming that the rest of the fields match).
1: SS[31:1] are don’t care in Alarm A comparison. Only SS[0] is compared.
2: SS[31:2] are don’t care in Alarm A comparison. Only SS[1:0] are compared.
...
31: SS[31] is don’t care in Alarm A comparison. Only SS[30:0] are compared.
From 32 to 63: All 32 SS bits are compared and must match to activate alarm.
Note: In BCD mode (BIN=00) the overflow bits of the synchronous counter (bits 31:15) are
never compared. These bits can be different from 0 only after a shift operation.
Bits 23:15 Reserved, must be kept at reset value.
Bits 14:0 SS[14:0]: Subseconds value
This value is compared with the contents of the synchronous prescaler counter to determine
if alarm A is to be activated. Only bits 0 up MASKSS-1 are compared.
This field is the mirror of SS[14:0] in the RTC_ALRABINR, and so can also be read or written
through RTC_ALRABINR.
Note: SS[3:0] must be 0000 when SSCLR is set with ATCKSEL[3] = 1 in TAMP_ATCR1.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

63.6.18

RM0456

RTC alarm B register (RTC_ALRMBR)
This register can be written only when ALRBE is reset in RTC_CR register, or in initialization
mode.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x48
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

MSK4

WD
SEL

29

28

27

DT[1:0]

26

25

24

DU[3:0]

23

22

MSK3

PM

21

20

19

18

HT[1:0]

17

16

HU[3:0]

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

rw

MSK2
rw

MNT[2:0]
rw

rw

MNU[3:0]
rw

MSK1
rw

rw

ST[2:0]
rw

rw

Bit 31 MSK4: Alarm B date mask
0: Alarm B set if the date and day match
1: Date and day don’t care in alarm B comparison
Bit 30 WDSEL: Week day selection
0: DU[3:0] represents the date units
1: DU[3:0] represents the week day. DT[1:0] is don’t care.
Bits 29:28 DT[1:0]: Date tens in BCD format
Bits 27:24 DU[3:0]: Date units or day in BCD format
Bit 23 MSK3: Alarm B hours mask
0: Alarm B set if the hours match
1: Hours don’t care in alarm B comparison
Bit 22 PM: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 HT[1:0]: Hour tens in BCD format
Bits 19:16 HU[3:0]: Hour units in BCD format
Bit 15 MSK2: Alarm B minutes mask
0: Alarm B set if the minutes match
1: Minutes don’t care in alarm B comparison
Bits 14:12 MNT[2:0]: Minute tens in BCD format
Bits 11:8 MNU[3:0]: Minute units in BCD format

<!-- pagebreak -->

RM0456 Rev 6

SU[3:0]

RM0456

Real-time clock (RTC)

Bit 7 MSK1: Alarm B seconds mask
0: Alarm B set if the seconds match
1: Seconds don’t care in alarm B comparison
Bits 6:4 ST[2:0]: Second tens in BCD format
Bits 3:0 SU[3:0]: Second units in BCD format

63.6.19

RTC alarm B subsecond register (RTC_ALRMBSSR)
This register can be written only when ALRBE is reset in RTC_CR register, or in initialization
mode.
This register can be protected against nonsecure access. Refer to Section 63.3.4: RTC
secure protection modes.
This register can be protected against unprivileged access. Refer to Section 63.3.5: RTC
privilege protection modes.
Address offset: 0x4C
Backup domain reset value: 0x0000 0000
System reset: not affected

31

30

29

SSCLR

Res.

MASKSS[5:4]
rw

rw

rw

rw

rw

rw

14

13

12

11

10

9

8

rw
15

28

27

26

25

24

MASKSS[3:0]

Res.

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

w

rw

rw

SS[14:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bit 31 SSCLR: Clear synchronous counter on alarm (Binary mode only)
0: The synchronous binary counter (SS[31:0] in RTC_SSR) is free-running.
1: The synchronous binary counter (SS[31:0] in RTC_SSR) is running from 0xFFFF FFFF to
RTC_ALRBBINR.SS[31:0] value and is automatically reloaded with 0xFFFF FFFF one
ck_apre cycle after reaching RTC_ALRBBINR.SS[31:0].
Note: SSCLR must be kept to 0 when BCD or mixed mode is used (BIN = 00, 10 or 11).
Bit 30 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

