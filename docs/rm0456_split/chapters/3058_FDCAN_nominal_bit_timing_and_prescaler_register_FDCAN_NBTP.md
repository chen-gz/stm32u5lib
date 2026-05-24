RM0456 Rev 6

RM0456

USB on-the-go full-speed (OTG_FS)
Table 754. Host-mode control and status registers (CSRs) (continued)

Acronym

Offset
address

Register name

OTG_HPRT

0x440

Section 72.15.27: OTG host port control and status register (OTG_HPRT)

OTG_HCCHARx

0x500
0x520
...
0x660

Section 72.15.28: OTG host channel x characteristics register
(OTG_HCCHARx)

OTG_HCINTx

0x508
0x528
....
0x668

Section 72.15.29: OTG host channel x interrupt register (OTG_HCINTx)

OTG_HCINTMSKx

0x50C
0x52C
....
0x66C

Section 72.15.30: OTG host channel x interrupt mask register
(OTG_HCINTMSKx)

OTG_HCTSIZx

0x510
0x530
....
0x670

Section 72.15.31: OTG host channel x transfer size register
(OTG_HCTSIZx)

Device-mode CSR map
These registers must be programmed every time the core changes to device mode.
Table 755. Device-mode control and status registers
Acronym

Offset
address

Register name

OTG_DCFG

0x800

Section 72.15.33: OTG device configuration register (OTG_DCFG)

OTG_DCTL

0x804

Section 72.15.34: OTG device control register (OTG_DCTL)

OTG_DSTS

0x808

Section 72.15.35: OTG device status register (OTG_DSTS)

OTG_DIEPMSK

0x810

Section 72.15.36: OTG device IN endpoint common interrupt mask
register (OTG_DIEPMSK)

OTG_DOEPMSK

0x814

Section 72.15.37: OTG device OUT endpoint common interrupt mask
register (OTG_DOEPMSK)

OTG_DAINT

0x818

Section 72.15.38: OTG device all endpoints interrupt register
(OTG_DAINT)

OTG_DAINTMSK

0x81C

Section 72.15.39: OTG all endpoints interrupt mask register
(OTG_DAINTMSK)

OTG_DVBUSDIS

0x828

Section 72.15.40: OTG device VBUS discharge time register
(OTG_DVBUSDIS)

OTG_DVBUSPULSE

0x82C

Section 72.15.41: OTG device VBUS pulsing time register
(OTG_DVBUSPULSE)

RM0456 Rev 6

<!-- pagebreak -->

3291

USB on-the-go full-speed (OTG_FS)

RM0456

Table 755. Device-mode control and status registers (continued)
Acronym

Offset
address

Register name

OTG_DIEPEMPMSK

0x834

Section 72.15.42: OTG device IN endpoint FIFO empty interrupt mask
register (OTG_DIEPEMPMSK)

OTG_DIEPCTL0

0x900

Section 72.15.43: OTG device control IN endpoint 0 control register
(OTG_DIEPCTL0)

OTG_DIEPCTLx

0x920
0x940
...
0x9A0

Section 72.15.44: OTG device IN endpoint x control register
(OTG_DIEPCTLx)

OTG_DIEPINTx

0x908
0x928
....
0x988

Section 72.15.45: OTG device IN endpoint x interrupt register
(OTG_DIEPINTx)

OTG_DIEPTSIZ0

0x910

Section 72.15.46: OTG device IN endpoint 0 transfer size register
(OTG_DIEPTSIZ0)

OTG_DTXFSTSx

0x918
0x938
....
0x998

Section 72.15.47: OTG device IN endpoint transmit FIFO status register
(OTG_DTXFSTSx)

OTG_DIEPTSIZx

0x930
0x950
...
0x9B0

Section 72.15.48: OTG device IN endpoint x transfer size register
(OTG_DIEPTSIZx)

OTG_DOEPCTL0

0xB00

Section 72.15.49: OTG device control OUT endpoint 0 control register
(OTG_DOEPCTL0)

OTG_DOEPINTx

0xB08
0xB28
...
0xBA8

Section 72.15.50: OTG device OUT endpoint x interrupt register
(OTG_DOEPINTx)

OTG_DOEPTSIZ0

0xB10

Section 72.15.51: OTG device OUT endpoint 0 transfer size register
(OTG_DOEPTSIZ0)

OTG_DOEPCTLx

0xB20
0xB40
...
0xBA0

Section 72.15.52: OTG device OUT endpoint x control register
(OTG_DOEPCTLx)

OTG_DOEPTSIZx

0xB30
0xB50
...
0xBB0

Section 72.15.53: OTG device OUT endpoint x transfer size register
(OTG_DOEPTSIZx)

<!-- pagebreak -->

