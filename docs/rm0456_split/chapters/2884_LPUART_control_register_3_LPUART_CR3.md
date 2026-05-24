The LFSDET flag is not asserted in AC’97 mode since the SAI audio block acts as a link
controller and generates the FS signal even when declared as slave.It has no meaning in
SPDIF mode since the signal FS is not used by the protocol.

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

Codec not ready (CNRDY AC’97)
The CNRDY flag in the SAI_xSR register is relevant only if the SAI audio block is configured
to operate in AC’97 mode (PRTCFG[1:0] = 10 in the SAI_xCR1 register). If the CNRDYIE bit
is set in the SAI_xIM register, an interrupt is generated when the CNRDY flag is set.
CNRDY is asserted when the codec is not ready to communicate during the reception of the
TAG 0 (slot 0) of the AC’97 audio frame. In this case, no data are automatically stored into
the FIFO since the codec is not ready, until the TAG 0 indicates that the codec is ready. All
the active slots defined in the SAI_xSLOTR register are captured when the codec is ready.
To clear the CNRDY flag, the CCNRDY bit must be set in the SAI_xCLRFR register.

Wrong clock configuration in master mode (with NODIV = 0)
When the audio block operates as a master (MODE[1] = 0) and the NODIV bit is equal to 0,
the WCKCFG flag is set as soon as the SAI is enabled if the following conditions are met:
•

(FRL+1) is not a power of 2, and

•

(FRL+1) is not between 8 and 256.

The MODE, NODIV, and SAIEN bits belong to the SAI_xCR1 register and FRL to the
SAI_xFRCR register.
If the WCKCFGIE bit is set, an interrupt is generated when the WCKCFG flag is set in the
SAI_xSR register. To clear this flag, set the CWCKCFG bit in the SAI_xCLRFR register.
When the WCKCFG bit is set, the audio block is automatically disabled, thus performing a
hardware clear of the SAIEN bit.

69.4.15

Disabling the SAI
The SAI audio block can be disabled at any moment by clearing the SAIEN bit in the
SAI_xCR1 register. All the already started frames are automatically completed before the
SAI stops working. The SAIEN bit remains high until the SAI is completely switched off at
the end of the current audio frame transfer.
If an audio block in the SAI operates synchronously with the other one, the one that is the
master must be disabled first.

69.4.16

SAI DMA interface
To free the CPU and to optimize bus bandwidth, each SAI audio block has an independent
DMA interface to read/write from/to the SAI_xDR register (to access the internal FIFO).
There is one DMA channel per audio subblock supporting the basic DMA
request/acknowledge protocol.
To configure the audio subblock for DMA transfer, set the DMAEN bit in the SAI_xCR1
register. The DMA request is managed directly by the FIFO controller depending on the
FIFO threshold level (for more details refer to Section 69.4.9: Internal FIFOs). The DMA
transfer direction is linked to the SAI audio subblock configuration:
•

If the audio block operates as a transmitter, the audio block FIFO controller outputs a
DMA request to load the FIFO with data written in the SAI_xDR register.

•

If the audio block operates as a receiver, the DMA request is related to read operations
from the SAI_xDR register.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Follow the sequence below to configure the SAI interface in DMA mode:

69.5

1.

Configure the SAI and FIFO threshold levels to specify when the DMA request is
launched.

2.

Configure the SAI DMA channel.

3.

Enable the DMA.

4.

Enable the SAI interface.

SAI interrupts
The SAI supports 7 interrupt sources, as shown in Table 715.
Table 715. SAI interrupt sources

Interrupt
acronym

Interrupt
source

Interrupt
group

Interrupt
enable

Audio block mode

Interrupt clear

Depends on:

FREQ

FREQ

Master or slave
Receiver or
transmitter

FREQIE in
SAI_xIM
register

– FIFO threshold setting (FLVL bits
in SAI_xCR2)
– Communication direction
(transmitter or receiver)

For more details refer to
Section 69.4.9: Internal FIFOs
ERROR

Master or slave
Receiver or
transmitter

OVRUDRIE in
SAI_xIM
register

COVRUDR = 1 in SAI_xCLRFR
register

ERROR

Slave
(not used in AC’97
mode and SPDIF
mode)

AFSDETIE in
SAI_xIM
register

CAFSDET = 1 in SAI_xCLRFR
register

LFSDET

ERROR

Slave
(not used in AC’97
mode and SPDIF
mode)

LFSDETIE in
SAI_xIM
register

CLFSDET = 1 in SAI_xCLRFR
register

CNRDY

ERROR

Slave
(only in AC’97 mode)

CNRDYIE in
SAI_xIM
register

CCNRDY = 1 in SAI_xCLRFR
register

MUTEDET

MUTE

Master or slave
Receiver mode only

MUTEDETIE in
SAI_xIM
register

CMUTEDET = 1 in SAI_xCLRFR
register

WCKCFG

ERROR

Master with NODIV =
0 in SAI_xCR1
register

WCKCFGIE in
SAI_xIM
register

CWCKCFG = 1 in SAI_xCLRFR
register

OVRUDR

AFSDET
SAI

<!-- pagebreak -->

