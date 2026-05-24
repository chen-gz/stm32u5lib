RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

39.8.5

MDF digital filter control register x (MDF_DFLTxCR)
Address offset: 0x088 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
The number of registers is equal to the amount of filters. Refer to Section 39.3 for details.

31

30

DFLTA
CTIVE

DFLTR
UN

r

r

15

14

29

28

Res.

Res.

13

12

TRGSRC[3:0]
rw

rw

rw

27

25

24

23

22

21

20

NBDIS[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

11

10

9

8

7

6

5

4

Res.

TRGSE
NS

Res.
rw

26

Res.

Res.

rw

ACQMOD[2:0]
rw

rw

19

17

16
SNPSF
MT

Res.

Res.

Res.

3

2

1

0

FTH

DMAE
N

DFLTE
N

rw

rw

w

rw

Res.
rw

18

Bit 31 DFLTACTIVE: Digital filter active flag
This bit is set and cleared by hardware. It indicates if the digital filter is active: can be running
or waiting for events.
0: Digital filter not active (can be re-enabled again, via DFLTEN, if needed)
1: Digital filter active
Bit 30 DFLTRUN: Digital filter run status flag
This bit is set and cleared by hardware. It indicates if the digital filter is running or not.
0: Digital filter not running and ready to accept a new trigger event
1: Digital filter running
Bits 29:28 Reserved, must be kept at reset value.
Bits 27:20 NBDIS[7:0]: Number of samples to be discarded
This bitfield is set and cleared by software. it is used to define the number of samples to be
discarded every time the DFLTx is re-started.
0: No sample discarded
1: 1 sample discarded
2: 2 samples discarded
...
255: 255 samples discarded
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 19:17 Reserved, must be kept at reset value.
Bit 16 SNPSFMT: Snapshot data format
This bit is set and cleared by software. It is used to select the data format for the snapshot
mode.
0: Integrator counter (INT_CNT) not inserted into MDF_SNPSxDR, leaving a data resolution
of 23 bits
1: Integrator counter (INT_CNT) inserted at position [15:9] of MDF_SNPSxDR, leaving a data
resolution of 16 bits.
Note: This bit can be write-protected (refer to Section 39.4.15 for details).

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Bits 15:12 TRGSRC[3:0]: Digital filter trigger signal selection
This bitfield is set and cleared by software. It is used to select which external signals trigger
the corresponding filter.
0000: TRGO selected
0001: OLDx event selected
0010: mdf_trg[0] selected
...
1111: mdf_trg[13] selected
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 11:9 Reserved, must be kept at reset value.
Bit 8 TRGSENS: Digital filter trigger sensitivity selection
this bit is set and cleared by software. It is used to select the trigger sensitivity of the external
signals
0: A rising edge event triggers the acquisition.
1: A falling edge even triggers the acquisition.
Note: When the trigger source is TRGO or OLDx event, TRGSENS value is not taken into
account. When TRGO is selected, the sensitivity is forced to falling edge, when OLDx
event is selected, the sensitivity is forced to rising edge.
This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 ACQMOD[2:0]: Digital filter trigger mode
This bitfield is set and cleared by software. It is used to select the filter trigger mode.
000: Asynchronous continuous acquisition mode
001: Asynchronous single-shot acquisition mode
010: Synchronous continuous acquisition mode
011: Synchronous, single-shot acquisition mode
100: Window continuous acquisition mode
101: Synchronous snapshot mode
Others: same a 000
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bit 3 Reserved, must be kept at reset value.
Bit 2 FTH: RXFIFO threshold selection
This bit is set and cleared by software. It is used to select the RXFIFO threshold. This bit is
not significant for RXFIFOs working in interleaved-transfer mode (see Interleaved-transfer
mode for details).
0: RXFIFO threshold event generated when the RXFIFO is not empty
1: RXFIFO threshold event generated when the RXFIFO is half-full
Note: This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 1 DMAEN: DMA requests enable
This bit is set and cleared by software. It is used to control the generation of DMA request to
transfer the processed samples into the memory.
0: DMA interface for the corresponding digital filter disabled
1: DMA interface for the corresponding digital filter enabled
Note: This bit can be write-protected (refer to Section 39.4.15 for details).

<!-- pagebreak -->

