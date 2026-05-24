2562

Graphic timer (GFXTIM)

59.3.9

RM0456

Watchdog timer
The watchdog timer is a 16-bit auto-reload down-counter with a programmable clock
source.
Figure 764. Watchdog timer
Reload
Line clock
Frame clock
HSYNC
VSYNC
TE
EV1
EV2
EV3
EV4

Watchdog
Auto-reload
16-bit counter

ALRM

Pre-alarm

PALRM
MSv66922V1

Clock source
The watchdog clock source can be selected through WDGCS (clock source) in
GTXTIM_WDGTCR (watchdog timer configuration register), between one of the following:
•

line clock

•

frame clock

•

HSYNC

•

VSYNC

•

TE

•

event 1

•

event 2

•

event 3

•

event 4

Startup
The watchdog is started by setting WDGEN in GFXTIM_WDGTCR and stops automatically
when reaching 0.
On start, the watchdog counter is automatically loaded with the auto-reload value
programmed in GFXTIM_WDGRR (watchdog reload register).
The current watchdog value can be read through GFXTIM_WDGCR.

Auto-reload
The auto-reload can be forced in one of the following ways:
•

by software, setting FWDGR (force watchdog reload) in GTXTIM_WDGTCR (watchdog
timer configuration register)

•

by hardware through an external trigger (gfxtim_wrld signal)
The polarity of the trigger is configured by WDGHRC (watchdog hardware reload
configuration) in GFXTIM_WDGTCR.

<!-- pagebreak -->

