3

13
Res.
Res.

0

1

Res.

14
Res.
Res.
Res.

4

15
Res.
Res.
Res.

1

Res.

16
Res.
Res.
Res.

0

5

17
Res.
Res.
Res.

0

1

Res.

18
Res.
Res.
Res.

Res.

0

6

19
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

WWDG_SR

Res.

0x008

Res.

Reset value

WDGTB
[2:0]

1

Res.

20
Res.

7

21
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

WWDG_CFR

Res.

0x004

0
Res.

Reset value

T[6:0]

Res.

22
Res.

8

23
Res.

WDGA

24
Res.

9

25
Res.

Res.

26
Res.

Res.

27
Res.

11

28
Res.

10

29
Res.

Res.

30

WWDG_CR

Res.

31

0x000

Register name

Res.

Offset

Res.

Table 631. WWDG register map and reset values

RM0456 Rev 6

RM0456

Real-time clock (RTC)

63

Real-time clock (RTC)

63.1

Introduction
The RTC provides an automatic wake-up to manage all low-power modes.
The real-time clock (RTC) is an independent BCD timer/counter. The RTC provides a timeof-day clock/calendar with programmable alarm interrupts.
As long as the supply voltage remains in the operating range, the RTC never stops,
regardless of the device status (Run mode, low-power mode or under reset).
The RTC is functional in VBAT mode.

63.2

RTC main features
The RTC supports the following features (see Figure 773: RTC block diagram):
•

Calendar with subseconds, seconds, minutes, hours (12 or 24 format), week day, date,
month, year, in BCD (binary-coded decimal) format.

•

Binary mode with 32-bit free-running counter.

•

Automatic correction for 28, 29 (leap year), 30, and 31 days of the month.

•

Two programmable alarms.

•

On-the-fly correction from 1 to 32767 RTC clock pulses. This can be used to
synchronize it with a master clock.

•

Reference clock detection: a more precise second source clock (50 or 60 Hz) can be
used to enhance the calendar precision.

•

Digital calibration circuit with 0.95 ppm resolution, to compensate for quartz crystal
inaccuracy.

•

Timestamp feature which can be used to save the calendar content. This function can
be triggered by an event on the timestamp pin, or by a tamper event, or by a switch to
VBAT mode.

•

17-bit auto-reload wake-up timer (WUT) for periodic events with programmable
resolution and period.

•

TrustZone support:

•

–

RTC fully securable

–

Alarm A, alarm B, wake-up Timer and timestamp individual secure or nonsecure
configuration

Alarm A, alarm B, wake-up Timer and timestamp individual privilege protection

The RTC is supplied through a switch that takes power either from the VDD supply when
present or from the VBAT pin.
The RTC is functional in VBAT mode and in all low-power modes when it is clocked by the
LSE.
All RTC events (Alarm, wake-up Timer, Timestamp) can generate an interrupt and wake-up
the device from the low-power modes.

RM0456 Rev 6

<!-- pagebreak -->

