2687

Real-time clock (RTC)

RM0456

By setting the TSIE bit in the RTC_CR register, an interrupt is generated when a timestamp
event occurs.
If a new timestamp event is detected while the timestamp flag (TSF) is already set, the
timestamp overflow flag (TSOVF) flag is set and the timestamp registers (RTC_TSTR and
RTC_TSDR) maintain the results of the previous event.
Note:

TSF is set up to 2 ck_apre cycles after the timestamp event from RTC_TS pin or from rtc_its
internal signal occurs, due to synchronization process. TSF is set up to 3 ck_apre cycles
after tamper flags.
TSOVF is set up to only 1 ck_apre cycle after the event occurs. This means that if two
timestamp events are close together, TSOVF can be seen as '1' while TSF is still '0'. As a
consequence, it is recommended to poll TSOVF only after TSF has been set.

Caution:

If a timestamp event occurs immediately after the TSF bit is supposed to be cleared, then
both TSF and TSOVF bits are set. To avoid masking a timestamp event occurring at the
same moment, the application must not write 0 into TSF bit unless it has already read it to 1.

63.3.18

Calibration clock output
When the COE bit is set to 1 in the RTC_CR register, a reference clock is provided on the
CALIB device output.
If the COSEL bit in the RTC_CR register is reset and PREDIV_A = 0x7F, the CALIB
frequency is fRTCCLK/64. This corresponds to a calibration output at 512 Hz for an RTCCLK
frequency at 32.768 kHz. The CALIB duty cycle is irregular: there is a light jitter on falling
edges. It is therefore recommended to use rising edges.
When COSEL is set and “PREDIV_S+1” is a non-zero multiple of 256 (i.e: PREDIV_S[7:0] =
0xFF), the CALIB frequency is fRTCCLK/(256 * (PREDIV_A+1)). This corresponds to a
calibration output at 1 Hz for prescaler default values (PREDIV_A = Ox7F, PREDIV_S =
0xFF), with an RTCCLK frequency at 32.768 kHz.

Note:

When COSEL is cleared, the CALIB output is the output of the 6th stage of the
asynchronous prescaler. If LPCAL is changed from 0 to 1, the output can be irregular
(glitch...) during the LPCAL switch. If LPCAL = 1 this output is always available. If LPCAL =
0, no output is present if PREDIV_A is < 0x20.
When COSEL is set, the CALIB output is the output of the 8th stage of the synchronous
prescaler.

63.3.19

Tamper and alarm output
The OSEL[1:0] control bits in the RTC_CR register are used to activate the alarm output
TAMPALRM, and to select the function which is output. These functions reflect the contents
of the corresponding flags in the RTC_SR register.
When the TAMPOE control bit is set in the RTC_CR, all external and internal tamper flags
are ORed and routed to the TAMPALRM output. If OSEL = 00 the TAMPALRM output
reflects only the tampers flags. If OSEL ≠ 00, the signal on TAMPALRM provides both
tamper flags and alarm A, B, or wake-up flag.
The polarity of the TAMPALRM output is determined by the POL control bit in RTC_CR so
that the opposite of the selected flags bit is output when POL is set to 1.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

TAMPALRM output
The TAMPALRM pin can be configured in output open drain or output push-pull using the
control bit TAMPALRM_TYPE in the RTC_CR register. It is possible to apply the internal
pull-up in output mode thanks to TAMPALRM_PU in the RTC_CR.
Note:

Once the TAMPALRM output is enabled, it has priority over CALIB on RTC_OUT1.

63.4

RTC low-power modes
Table 637. Effect of low-power modes on RTC
Mode

Description

Sleep

No effect
RTC interrupts cause the device to exit the Sleep mode.

Stop

The RTC remains active when the RTC clock source is LSE or LSI. RTC interrupts
cause the device to exit the Stop mode.

Standby

The RTC remains active when the RTC clock source is LSE or LSI. RTC interrupts
cause the device to exit the Standby mode.

Shutdown

The RTC remains active when the RTC clock source is LSE. RTC interrupts cause the
device to exit the Shutdown mode.

The table below summarizes the RTC pins and functions capability in all modes.
Table 638. RTC pins functionality over modes

63.5

Functions

Functional in all lowpower modes except
Stop 3, Standby and
Shutdown modes

Functional in Stop 3,
Standby and Shutdown
modes

Functional in VBAT
mode

RTC_TS

Yes

Yes

Yes

RTC_REFIN

Yes

No

No

RTC_OUT1

Yes

Yes

Yes

RTC_OUT2

Yes

Yes

Yes

RTC interrupts
The interrupt channel is set in the masked interrupt status register or in the secure masked
interrupt status register depending on its security mode configuration. The nonsecure
interrupt output or the secure interrupt output is also activated.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456
Table 639. Nonsecure interrupt requests

Interrupt acronym

RTC

Interrupt
event

Event
flag(1)

Enable
control bit(2)

Interrupt
clear
method

Exit from
low-power
modes

Alarm A

ALRAF

ALRAIE and
(ALRASEC=0 and
SEC=0)

write 1 in
CALRAF

Yes(3)

Alarm B

