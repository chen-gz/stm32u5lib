Don’t
care

0

1

RM0456 Rev 6

RM0456

Real-time clock (RTC)

00

0

1

Don’t
care

Don’t
care

0

00

0

0

00

0

1

Don’t
care

Don’t
care

0

00

0

0

00

0

1

RTC_TS and TAMP_IN1
input floating

RTC_TS input floating

Wake-up pin or Standard
GPIO

Don’t
care

Don’t
care

0

1

Don’t
care

Don’t
care

0

1

0

1

0

0

0
0
Don’t
care

Don’t
care

0
0

Don’t
care
1

1

0

Don’t
care
1

TSE
(RTC_TS input enable)

Don’t
care

TAMP1E
(TAMP_IN1 input enable)

0

TAMP2E=TAMP2AM=1 with
ATOSHARE=0, or TAMPxE=TAMPxAM=1
with ATOSHARE=1 and ATOSELx=1

OUT2EN

0

TAMPALRM_PU

COE
(CALIB output enable)

00

TAMPALRM_TYPE

TAMPOE
(TAMPER output enable)

PC13 pin function

OSEL[1:0]
(ALARM output enable)

Table 635. RTC pin PC13 configuration(1) (continued)

0
Don’t
care

Don’t
care

0
0

1. OD: open drain; PP: push-pull.
2. In this configuration the GPIO must be configured in input.

In addition, it is possible to output RTC_OUT2 on PB2 pin thanks to OUT2EN bit. The
different functions are mapped on RTC_OUT1 or on RTC_OUT2 depending on OSEL, COE
and OUT2EN configuration, as shown in table Table 636.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456
Table 636. RTC_OUT mapping

63.3.4

OSEL[1:0] bits
ALARM
output enable)

COE bit (CALIB
output enable)

00

0

00

1

01 or 10 or 11

OUT2EN
bit

RTC_OUT1 on
PC13

RTC_OUT2 on
PB2

-

-

CALIB

-

Don’t care

TAMPALRM

-

00

0

-

-

00

1

-

CALIB

01 or 10 or 11

0

-

TAMPALRM

01 or 10 or 11

1

TAMPALRM

CALIB

0

1

RTC secure protection modes
By default after a backup domain power-on reset, all RTC registers can be read or written in
both secure and nonsecure modes, except for the RTC secure configuration register
(RTC_SECCFGR) which can be written in secure mode only. The RTC protection
configuration is not affected by a system reset.
When the SEC bit is set in the RTC_SECCFGR register:
•

Writing the RTC registers is possible only in secure mode.

•

Reading RTC_SECCFGR, RTC_PRIVCFGR, RTC_MISR, RTC_TR, RTC_DR,
RTC_SSR, RTC_PRER and RTC_CALR is always possible in secure and nonsecure
modes. All the other RTC registers can be read only in secure mode.

When the SEC bit is cleared, it is still possible to protect some of the registers by setting
dedicated INITSEC, CALSEC, TSSEC, WUTSEC, ALRASEC or ALRBSEC control bits. If all
these bits are also clear, all the RTC registers can be read and written in secure and
nonsecure mode.
•

•

•

When INITSEC is set:
–

RTC_TR, RTC_DR, RTC_PRER registers, plus INIT, BIN and BCDU in
RTC_ICSR, FMT control bits in RTC_CR and INITPRIV in the RTC_PRIVCFGR
can be written only in secure mode.

–

These registers and control bits can be read in secure and nonsecure mode.

When CALSEC is set:
–

RTC_SHIFTR and RTC_CALR registers, plus ADD1H, SUB1H and REFCKON
control bits in the RTC_CR and CALPRIV in the RTC_PRIVCFGR can be written
only in secure mode.

–

These registers and control bits can be read in secure and nonsecure mode.

When ALRASEC is set:
–

<!-- pagebreak -->

RTC_ALRMAR, RTC_ALRMASSR and RTC_ALRABINR registers, plus ALRAE,
ALRAFCLR, ALRAIE and SSRUIE in the RTC_CR, CALRAF and CSSRUF in the

RM0456 Rev 6

RM0456

Real-time clock (RTC)
RTC_SCR, ALRAF and SSRUF in RTC_SR, and ALRAMF and SSRUMF in
RTC_SMISR can be read and written only in secure mode.
–
•

•

•

ALRAPRIV in the RTC_PRIVCFGR can be written only in secure mode

When ALRBSEC is set:
–

