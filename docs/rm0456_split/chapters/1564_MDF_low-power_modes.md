1599

Multi-function digital filter (MDF)

RM0456

Table 382. Register protection summary (continued)
Registers

Unprotected
fields

Write-protected fields

Write-protection
condition

MDF DFLTx interrupt status register x
(MDF_DFLTxISR)

All fields

-

-

MDF offset error compensation control register
x (MDF_OECxCR)

OFFSET[25:0]

-

-

All the MDF processing is performed in the mdf_proc_ck clock domain. For that reason,
enabling or disabling a MDF sub-block may take some time due to the re-synchronization
between the AHB clock domain and the mdf_proc_ck clock domain. XXXACTIVE flags are
available to allow the application to check that the synchronization between the two clock
domains is completed.
To change a write-protected bitfield, the application must follow this sequence:
1.

Set the enable bit of the sub-block to 0.

2.

Wait for corresponding flag XXXACTIVE = 0.

3.

Modify the wanted fields.

4.

Set the enable bit of the sub-block to 1.

Refer to the description of each sub-block for more details.

39.5

MDF low-power modes
Table 383. Effect of low-power modes on MDF

Mode

Description

Sleep

No effect. MDF interrupts cause the device to exit Sleep mode.

Stop(1)

The MDF registers content is kept.
If the MDF is clocked by an internal oscillator available in Stop mode, the MDF remains active. The
interrupts cause the device to exit Stop mode.

Standby The MDF is powered down and must be reinitialized after exiting Standby mode.
1. Refer to Section 39.3: MDF implementation for details about Stop modes supported by the MDF.

39.6

MDF interrupts
To increase the CPU performance, the MDF offers the following interrupt lines per digital
filter:

Note:

<!-- pagebreak -->

•

receive interrupt MDF_FLTx_RX (mdf_fltx_rx_it)

•

event interrupt MDF_FLTx_EVT (mdf_fltx_evt_it)

•

a combination of both interrupts MDF_FLTx (mdf_fltx_it)

Interrupts are not always connected to the device (see Section 39.3: MDF implementation
for more details).
The status flags are available even if the corresponding interrupt enable flag is not enabled.

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
The interrupt interface is controlled via the MDF DFLTx interrupt enable register x
(MDF_DFLTxIER) and the MDF DFLTx interrupt status register x (MDF_DFLTxISR).
Figure 356. MDF interrupt interface

FTHIE

clear
snps_evtx

dovr_evtx

MDF

FTHF

fth_evtx

Edge
detector
clear

SSDRF

OR

mdf_fltx_rx_it (1)

SSDRIE

Edge
detector

DOVRF
DOVRIE

clear
snpsovr_evtx

Edge
detector

SSOVRF
SSOVRIE

clear
rfovr_evtx

Edge
detector

mdf_fltx_it

RFOVRF
RFOVRIE

clear

old_thlx

scd_evtx

sat_evtx

ckab_evtx

OLDIE

Edge
detector

THHF

OLDF

Edge
detector
clear
Edge
detector
clear
Edge
detector
clear

OR

THLF
mdf_fltx_evt_it (1)
SCDF

read

old_thhx

Edge
detector

write

old_evtx

Edge
detector

MDF_DFLTxISR

SCDIE
SATF
SATIE
CKABF
CKABIE
MDF_DFLTxIER

(1) Not always implemented. Refer to the vector table of the product for details.

RM0456 Rev 6

MSv63600V2

<!-- pagebreak -->