ALRBF

ALRBIE and
(ALRBSEC=0 and
SEC=0)

write 1 in
CALRBF

Yes(3)

Timestamp

TSF

TSIE and (TSSEC=0
and SEC=0)

write 1 in
CTSF

Yes(3)

Wake-up timer

WUTF

WUTIE and
(WUTSEC=0 and
SEC=0)

write 1 in
CWUTF

Yes(3)

SSR underflow
(reload)

SSRUF

SSRUIE and
(ALRASEC=0 and
SEC=0)

write 1 in
CSSRUF

Yes(3)

1. The event flags are in the RTC_SR register.
2. The interrupt masked flags (resulting from event flags AND enable control bits) are in the RTC_MISR
register.
3. When the RTC is clocked by an oscillator functional in the low-power mode.

Table 640. Secure interrupt requests
Interrupt acronym

RTC_S

Interrupt
event

Event
flag(1)

Enable
control bit(2)

Interrupt
clear
method

Exit from
low-power
modes

Alarm A

ALRAF

ALRAIE and
(ALRASEC=1 or
SEC=1)

write 1 in
CALRAF

Yes(3)

Alarm B

ALRBF

ALRBIE and
(ALRBSEC=1 or
SEC=1)

write 1 in
CALRBF

Yes(3)

Timestamp

TSF

TSIE and
(TSSEC=1 or
SEC=1)

write 1 in
CTSF

Yes(3)

Wake-up timer

WUTF

WUTIE and
(WUTSEC=1 or
SEC=1)

write 1 in
CWUTF

Yes(3)

SSR underflow
(reload)

SSRUF

SSRUIE and
(ALRASEC=1 or
SEC=1)

write 1 in
CSSRUF

Yes(3)

1. The event flags are in the RTC_SR register.
2. The interrupt masked flags (resulting from event flags AND enable control bits) are in the RTC_SMISR
register.
3. When the RTC is clocked by an oscillator functional in the low-power mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63.6

RTC registers
Refer to Section 1.2 of the reference manual for a list of abbreviations used in register
descriptions.
The peripheral registers can be accessed by words (32-bit).

63.6.1

RTC time register (RTC_TR)
The RTC_TR is the calendar time shadow register. This register must be written in
initialization mode only. Refer to Calendar initialization and configuration on page 2599 and
Reading the calendar on page 2600.
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be write-protected against nonsecure access. Refer to Section 63.3.4:
RTC secure protection modes.
This register can be write-protected against unprivileged access. Refer to Section 63.3.5:
RTC privilege protection modes.
Address offset: 0x00
Backup domain reset value: 0x0000 0000
System reset value: 0x0000 0000 (when BYPSHAD = 0, not affected when BYPSHAD = 1)

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
rw

15

14

Res.

13

12

11

MNT[2:0]
rw

rw

10

9

8

MNU[3:0]
rw

rw

rw

rw

7

6

Res.
rw

21

20

19

17

16

HU[3:0]

rw

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

ST[2:0]
rw

18

HT[1:0]

rw

SU[3:0]
rw

rw

rw

Bits 31:23 Reserved, must be kept at reset value.
Bit 22 PM: AM/PM notation
0: AM or 24-hour format
1: PM
Bits 21:20 HT[1:0]: Hour tens in BCD format
Bits 19:16 HU[3:0]: Hour units in BCD format
Bit 15 Reserved, must be kept at reset value.
Bits 14:12 MNT[2:0]: Minute tens in BCD format
Bits 11:8 MNU[3:0]: Minute units in BCD format
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 ST[2:0]: Second tens in BCD format
Bits 3:0 SU[3:0]: Second units in BCD format

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

63.6.2

RM0456

RTC date register (RTC_DR)
The RTC_DR is the calendar date shadow register. This register must be written in
initialization mode only. Refer to Calendar initialization and configuration on page 2599 and
Reading the calendar on page 2600.
This register is write protected. The write access procedure is described in RTC register
write protection on page 2598.
This register can be write-protected against nonsecure access. Refer to Section 63.3.4:
RTC secure protection modes.
This register can be write-protected against unprivileged access. Refer to Section 63.3.5:
RTC privilege protection modes.
Address offset: 0x04
Backup domain reset value: 0x0000 2101
System reset value: 0x0000 2101 (when BYPSHAD = 0, not affected when BYPSHAD = 1)

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

WDU[2:0]
rw

rw

MT
rw

rw

MU[3:0]
rw

rw

rw

23

22

21

20

19

18

YT[3:0]
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

Res.

Res.

rw

rw

rw

DT[1:0]
rw

rw

DU[3:0]
rw

rw

Bits 23:20 YT[3:0]: Year tens in BCD format
Bits 19:16 YU[3:0]: Year units in BCD format
Bits 15:13 WDU[2:0]: Week day units
000: forbidden
001: Monday
...
111: Sunday
Bit 12 MT: Month tens in BCD format
Bits 11:8 MU[3:0]: Month units in BCD format
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 DT[1:0]: Date tens in BCD format
Bits 3:0 DU[3:0]: Date units in BCD format

<!-- pagebreak -->