RTC_ALRMBR, RTC_ALRMBSSR and RTC_ALRBBINR registers, plus ALRBE
ALRBFCLR, ALRBIE in the RTC_CR, CALRBF in the RTC_SCR, ALRBF in
RTC_SR, and ALRBMF in RTC_SMISR can be read and written only in secure
mode.

–

ALRBPRIV in the RTC_PRIVCFGR can be written only in secure mode.

When WUTSEC is set:
–

RTC_WUTR register, plus WUTE, WUTIE and WUCKSEL control bits in the
RTC_CR, CWUTF in the RTC_SCR, WUTF in RTC_SR, and WUTMF in
RTC_SMISR can be read and written only in secure mode.

–

WUTPRIV in the RTC_PRIVCFGR can be written only in secure mode

When TSSEC is set:
–

RTC_TSTR, RTC_TSDR and RTC_TSSSR registers, plus TAMPTS, ITSE, TSE,
TSIE, TSEDGE control bits in the RTC_CR, CITSF, CTSOVF and CTSF bits in the
RTC_SCR, TSF, TSOVF and ITSF in RTC_SR, and TSMF, TSOVMF and ITSMF
in RTC_SMISR can be read and written only in secure mode.

–

TSPRIV in the RTC_PRIVCFGR can be written only in secure mode

A nonsecure access to a secure-protected register is denied:
•

There is no bus error generated.

•

In case the register has a global protection: a notification is generated through a
flag/interrupt in the TZIC (TrustZone illegal access controller). In case only a few bits of
the register are protected (for registers with mixed features such as RTC_CR...), no
notification is generated.

•

When write protected, the bits are not written.

•

When read protected they are read as 0.

As soon as at least one function is configured to be secured, the RTC reset and clock
control is also secured in the RCC.

63.3.5

RTC privilege protection modes
By default after a backup domain power-on reset, all RTC registers can be read or written in
both privileged and unprivileged modes, except for the RTC privilege mode control register
(RTC_PRIVCFGR) which can be written in privilege mode only. The RTC protection
configuration is not affected by a system reset.
When the PRIV bit is set in the RTC_PRIVCFGR register:
•

Writing the RTC registers is possible only in privileged mode.

•

Reading the RTC_SECCFGR, RTC_PRIVCFGR, RTC_TR, RTC_DR, RTC_SSR,
RTC_PRER and RTC_CALR is always possible in privilege and unprivilege modes. All
the other RTC registers can be read only in privilege mode.

When the PRIV bit is cleared, it is still possible to protect some of the registers by setting
dedicated INITPRIV, CALPRIV, TSPRIV, WUTPRIV, ALRAPRV or ALRBPRIV control bits. If

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

all these bits are also cleared, all the RTC registers can be read or written in privilege and
unprivilege modes.
•

•

•

•

•

•

When INITPRIV is set:
–

RTC_TR, RTC_DR, RTC_PRER registers, plus INIT, BIN and BCDU in
RTC_ICSR and FMT control bits in RTC_CR, plus INITSEC in the
RTC_SECCFGR can be written only in privilege mode.

–

These registers and control bits can be read in privilege and unprivilege mode.

When CALPRIV is set:
–

RTC_SHIFTR and RTC_CALR registers, plus ADD1H, SUB1H and REFCKON
control bits in the RTC_CR, plus CALDSEC in the RTC_SECCFGR can be written
only in privilege mode.

–

These registers and control bits can be read in privilege and unprivilege mode.

When ALRAPRIV is set:
–

RTC_ALRMAR, RTC_ALRMASSR and RTC_ALRABINR registers, plus ALRAE,
ALRAFCLR, ALRAIE and SSRUIE in the RTC_CR, and CALRAF and CSSRUF in
the RTC_SCR, ALRAF and SSRUF in RTC_SR, and ALRAMF and SSRUMF in
RTC_MISR and RTC_SMISR can be read and written only in privilege mode.

–

ALRASEC in the RTC_SECCFGR can be written only in privilege mode.

When ALRBPRIV is set:
–

RTC_ALRMBR, RTC_ALRMBSSR and RTC_ALRBBINR registers, plus ALRBE,
ALRBFCLR, ALRBIE in the RTC_CR, and CALRBF in the RTC_SCR, ALRBF in
RTC_SR, and ALRBMF in RTC_MISR and RTC_SMISR can be read and written
only in privilege mode.

–

ALRBSEC in the RTC_SECCFGR can be written only in privilege mode.

