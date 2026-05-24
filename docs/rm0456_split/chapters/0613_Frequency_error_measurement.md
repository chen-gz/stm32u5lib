RM0456 Rev 6

RM0456

12.4.4

Clock recovery system (CRS)

Frequency error measurement
The frequency error counter is a 16-bit down/up counter, reloaded with the RELOAD value
on each SYNC event. It starts counting down until it reaches the zero value, where the
ESYNC (expected synchronization) event is generated. Then it starts counting up to the
OUTRANGE limit, where it eventually stops (if no SYNC event is received), and generates a
SYNCMISS event. The OUTRANGE limit is defined as the frequency error limit (FELIM field
of the CRS_CFGR register) multiplied by 128.
When the SYNC event is detected, the actual value of the frequency error counter and its
counting direction are stored in the FECAP (frequency error capture) field and in the FEDIR
(frequency error direction) bit of the CRS_ISR register. When the SYNC event is detected
during the down-counting phase (before reaching the zero value), it means that the actual
frequency is lower than the target (the TRIM value must be incremented). When it is
detected during the up-counting phase, it means that the actual frequency is higher (the
TRIM value must be decremented).
Figure 44. CRS counter behavior

CRS counter value
RELOAD

ESYNC
Down

Up

Frequency
error counter
stopped

OUTRANGE
(128 x FELIM)

WARNING LIMIT
(3 x FELIM)
TOLERANCE LIMIT
(FELIM)
Trimming action:
CRS event:

0

+2

SYNCERR

SYNCWARN

+1

0

-2

-1

SYNCOK

0

SYNCWARN
SYNCMISS
MSv32122V1

12.4.5

Frequency error evaluation and automatic trimming
The measured frequency error is evaluated by comparing its value with a set of limits:
•

TOLERANCE LIMIT, given directly in the FELIM field of the CRS_CFGR register

•

WARNING LIMIT, defined as 3 × FELIM value

•

OUTRANGE (error limit), defined as 128 × FELIM value

RM0456 Rev 6

<!-- pagebreak -->

