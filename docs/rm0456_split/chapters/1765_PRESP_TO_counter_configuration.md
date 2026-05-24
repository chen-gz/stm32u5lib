RM0456 Rev 6

RM0456

DSI Host (DSI)
Figure 425. Timing of PRESP_TO after a write request (HS or LP)

Host

Device

WRITE

Reque

st

PRESP_TO

Timer < PRESP_TO

LP-11

Arbitra

Device Ready

ry even

t after W

RITE R

eq.

MSv35868V1

Table 441 describes the fields used for the configuration of the PRESP_TO counter.
Table 441. PRESP_TO counter configuration
Description

Period for which the DSI Host
keeps the link still

Period for which the DSI Host
keeps the link inactive

Register

Field

After sending a
High-speed read operation

DSI_TCCR1

HSRD_TOCNT

After sending a
Low-power read operation

DSI_TCCR2

LPRD_TOCNT

After completing a
Bus-turn-around (BTA)

DSI_TCCR5

BTA_TOCNT

After sending a
High-speed write operation

DSI_TCCR3

HSWR_TOCNT

After sending a
Low-power write operation

DSI_TCCR4

LPWR_TOCNT

The values in these registers are measured in number of cycles of the lane byte clock.
These registers are only used in command mode because in video mode, there is a rigid
timing schedule to be met to keep the display properly refreshed and it must not be broken
by these or any other timeouts. Setting a given timeout to 0 disables going into LP-11 state
and timeout for events of that category.
The read and the write requests in high-speed mode are distinct from those in low-power
mode. For example, if HSRD_TOCNT is set to 0 and LPRD_TOCNT is set to a non-0 value,
a generic read with no parameters does not activate the PRESP_TO counter in high-speed,
but activates the PRESP_TO in low-power.
The DSI Host timeout counter configuration register 4 (DSI_TCCR3) includes a special
Presp mode (PM) bit to change the normal behavior of PRESP_TO in Adaptive command

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

mode for high-speed write operation timeout. When set to 1, this bit allows the PRESP_TO
from HSWR_TOCNT to be used only once, when both of the following conditions are met:
•

the LTDC VSYNC signal rises and falls

•

the packets originated from the LTDC interface in adapted command mode are
transmitted and its FIFO is empty again.

In this scenario, non-adapted command mode requests are not sent to the D-PHY, even if
there is traffic from the generic interface ready to be sent, returning them to the Stop state.
When it happens, the PRESP_TO counter is activated and only when it is completed, the
DSI Host sends any other traffic that is ready, as illustrated in Figure 426.
Figure 426. Effect of prep mode at 1
dpivsync_edpiwms
dpidataen
dpidata[29:0]

A10

A20

A30

edpi_fifo_empty
gen_wr_en
gen_data[31:0]
link_state[1:0]

B3

LP

HS

LP

HS

LP

link_data[31:0]
PRESP_TO_active
MSv35880V1

<!-- pagebreak -->