When WUTPRIV is set:
–

RTC_WUTR register, plus WUTE, WUTIE and WUCKSEL control bits in the
RTC_CR, and CWUTF in the RTC_SCR, WUTF in RTC_SR, and WUTMF in
RTC_MISR and RTC_SMISR can be read and written only in privilege mode.

–

WUTSEC in the RTC_SECCFGR can be written only in privilege mode.

When TSPRIV is set:
–

RTC_TSTR, RTC_TSDR and RTC_TSSSR registers, plus TAMPTS, ITSE, TSE,
TSIE, TSEDGE control bits in the RTC_CR, CITSF, CTSOVF and CTSF bits in the
RTC_SCR, TSF, TSOVF and ITSF in RTC_SR, and TSMF, TSOVMF and ITSMF
in RTC_MISR and RTC_SMISR can be read and written only in privilege mode.

–

TSSEC in the RTC_SECCFGR can be written only in privilege mode.

A unprivileged access to a privileged-protected register is denied:

63.3.6

•

There is no bus error generated.

•

When write protected, the bits are not written.

•

When read protected they are read as 0.

Clock and prescalers
The RTC clocks must respect this ratio: frequency(PCLK) ≥ 2 × frequency(RTCCLK).
For more information on the RTC clock (RTCCLK) source configuration, refer to “Reset and
clock control (RCC)”.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

BCD mode (BIN=00)
A programmable prescaler stage generates a 1 Hz clock which is used to update the
calendar. To minimize power consumption, the prescaler is split into 2 programmable
prescalers (see Figure 773: RTC block diagram):

Note:

•

A 7-bit asynchronous prescaler configured through the PREDIV_A bits of the
RTC_PRER register.

•

A 15-bit synchronous prescaler configured through the PREDIV_S bits of the
RTC_PRER register.

When both prescalers are used, it is recommended to configure the asynchronous prescaler
to a high value to minimize consumption.
The asynchronous prescaler division factor is set to 128, and the synchronous division
factor to 256, to obtain an internal clock frequency of 1 Hz (ck_spre) with an LSE frequency
of 32.768 kHz.
The minimum division factor is 1 and the maximum division factor is 222.
This corresponds to a maximum input frequency of around 4 MHz.
fck_apre is given by the following formula:
f RTCCLK
f CK_APRE = -------------------------------------PREDIV_A + 1

The ck_apre clock is used to clock the binary RTC_SSR subsecond downcounter. When it
reaches 0, RTC_SSR is reloaded with the content of PREDIV_S.
fck_spre is given by the following formula:
f RTCCLK
f CK_SPRE = ----------------------------------------------------------------------------------------------( PREDIV_S + 1 ) × ( PREDIV_A + 1 )

The ck_spre clock can be used either to update the calendar or as timebase for the 16-bit
wake-up auto-reload timer. To obtain short timeout periods, the 16-bit wake-up auto-reload
timer can also run with the RTCCLK divided by the programmable 4-bit asynchronous
prescaler (see Section 63.3.10: Periodic auto-wake-up for details).

Binary mode (BIN=01)
The SSR binary down-counter is extended to 32-bit length and is free running. The time and
date calendar BCD registers are not functional.
This down-counter is clocked by ck_apre: the output of the 7-bit asynchronous prescaler
configured through the PREDIV_A bits of the RTC_PRER register.
PREDIV_S value is don’t care.

Mixed mode (BIN=10 or 11)
The SSR binary down-counter is extended to 32-bit length and is free running. The time and
date calendar BCD registers are also available.
This down-counter is clocked by ck_apre: the output of the 7-bit asynchronous prescaler
configured through the PREDIV_A bits of the RTC_PRER register. The bits BCDU[2:0] are

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

used to define when the calendar is incremented by 1 second, using the SSR least
significant bits.

63.3.7

Real-time clock and calendar
The RTC calendar time and date registers are accessed through shadow registers which
are synchronized with PCLK (APB clock). They can also be accessed directly in order to
avoid waiting for the synchronization duration.
•

RTC_SSR for the subseconds

•

RTC_TR for the time

•

RTC_DR for the date

