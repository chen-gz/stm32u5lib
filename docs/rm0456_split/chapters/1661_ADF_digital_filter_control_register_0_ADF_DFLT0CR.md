RM0456 Rev 6

RM0456

Audio digital filter (ADF)

40.8.5

ADF digital filter control register 0 (ADF_DFLT0CR)
Address offset: 0x088
Reset value: 0x0000 0000

31

30

29

28

DFLTA
CTIVE

DFLTR
UN

Res.

Res.

13

12

r

r

15

14

TRGSRC[3:0]
rw

rw

rw

rw

27

26

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

Res.

Res.

TRGSE
NS

Res.

rw

ACQMOD[2:0]
rw

rw

rw

19

18

17

16

Res.

Res.

Res.

Res.

3

2

1

0

Res.

FTH

DMAE
N

DFLTE
N

rw

rw

w

Bit 31 DFLTACTIVE: DFLT0 active flag
This bit is set and cleared by hardware. It indicates if DFLT0 is active: can be running or
waiting for events.
0: DFLT0 not active (can be re-enabled again, via DFLTEN bit, if needed)
1: DFLT0 active
Bit 30 DFLTRUN: DFLT0 run status flag
This bit is set and cleared by hardware. It indicates if DFLT0 is running or not.
0: DFLT0 not running and ready to accept a new trigger event
1: DFLT0running
Bits 29:28 Reserved, must be kept at reset value.
Bits 27:20 NBDIS[7:0]: Number of samples to be discarded
This bitfield is set and cleared by software. It is used to define the number of samples to be
discarded every time DFLT0 is re-started.
0: No sample discarded
1: 1 sample discarded
2: 2 samples discarded
...
255: 255 samples discarded
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 19:16 Reserved, must be kept at reset value.
Bits 15:12 TRGSRC[3:0]: DFLT0 trigger signal selection
This bitfield is set and cleared by software. It is used to select which external signals trigger
DFLT0.
0000: TRGO selected
0010: adf_trgi selected
Others: Reserved
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bits 11:9 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1674

Audio digital filter (ADF)

RM0456

Bit 8 TRGSENS: DFLT0 trigger sensitivity selection
This bitfield is set and cleared by software. It is used to select the trigger sensitivity of the
external signals
0: A rising edge event triggers the acquisition.
1: A falling edge even triggers the acquisition.
Note: When the trigger source is TRGO, TRGSENS value is not taken into account. When
TRGO is selected, the sensitivity is forced to falling edge.
This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 7 Reserved, must be kept at reset value.
Bits 6:4 ACQMOD[2:0]: DFLT0 trigger mode
This bitfield is set and cleared by software. It is used to select the filter trigger mode.
000: Asynchronous continuous acquisition mode
001: Asynchronous single-shot acquisition mode
010: Synchronous continuous acquisition mode
011: Synchronous single-shot acquisition mode
100: Window continuous acquisition mode
Others: Same as 000
Note: This bitfield can be write-protected (see Section 40.4.13: Register protection for details).
Bit 3 Reserved, must be kept at reset value.
Bit 2 FTH: RXFIFO threshold selection
This bit is set and cleared by software. It is used to select the RXFIFO threshold.
0: RXFIFO threshold event generated when the RXFIFO is not empty
1: RXFIFO threshold event generated when the RXFIFO is half-full
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 1 DMAEN: DMA requests enable
This bit is set and cleared by software. It is used to control the generation of DMA request to
transfer the processed samples into the memory.
0: DMA interface for the corresponding digital filter disabled
1: DMA interface for the corresponding digital filter enabled
Note: This bit can be write-protected (see Section 40.4.13: Register protection for details).
Bit 0 DFLTEN: DFLT0 enable
This bit is set and cleared by software. It is used to control the start of acquisition of the
DFLT0 path. This bit behavior depends on ACQMOD[2:0] and external events. The serial or
parallel interface delivering the samples must be enabled as well.
0: Acquisition immediately stopped
1: Acquisition immediately started if ACQMOD[2:0] = 00x or 101, or acquisition started when
the proper trigger event occurs if ACQMOD[2:0] = 01x.

<!-- pagebreak -->

