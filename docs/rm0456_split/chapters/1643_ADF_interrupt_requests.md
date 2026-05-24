The status flags are available even if the corresponding interrupt enable flag is not enabled.

RM0456 Rev 6

RM0456

Audio digital filter (ADF)
The interrupt interface is controlled via the ADF DFLT0 interrupt enable register
(ADF_DFLT0IER) and the ADF DFLT0 interrupt status register 0 (ADF_DFLT0ISR).
Figure 385. ADF interrupt interface

ADF

FTHF

fth_evt0

FTHIE
clear
dovr_evt0

Edge
detector

DOVRF
DOVRIE

clear
Edge
detector

RFOVRF

clear
sat_evt0

ckab_evt0

Edge
detector
clear

read

RFOVRIE

write

rfovr_evt0

Edge
detector

SATF
SATIE

OR

adf_flt0_it

CKABF
CKABIE
(1)

clear
sddet_evt

sdlvl_evt

Edge
detector
clear
Edge
detector

SDDETF
SDDETIE
SDLVLF
SDLVLIE

ADF_DFLT0ISR

ADF_DFLT0IER

(1) Only present if the SAD is implemented, refer to section ADF implementation for details.

MSv63668V2

Table 404 shows which interrupt line is affected by which event, and how to clear and
activate each interrupt/event.
Table 404. ADF interrupt requests

ADF_FLT0(2)

Interrupt
vector

Event flag

Event/interrupt
clearing method

RXFIFO threshold
reached

FTHF

Read ADF_DFLT0DR
until RXFIFO level is
lower than the
threshold.

RXFIFO overrun

DOVRF

Write DOVRF to 1.

Interrupt event

RSFLT overrun

RFOVRF

Write RFOVRF to 1.

Saturation detection

SATF

Write SATF to 1.

Channel clock
absence detection

CKABF

Write CKABF to 1.

SAD: sound detected

SDDETF

Write SDDETF to 1.

SAD: sound level
value available

SDLVLF

Write SDLVLF to 1.

RM0456 Rev 6

Exit Sleep
mode

Exit Stop
modes(1)

Exit Standby
mode

Yes

Yes

No

<!-- pagebreak -->

