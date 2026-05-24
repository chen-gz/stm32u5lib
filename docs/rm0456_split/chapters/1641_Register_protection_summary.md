RM0456 Rev 6

MSv63667V1

RM0456

40.4.12

Audio digital filter (ADF)

Autonomous mode
The ADF can work even if the AHB bus clock is not available (Stop modes). The ADF uses
the AHB clock only for the register interface. All the processing part is clocked with the
kernel clock.
In Stop mode, the ADF receives a kernel clock if the following conditions are met:
•

The ADF autonomous mode is enabled in the RCC.

•

The selected kernel clock source is taken from an oscillator available in Stop mode.

In Stop mode, the ADF receives the AHB clock if the following conditions are met:
•

The ADF autonomous mode is enabled in the RCC.

•

The ADF requests the AHB clock in the following situations:
–

when the ADF must transfer data into memory via the DMA
The data is directly transferred to the SRAM thanks to the DMA while the product
remains in Stop mode. The AHB clock request is maintained until the DMA
transfer is completed.

–

when the ADF needs to generate an interrupt
An interrupt generally wakes up the device from Stop mode, as an action from the
application is needed. Once the AHB clock is available, the interrupt is
generated.The AHB clock request is maintained as long as an enabled interrupt
flag is still active.

40.4.13

Register protection
The ADF embeds some hardware protection to prevent invalid situations.
Table 402 shows the list of write-protected and unprotected fields.
Table 402. Register protection summary
Unprotected
fields

Write-protected fields

Write-protection
condition

ADF global control register (ADF_GCR)

TRGO

-

DFLTACTIVE0 = 1

ADF clock generator control register
(ADF_CKGCR)

CKGDEN
CCK0EN
CCK1EN

PROCDIV[6:0], CCKDIV[3:0],
CKGMOD, TRGSRC[3:0],
TRGSENS, CCK[1:0]DIR

CKGACTIVE = 1

ADF serial interface control register 0
(ADF_SITF0CR)

SITFEN

STH[4:0], SITFMOD[1:0],
SCKSRC[1:0]

SITFACTIVEx = 1

ADF bitstream matrix control register 0
(ADF_BSMX0CR)

-

BSSEL[4:0]

DFLTACTIVEx = 1

DFLTEN

NBDIS[7:0], TRGSRC[3:0],
TRGSENS, FTH, DMAEN,
SNPSFMT, ACQMOD[2:0]

ADF digital filer configuration register 0
(ADF_DFLT0CICR)

SCALE[5:0]

MCICD[8:0], CICMOD[2:0],
DATSRC[1:0]

ADF reshape filter configuration register 0
(ADF_DFLT0RSFR)

-

All fields

ADF delay control register 0 (ADF_DLY0CR)

-

SKPDLY[6:0]

Registers

ADF digital filter control register 0
(ADF_DFLT0CR)

RM0456 Rev 6

DFLTACTIVEx = 1

SKPBF = 1

<!-- pagebreak -->

