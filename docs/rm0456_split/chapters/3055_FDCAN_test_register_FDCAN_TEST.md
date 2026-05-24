3291

USB on-the-go full-speed (OTG_FS)

RM0456

Figure 901. Interrupt hierarchy
Wakeup interrupt
OTG_FS_WKUP

(1)

AND

Global interrupt
OTG_FS

OR

NT
GI

AND

OT

HC

INT
HP
RT
IN

OE
PIN
IEP P
INT

T

Global interrupt mask (bit 0)
OTG_AHBCFG
AHB configuration register

OTG_GINTSTS
Core register interrupt
31:26

25 24

23:20

19 18

17:3

2

1:0

OTG_GINTMSK
Core interrupt mask register

OTG_GOTGINT
OTG interrupt register

(15 + #EP):16
OUT endpoints

(#EP-1):0
IN endpoints

OTG_DAINTMSK
Device all endpoints interrupt mask
register
OTG_DAINT
Device all endpoints interrupt register

OTG_DIEPMSK/
OTG_DOEPMSK
Device IN/OUT endpoints common
interrupt mask register

x=0
...

OTG_DIEPINTx/
OTG_DOEPINTx
Device IN/OUT endpoint interrupt
registers

x = #EP-1

OTG_HPRT
Host port control and status register

OTG_HAINTMSK
Host all channels interrupt mask register
OTG_HAINT
Host all channels interrupt register
x=0
...

OTG_HCTINTMSKx
Host channels interrupt mask registers

x = #HC-1

OTG_HCTINTx
Host channels interrupt registers

MSv36921V4

1. OTG_FS_WKUP becomes active (high state) when resume condition occurs during L1 SLEEP or L2 SUSPEND states.

<!-- pagebreak -->