Every RTCCLK periods, the current calendar value is copied into the shadow registers, and
the RSF bit of RTC_ICSR register is set (see Section 63.6.12: RTC shift control register
(RTC_SHIFTR)). The copy is not performed in Stop and Standby mode. When exiting these
modes, the shadow registers are updated after up to 4 RTCCLK periods.
When the application reads the calendar registers, it accesses the content of the shadow
registers. It is possible to make a direct access to the calendar registers by setting the
BYPSHAD control bit in the RTC_CR register. By default, this bit is cleared, and the user
accesses the shadow registers.
When reading the RTC_SSR, RTC_TR or RTC_DR registers in BYPSHAD = 0 mode, the
frequency of the APB clock (fAPB) must be at least 7 times the frequency of the RTC clock
(fRTCCLK).
The shadow registers are reset by system reset.

63.3.8

Calendar ultra-low power mode
It is possible to reduce drastically the RTC power consumption by setting the LPCAL bit in
the RTC_CALR register. In this configuration, the whole RTC is clocked by ck_apre only
instead of both RTCCLK and ck_apre. Consequently, some flags delays are longer, and the
calibration window is longer (refer to Section : RTC ultra-low-power mode).
The LPCAL bit is ignored (assumed to be 0) when asynchronous prescaler division factor
(PREDIV_A+1) is not a power of 2.
Switching from LPCAL=0 to LPCAL=1 or from LPCAL=1 to LPCAL=0 is not immediate and
requires a few ck_apre periods to complete.

63.3.9

Programmable alarms
The RTC unit provides programmable alarm: alarm A and alarm B. The description below is
given for alarm A, but can be translated in the same way for alarm B.
The programmable alarm function is enabled through the ALRAE bit in the RTC_CR
register.
The ALRAF is set to 1 if the calendar subseconds, seconds, minutes, hours, date or day
match the values programmed in the alarm registers RTC_ALRMASSR and
RTC_ALRMAR. Each calendar field can be independently selected through the MSKx bits
of the RTC_ALRMAR register, and through the MASKSSx bits of the RTC_ALRMASSR
register.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)
When the binary mode is used, the subsecond field can be programmed in the alarm binary
register RTC_ALRABINR.
The alarm interrupt is enabled through the ALRAIE bit in the RTC_CR register.
In case the Alarm is used to generate a trigger event for another peripheral, the ALRAF can
be automatically cleared by hardware by configuring the ALRAFCLR bit at 1 in the RTC_CR
register. In this configuration there is no need for software intervention if the only purpose is
clearing the ALRAF flag.

Caution:

If the seconds field is selected (MSK1 bit reset in RTC_ALRMAR), the synchronous
prescaler division factor set in the RTC_PRER register must be at least 3 to ensure correct
behavior.
Alarm A and alarm B (if enabled by bits OSEL[1:0] in RTC_CR register) can be routed to the
TAMPALRM output. TAMPALRM output polarity can be configured through bit POL the
RTC_CR register.

63.3.10

Periodic auto-wake-up
The periodic wake-up flag is generated by a 16-bit programmable auto-reload downcounter. The wake-up timer range can be extended to 17 bits.
The wake-up function is enabled through the WUTE bit in the RTC_CR register.
The wake-up timer clock input ck_wut can be:
•

RTC clock (RTCCLK) divided by 2, 4, 8, or 16.
When RTCCLK is LSE (32.768 kHz), this permits the wake-up interrupt period to be
configured from 122 µs to 32 s, with a resolution down to 61 µs.

•

ck_spre (usually 1 Hz internal clock) in BCD mode, or the clock used to update the
calendar as defined by BCDU in binary or mixed (BCD-binary) modes.
When ck_spre frequency is 1 Hz, a wake-up time from 1 s to around 36 hours can be
achieved with one-second resolution. This large programmable time range is divided in
2 parts:
–

from 1 s to 18 hours when WUCKSEL [2:1] = 10

–

and from around 18 h to 36 h when WUCKSEL[2:1] = 11. In this last case 216 is
added to the 16-bit counter current value. When the initialization sequence is
complete (see Programming the wake-up timer on page 2600), the timer starts
counting down. When the wake-up function is enabled, the down-counting
remains active in low-power modes. In addition, when it reaches 0, the WUTF flag
is set in the RTC_SR register, and the wake-up counter is automatically reloaded
with its reload value (RTC_WUTR register value).

Depending on WUTOCLR in the RTC_WUTR register, the WUTF flag must either be
cleared by software (WUTOCLR = 0x0000), or the WUTF is automatically cleared by
hardware when the auto-reload down counter reaches WUTOCLR value
(0x0000 < WUTOCLR ≤ WUT).
The wake-up flag is output on an internal signal rtc_wut that can be used by other
peripherals (refer to section Section 63.3.1: RTC block diagram).
When the periodic wake-up interrupt is enabled by setting the WUTIE bit in the RTC_CR
register, it can exit the device from low-power modes.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

