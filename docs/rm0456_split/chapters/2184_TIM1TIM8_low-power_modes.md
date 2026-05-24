2236

Advanced-control timers (TIM1/TIM8)

RM0456

For safety purposes, when the counter is stopped, the outputs are disabled (as if the MOE
bit was reset). The outputs can either be forced to an inactive state (OSSI bit = 1), or have
their control taken over by the GPIO controller (OSSI bit = 0), typically to force a Hi-Z.
For more details, refer to section Debug support (DBG).

54.4

TIM1/TIM8 low-power modes
Table 552. Effect of low-power modes on TIM1/TIM8
Mode

54.5

Description

Sleep

No effect, peripheral is active. The interrupts can cause the device to exit from Sleep
mode.

Stop

The timer operation is stopped and the register content is kept. No interrupt can be
generated.

Standby

The timer is powered-down and must be reinitialized after exiting the Standby mode.

TIM1/TIM8 interrupts
The TIM1/TIM8 can generate multiple interrupts, as shown in Table 553.
Table 553. Interrupt requests

Event
flag

Enable
control
bit

Interrupt clear
method

Exit
Exit
from
from
Stop
Sleep
and
mode Standby
mode

Update

UIF

UIE

write 0 in UIF

Yes

No

Capture/compare 1

CC1IF

CC1IE

write 0 in CC1IF

Yes

No

Capture/compare 2

CC2IF

CC2IE

write 0 in CC2IF

Yes

No

Capture/compare 3

CC3IF

CC3IE

write 0 in CC3IF

Yes

No

Capture/compare 4

CC4IF

CC4IE

write 0 in CC4IF

Yes

No

TIM_COM

Commutation (COM) COMIF

COMIE

write 0 in COMIF

Yes

No

TIM_TRGI

Trigger

TIF

TIE

write 0 in TIF

Yes

No

TIM_IDX

Index

IDXF

IDXIE

write 0 in IDXF

Yes

No

TIM_DIR

Direction

DIRF

DIRIE

write 0 in DIRF

Yes

No

Break

BIF

write 0 in BIF

Yes

No

Break2

B2IF

write 0 in B2IF

Yes

No

System Break

SBIF

write 0 in SBIF

Yes

No

Index Error

IERRF

write 0 in IERRF

Yes

No

TERRF TERRIE write 0 in TERRF

Yes

No

Interrupt acronym

TIM_UPD

TIM_CC

TIM_TRGI_COM_
DIR_IDX

TIM_BRK
TIM_BRK_TERR_
IERR
TIM_IERR

Interrupt event

TIM_TERR Transition Error

<!-- pagebreak -->

