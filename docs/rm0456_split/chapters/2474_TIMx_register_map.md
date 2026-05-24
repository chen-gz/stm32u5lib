WWDG

RM0456 Rev 6

RM0456

System window watchdog (WWDG)

4. Controlled via the option byte WWDG_SW. When WWDG_SW is set in HW mode the WWDG is running as
soon as the CPU is in Run or Sleep modes.

62.4

WWDG functional description
If the watchdog is activated (the WDGA bit is set in the WWDG_CR register), and when the
7-bit down-counter (T[6:0] bits) is decremented from 0x40 to 0x3F (T6 becomes cleared), it
initiates a reset. If the software reloads the counter while the counter is greater than the
value stored in the window register, then a reset is generated.
The application program must write in the WWDG_CR register at regular intervals during
normal operation to prevent a reset. This operation can take place only when the counter
value is lower than or equal to the window register value, and higher than 0x3F. The value to
be stored in the WWDG_CR register must be between 0xFF and 0xC0.
Refer to Figure 771 for the WWDG block diagram.

62.4.1

WWDG block diagram
Figure 771. Watchdog block diagram

APB bus

W[6:0]
CMP

WWDG_CFR

WWDG

CMP = 1 when
T[6:0] > W[6:0]

Register interface

WWDG_SR

wwdg_out_rst

Write to
WWDG_CR

preload

= 0x40 ?

T[6:0]

WWDG_CR

T6

cnt_out

EWI
EWIF

Logic

readback

WDGA

wwdg_it

7-bit down-counter (CNT)
pclk

÷ 4096

÷ 2WDGTB
MS47214V2

62.4.2

WWDG internal signals
Table 629 gives the list of WWDG internal signals.
Table 629. WWDG internal input/output signals
Signal name

Signal type

Description

pclk

Digital input

APB bus clock

wwdg_out_rst

Digital output

WWDG reset signal output

wwdg_it

Digital output

WWDG early interrupt output

RM0456 Rev 6

<!-- pagebreak -->