The periodic wake-up flag can be routed to the TAMPALRM output provided it has been
enabled through bits OSEL[1:0] of RTC_CR register. TAMPALRM output polarity can be
configured through the POL bit in the RTC_CR register.
System reset, as well as low-power modes (Sleep, Stop, and Standby) have no influence on
the wake-up timer.

63.3.11

RTC initialization and configuration
RTC Binary, BCD or Mixed mode
By default the RTC is in BCD mode (BIN = 00 in the RTC_ICSR register): the RTC_SSR
register contains the subsecond field SS[15:0], clocked by ck_apre, allowing to generate a
1 Hz clock to update the calendar registers in BCD format (RTC_TR and RTC_DR).
When the RTC is configured in binary mode (BIN = 01 in the RTC_ICSR register): the
RTC_SSR register contains the binary counter SS[31:0], clocked by ck_apre. The calendar
registers in BCD format (RTC_TR and RTC_DR) are not used.
When the RTC is configured in mixed mode (BIN = 10 or 11 in the RTC_ICSR register): the
RTC_SSR register contains the binary counter SS[31:0], clocked by ck_apre. The calendar
is updated (1 second increment) each time the SSR[BCDU+7:0] reaches 0.

RTC register write protection
After system reset, the RTC registers are protected against parasitic write access by the
DBP bit in the power control peripheral (refer to the PWR power control section). DBP bit
must be set in order to enable RTC registers write access.
After Backup domain reset, some of the RTC registers are write-protected: RTC_TR,
RTC_DR, RTC_PRER, RTC_CALR, RTC_SHIFTR, the bits INIT, BIN and BCDU in
RTC_ICSR and the bits FMT, SUB1H, ADD1H, REFCKON in RTC_CR.
The following steps are required to unlock the write protection on the protected RTC
registers.
1.

Write 0xCA into the RTC_WPR register.

2.

Write 0x53 into the RTC_WPR register.

Writing a wrong key reactivates the write protection.
The protection mechanism is not affected by system reset.
The registers protected by INITPRIV are write-protected by the INIT KEY.
The registers protected by CALPRIV are write-protected by the CAL KEY.
In case PRIV or INITPRIV is set in the RTC_PRIVCFGR, and/or SEC or INITSEC is set in
the RTC_SECCFGR: the INIT KEY is unlocked and locked only if the write accesses into
the RTC_WPR register are done in the privilege and security mode defined by PRIV,
INITPRIV, SEC, INITSEC configuration.
In case PRIV or CALPRIV is set in the RTC_PRIVCFGR, and/or SEC or CALSEC is set in
the RTC_SECCFGR: the CAL KEY is unlocked and locked only if the write accesses into
the RTC_WPR register are done in the privilege and security mode defined by PRIV,
CALPRIV, SEC, CALSEC configuration.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)

Calendar initialization and configuration
To program the initial time and date calendar values, including the time format and the
prescaler configuration, the following sequence is required:
1.

Set INIT bit to 1 in the RTC_ICSR register to enter initialization mode. In this mode, the
calendar counter is stopped and its value can be updated.

2.

Poll INITF bit of in the RTC_ICSR register. The initialization phase mode is entered
when INITF is set to 1.
–

If LPCAL=0: INITF is set around 2 RTCCLK cycles after INIT bit is set.

–

If LPCAL=1: INITF is set up to 2 ck_apre cycle after INIT bit is set.

3.

To generate a 1 Hz clock for the calendar counter, program both the prescaler factors
in RTC_PRER register, plus BIN and BCDU in the RTC_ICSR register.

4.

Load the initial time and date values in the shadow registers (RTC_TR and RTC_DR),
and configure the time format (12 or 24 hours) through the FMT bit in the RTC_CR
register.

5.

Exit the initialization mode by clearing the INIT bit. The actual calendar counter value is
then automatically loaded.
–

If LPCAL=0: the counting restarts after 4 RTCCLK clock cycles.

–

If LPCAL=1: the counting restarts after up to 2 RTCCLK + 1 ck_apre.

When the initialization sequence is complete, the calendar starts counting. The RTC_SSR
content is initialized with:
•

PREDIV_S in BCD mode (BIN=00)

•

0xFFFF FFFF in binary or mixed (BCD-binary) modes (BIN=01, 10 or 11).

