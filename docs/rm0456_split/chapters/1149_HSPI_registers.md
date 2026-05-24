Event flag

Enable control bit

Timeout

TOF

TOIE

Status match

SMF

SMIE

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
Table 270. HSPI interrupt requests
Interrupt event

Event flag

Enable control bit

FIFO threshold

FTF

FTIE

Transfer complete

TCF

TCIE

Transfer error

TEF

TEIE

30.7

HSPI registers

30.7.1

HSPI control register (HSPI_CR)
Address offset: 0x000
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

PMM

APMS

Res.

TOIE

SMIE

FTIE

TCIE

TEIE

rw

rw

rw

rw

rw

rw

rw

7

6

4

3

2

1

0

MSEL[1:0]

FMODE[1:0]

rw

rw

rw

rw

15

14

13

12

Res.

Res.

11

10

9

8

FTHRES[5:0]
rw

rw

rw

rw

Res.
rw

rw

DMM
rw

5
Res.

Res.

TCEN
rw

DMAE
ABORT
N
rw

rw

EN
rw

Bits 31:30 MSEL[1:0]: Flash select
These bits select the memory to be addressed in single-, dual-, quad-, or octal-SPI mode
in single-memory configuration (when DMM = 0).
- when in quad-SPI mode:
00: Data exchanged over IO[3:0]
01: Data exchanged over IO[7:4]
10: Data exchanged over IO[11:8]
11: Data exchanged over IO[15:12]
- when in octal-SPI mode or dual-quad configuration:
0x: Data exchanged over IO[7:0]
1x: Data exchanged over IO[15:8]
These bits are ignored when in dual-octal configuration (data on 8 bits and DMM = 1) or
16-bit configuration (data exchanged over IO[15:0]).
Note: Bit 30 is mirrored in bit 7. This bitfield can be modified only when BUSY = 0.
Bits 29:28 FMODE[1:0]: Functional mode
This bitfield defines the HSPI functional mode of operation.
00: Indirect-write mode
01: Indirect-read mode
10: Automatic status-polling mode (relevant in regular-command protocol only)
11: Memory-mapped mode
If DMAEN = 1 already, then the DMA controller for the corresponding channel must be
disabled before changing the FMODE[1:0] value.
Note: This bitfield can be modified only when BUSY = 0.
Bits 27:24 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bit 23 PMM: Polling match mode
This bit indicates which method must be used to determine a match during the automatic
status-polling mode.
0: AND-match mode, SMF is set if all the unmasked bits received from the device match
the corresponding bits in the match register.
1: OR-match mode, SMF is set if any of the unmasked bits received from the device matches
its corresponding bit in the match register.
Note: This bit can be modified only when BUSY = 0.
Bit 22 APMS: Automatic status-polling mode stop
This bit determines if the automatic status-polling mode is stopped after a match.
0: Automatic status-polling mode is stopped only by abort or by disabling the HSPI.
1: Automatic status-polling mode stops as soon as there is a match.
Note: This bit can be modified only when BUSY = 0.
Bit 21 Reserved, must be kept at reset value.
Bit 20 TOIE: Timeout interrupt enable
This bit enables the timeout interrupt.
0: Interrupt disabled
1: Interrupt enabled
Bit 19 SMIE: Status match interrupt enable
This bit enables the status match interrupt.
0: Interrupt disabled
1: Interrupt enabled
Bit 18 FTIE: FIFO threshold interrupt enable
This bit enables the FIFO threshold interrupt.
0: Interrupt disabled
1: Interrupt enabled
Bit 17 TCIE: Transfer complete interrupt enable
This bit enables the transfer complete interrupt.
0: Interrupt disabled
1: Interrupt enabled
Bit 16 TEIE: Transfer error interrupt enable
This bit enables the transfer error interrupt.
0: Interrupt disabled
1: Interrupt enabled
Bits 15:14 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

Bits 13:8 FTHRES[5:0]: FIFO threshold level
This bitfield defines, in indirect mode, the threshold number of bytes in the FIFO that causes
the FIFO threshold flag FTF in HSPI_SR, to be set.
000000: FTF is set if there are one or more free bytes available to be written to in the FIFO
in indirect-write mode, or if there are one or more valid bytes can be read from the FIFO
in indirect-read mode.
000001: FTF is set if there are two or more free bytes available to be written to in the FIFO
in indirect-write mode, or if there are two or more valid bytes can be read from the FIFO
in indirect-read mode.
...
111111: FTF is set if there are 64 free bytes available to be written to in the FIFO
in indirect-write mode, or if there are 64 valid bytes can be read from the FIFO
in indirect-read mode.
Note: If DMAEN = 1, the DMA controller for the corresponding channel must be disabled
before changing the FTHRES[5:0] value.
Bit 7 Reserved, must be kept at reset value.
Bit 6 DMM: Dual-memory configuration
This bit activates the dual-memory configuration, where two external devices are used
simultaneously to double the throughput and the capacity.
0: Dual-memory configuration disabled
1: Dual-memory configuration enabled
Note: This bit can be modified only when BUSY = 0.
Bits 5:4 Reserved, must be kept at reset value.
Bit 3 TCEN: Timeout counter enable
This bit is valid only when the memory-mapped mode (FMODE[1:0] = 11) is selected. This bit
enables the timeout counter.
0: The timeout counter is disabled, and thus the chip-select (NCS) remains active indefinitely
after an access in memory-mapped mode.
1: The timeout counter is enabled, and thus the chip-select is released
in the memory-mapped mode after TIMEOUT[15:0] cycles of external device inactivity.
Note: This bit can be modified only when BUSY = 0.
Bit 2 DMAEN: DMA enable
In indirect mode, the DMA can be used to input or output data via HSPI_DR. DMA transfers
are initiated when FTF is set.
0: DMA disabled for indirect mode
1: DMA enabled for indirect mode
Note: Resetting the DMAEN bit while a DMA transfer is ongoing, breaks the handshake
with the DMA. Do not write this bit during DMA operation.
Bit 1 ABORT: Abort request
This bit aborts the ongoing command sequence. It is automatically reset once the abort is
completed. This bit stops the current transfer.
0: No abort requested
1: Abort requested
Note: This bit is always read as 0.

RM0456 Rev 6

<!-- pagebreak -->

