621

Clock recovery system (CRS)

RM0456

Bits 26:24 SYNCDIV[2:0]: SYNC divider
These bits are set and cleared by software to control the division factor of the SYNC signal.
000: SYNC not divided (default)
001: SYNC divided by 2
010: SYNC divided by 4
011: SYNC divided by 8
100: SYNC divided by 16
101: SYNC divided by 32
110: SYNC divided by 64
111: SYNC divided by 128
Bits 23:16 FELIM[7:0]: Frequency error limit
FELIM contains the value to be used to evaluate the captured frequency error value latched
in the FECAP[15:0] bits of the CRS_ISR register. Refer to Section 12.4.5 for more details
about FECAP evaluation.
Bits 15:0 RELOAD[15:0]: Counter reload value
RELOAD is the value to be loaded in the frequency error counter with each SYNC event.
Refer to Section 12.4.4 for more details about counter behavior.

12.7.3

CRS interrupt and status register (CRS_ISR)
Address offset: 0x08
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

FECAP[15:0]
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

FEDIR

Res.

Res.

Res.

Res.

TRIM
OVF

SYNC
MISS

SYNC
ERR

Res.

Res.

Res.

Res.

ESYNCF

ERRF

SYNC
WARNF

SYNC
OKF

r

r

r

r

r

r

r

r

Bits 31:16 FECAP[15:0]: Frequency error capture
FECAP is the frequency error counter value latched in the time of the last SYNC event.
Refer to Section 12.4.5 for more details about FECAP usage.
Bit 15 FEDIR: Frequency error direction
FEDIR is the counting direction of the frequency error counter latched in the time of the last
SYNC event. It shows whether the actual frequency is below or above the target.
0: Up-counting direction, the actual frequency is above the target
1: Down-counting direction, the actual frequency is below the target
Bits 14:11 Reserved, must be kept at reset value.
Bit 10 TRIMOVF: Trimming overflow or underflow
This flag is set by hardware when the automatic trimming tries to over- or under-flow the
TRIM value. An interrupt is generated if the ERRIE bit is set in the CRS_CR register. It is
cleared by software by setting the ERRC bit in the CRS_ICR register.
0: No trimming error signaled
1: Trimming error signaled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Clock recovery system (CRS)

Bit 9 SYNCMISS: SYNC missed
This flag is set by hardware when the frequency error counter reaches value FELIM * 128
and no SYNC is detected, meaning either that a SYNC pulse was missed, or the frequency
error is too big (internal frequency too high) to be compensated by adjusting the TRIM value,
hence some other action must be taken. At this point, the frequency error counter is stopped
(waiting for a next SYNC), and an interrupt is generated if the ERRIE bit is set in the
CRS_CR register. It is cleared by software by setting the ERRC bit in the CRS_ICR register.
0: No SYNC missed error signaled
1: SYNC missed error signaled
Bit 8 SYNCERR: SYNC error
This flag is set by hardware when the SYNC pulse arrives before the ESYNC event and the
measured frequency error is greater than or equal to FELIM * 128. This means that the
frequency error is too big (internal frequency too low) to be compensated by adjusting the
TRIM value, and that some other action has to be taken. An interrupt is generated if the
ERRIE bit is set in the CRS_CR register. It is cleared by software by setting the ERRC bit in
the CRS_ICR register.
0: No SYNC error signaled
1: SYNC error signaled
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 ESYNCF: Expected SYNC flag
This flag is set by hardware when the frequency error counter reached a zero value. An
interrupt is generated if the ESYNCIE bit is set in the CRS_CR register. It is cleared by
software by setting the ESYNCC bit in the CRS_ICR register.
0: No expected SYNC signaled
1: Expected SYNC signaled
Bit 2 ERRF: Error flag
This flag is set by hardware in case of any synchronization or trimming error. It is the logical
OR of the TRIMOVF, SYNCMISS and SYNCERR bits. An interrupt is generated if the ERRIE
bit is set in the CRS_CR register. It is cleared by software in reaction to setting the ERRC bit
in the CRS_ICR register, which clears the TRIMOVF, SYNCMISS and SYNCERR bits.
0: No synchronization or trimming error signaled
1: Synchronization or trimming error signaled
Bit 1 SYNCWARNF: SYNC warning flag
This flag is set by hardware when the measured frequency error is greater than or equal to
FELIM * 3, but smaller than FELIM * 128. This means that to compensate the frequency
error, the TRIM value must be adjusted by two steps or more. An interrupt is generated if the
SYNCWARNIE bit is set in the CRS_CR register. It is cleared by software by setting the
SYNCWARNC bit in the CRS_ICR register.
0: No SYNC warning signaled
1: SYNC warning signaled
Bit 0 SYNCOKF: SYNC event OK flag
This flag is set by hardware when the measured frequency error is smaller than FELIM * 3.
This means that either no adjustment of the TRIM value is needed or that an adjustment by
one trimming step is enough to compensate the frequency error. An interrupt is generated if
the SYNCOKIE bit is set in the CRS_CR register. It is cleared by software by setting the
SYNCOKC bit in the CRS_ICR register.
0: No SYNC event OK signaled
1: SYNC event OK signaled

RM0456 Rev 6

<!-- pagebreak -->