In BCD mode, RTC_SSR contains the value of the synchronous prescaler counter. This
enables one to calculate the exact time being maintained by the RTC down to a resolution of
1 / (PREDIV_S + 1) seconds. As a consequence, the resolution can be improved by
increasing the synchronous prescaler value (PREDIV_S[14:0]). The maximum resolution
allowed (30.52 μs with a 32768 Hz clock) is obtained with PREDIV_S set to 0x7FFF.
However, increasing PREDIV_S means that PREDIV_A must be decreased in order to
maintain the synchronous prescaler output at 1 Hz. In this way, the frequency of the
asynchronous prescaler output increases, which may increase the RTC dynamic
consumption. The RTC dynamic consumption is optimized for PREDIV_A+1 being a power
of 2.
Note:

After a system reset, the application can read the INITS flag in the RTC_ICSR register to
check if the calendar has been initialized or not. If this flag equals 0, the calendar has not
been initialized since the year field is set at its Backup domain reset default value (0x00).

Note:

To read the calendar after initialization, the software must first check that the RSF flag is set
in the RTC_ICSR register.

Daylight saving time
The daylight saving time management is performed through bits SUB1H, ADD1H, and BKP
of the RTC_CR register.
Using SUB1H or ADD1H, the software can subtract or add one hour to the calendar in one
single operation without going through the initialization procedure.
In addition, the software can use the BKP bit to memorize this operation.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

Programming the alarm
A similar procedure must be followed to program or update the programmable alarms. The
procedure below is given for alarm A but can be translated in the same way for alarm B.

Note:

1.

Clear ALRAE in RTC_CR to disable alarm A.

2.

Program the alarm A registers (RTC_ALRMASSR/RTC_ALRMAR or
RTC_ALRABINR).

3.

Set ALRAE in the RTC_CR register to enable alarm A again.

Each change of the RTC_CR register is taken into account after around 2 RTCCLK clock
cycles due to clock synchronization.

Programming the wake-up timer
The following sequence is required to configure or change the wake-up timer auto-reload
value (WUT[15:0] in RTC_WUTR):
1.

Clear WUTE in RTC_CR to disable the wake-up timer.

2.

Poll WUTWF until it is set in RTC_ICSR to make sure the access to wake-up autoreload counter and to WUCKSEL[2:0] bits is allowed. This step must be skipped in
calendar initialization mode.

3.

63.3.12

–

If WUCKSEL[2] = 0: WUTWF is set around 1 ck_wut + 1 RTCCLK cycles after
WUTE bit is cleared.

–

If WUCKSEL[2] = 1: WUTWF is set up to 1 ck_apre + 1 RTCCLK cycles after
WUTE bit is cleared.

Program the wake-up auto-reload value WUT[15:0], WUTOCLR[15:0] and the wake-up
clock selection (WUCKSEL[2:0] bits in RTC_CR). Set WUTE in RTC_CR to enable the
timer again. The wake-up timer restarts down-counting.
–

If WUCKSEL[2] = 0: WUTWF is cleared around 1 ck_wut + 1 RTCCLK cycles after
WUTE bit is set.

–

If WUCKSEL[2] = 1: WUTWF is cleared up to 1 ck_apre + 1 RTCCLK cycles after
WUTE bit is set.

Reading the calendar
When BYPSHAD control bit is cleared in the RTC_CR register
To read the RTC calendar registers (RTC_SSR, RTC_TR and RTC_DR) properly, the APB
clock frequency (fPCLK) must be equal to or greater than seven times the RTC clock
frequency (fRTCCLK). This ensures a secure behavior of the synchronization mechanism.
If the APB clock frequency is less than seven times the RTC clock frequency, the software
must read the calendar time and date registers twice. If the second read of the RTC_TR
gives the same result as the first read, this ensures that the data is correct. Otherwise a third
read access must be done. In any case the APB clock frequency must never be lower than
the RTC clock frequency.
The RSF bit is set in RTC_ICSR register each time the calendar registers are copied into
the RTC_SSR, RTC_TR and RTC_DR shadow registers. The copy is performed every
RTCCLK cycle. To ensure consistency between the 3 values, reading either RTC_SSR or
RTC_TR locks the values in the higher-order calendar shadow registers until RTC_DR is
read. In case the software makes read accesses to the calendar in a time interval smaller
than 1 RTCCLK periods: RSF must be cleared by software after the first calendar read, and

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)
then the software must wait until RSF is set before reading again the RTC_SSR, RTC_TR
and RTC_DR registers.
After waking up from low-power mode (Stop or Standby), RSF must be cleared by software.
The software must then wait until it is set again before reading the RTC_SSR, RTC_TR and
RTC_DR registers.
The RSF bit must be cleared after wake-up and not before entering low-power mode.
After a system reset, the software must wait until RSF is set before reading the RTC_SSR,
RTC_TR and RTC_DR registers. Indeed, a system reset resets the shadow registers to
their default values.
After an initialization (refer to Calendar initialization and configuration on page 2599): the
software must wait until RSF is set before reading the RTC_SSR, RTC_TR and RTC_DR
registers.
After synchronization (refer to Section 63.3.14: RTC synchronization): the software must
wait until RSF is set before reading the RTC_SSR, RTC_TR and RTC_DR registers.

