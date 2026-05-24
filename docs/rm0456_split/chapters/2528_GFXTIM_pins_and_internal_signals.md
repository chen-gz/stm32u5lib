RM0456 Rev 6

RM0456

Real-time clock (RTC)

Bit 4 TSOVF: Timestamp overflow flag
This flag is set by hardware when a timestamp event occurs while TSF is already set.
It is recommended to check and then clear TSOVF only after clearing the TSF bit. Otherwise,
an overflow might not be noticed if a timestamp event occurs immediately before the TSF bit
is cleared.
Bit 3 TSF: Timestamp flag
This flag is set by hardware when a timestamp event occurs.
If ITSF flag is set, TSF must be cleared together with ITSF.
Note: TSF is not set if TAMPTS = 1 and the tamper flag is read during the 3 ck_apre cycles
following tamper event. Refer to Timestamp on tamper event for more details.
Bit 2 WUTF: Wake-up timer flag
This flag is set by hardware when the wake-up auto-reload counter reaches 0.
If WUTOCLR[15:0] is different from 0x0000, WUTF is cleared by hardware when the wakeup auto-reload counter reaches WUTOCLR value.
If WUTOCLR[15:0] is 0x0000, WUTF must be cleared by software.
This flag must be cleared by software at least 1.5 RTCCLK periods before WUTF is set to 1
again.
Bit 1 ALRBF: Alarm B flag
This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the
alarm B register (RTC_ALRMBR).
Bit 0 ALRAF: Alarm A flag
This flag is set by hardware when the time/date registers (RTC_TR and RTC_DR) match the
alarm A register (RTC_ALRMAR).

Note:

The bits of this register are cleared few APB clock cycles after setting their corresponding
clear bit in the RTC_SCR register. After clearing the flag, read it until it is read at 0 before
leaving the interrupt routine.

63.6.21

RTC nonsecure masked interrupt status register (RTC_MISR)
This register can be globally protected, or each bit of this register can be individually
protected against unprivileged access. Refer to Section 63.3.5: RTC privilege protection
modes.
Address offset: 0x54
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

Res.

Res.

Res.

Res.

Res.

Res.

SSR
UMF

ITS
MF

TSOV
MF

TS
MF

WUT
MF

ALRB
MF

ALRA
MF

r

r

r

r

r

r

r

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 SSRUMF: SSR underflow nonsecure masked flag
This flag is set by hardware when the SSR underflow nonsecure interrupt occurs.

RM0456 Rev 6

<!-- pagebreak -->