When the BYPSHAD control bit is set in the RTC_CR register (bypass shadow
registers)
Reading the calendar registers gives the values from the calendar counters directly, thus
eliminating the need to wait for the RSF bit to be set. This is especially useful after exiting
from low-power modes (Stop or Standby), since the shadow registers are not updated
during these modes.
When the BYPSHAD bit is set to 1, the results of the different registers might not be
coherent with each other if an RTCCLK edge occurs between two read accesses to the
registers. Additionally, the value of one of the registers may be incorrect if an RTCCLK edge
occurs during the read operation. The software must read all the registers twice, and then
compare the results to confirm that the data is coherent and correct. Alternatively, the
software can just compare the two results of the least-significant calendar register.
Note:

While BYPSHAD = 1, instructions which read the calendar registers require one extra APB
cycle to complete.

63.3.13

Resetting the RTC
The calendar shadow registers (RTC_SSR, RTC_TR and RTC_DR) and some bits of the
RTC status register (RTC_ICSR) are reset to their default values by all available system
reset sources.
On the contrary, the following registers are reset to their default values by a Backup domain
reset and are not affected by a system reset: the RTC current calendar registers, the RTC
control register (RTC_CR), the prescaler register (RTC_PRER), the RTC calibration register
(RTC_CALR), the RTC shift register (RTC_SHIFTR), the RTC timestamp registers
(RTC_TSSSR, RTC_TSTR and RTC_TSDR), the wake-up timer register (RTC_WUTR), the
alarm A and alarm B registers (RTC_ALRMASSR/RTC_ALRMAR/RTC_ALRABINR and
RTC_ALRMBSSR/RTC_ALRMBR/RTC_ALRBBINR).
In addition, when clocked by LSE, the RTC keeps on running under system reset if the reset
source is different from the Backup domain reset one (refer to RCC for details about RTC
clock sources not affected by system reset). When a Backup domain reset occurs, the RTC
is stopped and all the RTC registers are set to their reset values.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

63.3.14

RM0456

RTC synchronization
The RTC can be synchronized to a remote clock with a high degree of precision. After
reading the subsecond field (RTC_SSR or RTC_TSSSR), a calculation can be made of the
precise offset between the times being maintained by the remote clock and the RTC. The
RTC can then be adjusted to eliminate this offset by “shifting” its clock by a fraction of a
second using RTC_SHIFTR.
The RTC can be finely adjusted using the RTC shift control register (RTC_SHIFTR). Writing
to RTC_SHIFTR can shift (either delay or advance) the clock with a resolution of 1 ck_apre
period.
The shift operation consists in adding the SUBFS[14:0] value to the synchronous prescaler
counter SS[15:0]: this delays the clock.
If at the same time the ADD1S bit is set in BCD or mixed mode, this results in adding one
second and at the same time subtracting a fraction of second, so this advances the clock.
ADD1S has no effect in binary mode.
As soon as a shift operation is initiated by a write to the RTC_SHIFTR register, the SHPF
flag is set by hardware to indicate that a shift operation is pending. This bit is cleared by
hardware as soon as the shift operation has completed.

Caution:

In mixed mode (BIN=10 or 11), the SUBFS[14:BCDU+8] must be written with 0.

Caution:

Before initiating a shift operation in BCD mode, the user must check that SS[15] = 0 in order
to ensure that no overflow occurs. In mixed mode, the user must check that the bit
SS[BCDU+8] = 0.

Caution:

This synchronization feature is not compatible with the reference clock detection feature:
firmware must not write to RTC_SHIFTR when REFCKON = 1.

63.3.15

RTC reference clock detection
This feature is available only in BCD mode (BIN=00).
The update of the RTC calendar can be synchronized to a reference clock, RTC_REFIN,
which is usually the mains frequency (50 or 60 Hz). The precision of the RTC_REFIN
reference clock should be higher than the 32.768 kHz LSE clock. When the RTC_REFIN
detection is enabled (REFCKON bit of RTC_CR set to 1), the calendar is still clocked by the
LSE, and RTC_REFIN is used to compensate for the imprecision of the calendar update
frequency (1 Hz).
Each 1 Hz clock edge is compared to the nearest RTC_REFIN clock edge (if one is found
within a given time window). In most cases, the two clock edges are properly aligned. When
the 1 Hz clock becomes misaligned due to the imprecision of the LSE clock, the RTC shifts
the 1 Hz clock a bit so that future 1 Hz clock edges are aligned. Thanks to this mechanism,
the calendar becomes as precise as the reference clock.
The RTC detects if the reference clock source is present by using the 256 Hz clock
(ck_apre) generated from the 32.768 kHz quartz. The detection is performed during a time
window around each of the calendar updates (every 1 s). The window equals 7 ck_apre
periods when detecting the first reference clock edge. A smaller window of 3 ck_apre
periods is used for subsequent calendar updates.
Each time the reference clock is detected in the window, the asynchronous prescaler which
outputs the ck_spre clock is forced to reload. This has no effect when the reference clock
and the 1 Hz clock are aligned because the prescaler is being reloaded at the same

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Real-time clock (RTC)
moment. When the clocks are not aligned, the reload shifts future 1 Hz clock edges a little
for them to be aligned with the reference clock.
If the reference clock halts (no reference clock edge occurred during the 3 ck_apre window),
the calendar is updated continuously based solely on the LSE clock. The RTC then waits for
the reference clock using a large 7 ck_apre period detection window centered on the
ck_spre edge.
When the RTC_REFIN detection is enabled, PREDIV_A and PREDIV_S must be set to their
default values:
•

PREDIV_A = 0x007F

•

PREVID_S = 0x00FF

Note:

RTC_REFIN clock detection is not available in Standby mode.

63.3.16

RTC smooth digital calibration
The RTC frequency can be digitally calibrated with a resolution of about 0.954 ppm with a
range from -487.1 ppm to +488.5 ppm. The correction of the frequency is performed using
series of small adjustments (adding and/or subtracting individual ck_cal pulses).
If LPCAL=0: ck_cal = RTCCLK
If LPCAL=1: ck_cal = ck_apre
These adjustments are fairly well distributed so that the RTC is well calibrated even when
observed over short durations of time.

RTC ultra-low-power mode
The RTC consumption can be reduced by setting the LPCAL bit in the RTC calibration
register (RTC_CALR). In this case, the calibration mechanism is applied on ck_apre instead
of RTCCLK. The resulting accuracy is the same, but the calibration is performed during a
calibration cycle of about 220 x PREDIV_A x RTCCLK pulses instead of 220 RTCCLK pulses
when LPCAL=0.

Smooth calibration mechanism
The smooth calibration register (RTC_CALR) specifies the number of ck_cal clock cycles to
be masked during the calibration cycle:

Note:

•

Setting the bit CALM[0] to 1 causes exactly one pulse to be masked during the
calibration cycle.

•

Setting CALM[1] to 1 causes two additional cycles to be masked

•

Setting CALM[2] to 1 causes four additional cycles to be masked

•

and so on up to CALM[8] set to 1 which causes 256 clocks to be masked.

CALM[8:0] (RTC_CALR) specifies the number of ck_cal pulses to be masked during the
calibration cycle. Setting the bit CALM[0] to 1 causes exactly one pulse to be masked during
the calibration cycle at the moment when cal_cnt[19:0] is 0x80000; CALM[1] = 1 causes two
other cycles to be masked (when cal_cnt is 0x40000 and 0xC0000); CALM[2] = 1 causes
four other cycles to be masked (cal_cnt = 0x20000/0x60000/0xA0000/ 0xE0000); and so on
up to CALM[8] = 1 which causes 256 clocks to be masked (cal_cnt = 0xXX800).
While CALM permits the RTC frequency to be reduced by up to 487.1 ppm with fine
resolution, the bit CALP can be used to increase the frequency by 488.5 ppm. Setting CALP

RM0456 Rev 6

<!-- pagebreak -->

