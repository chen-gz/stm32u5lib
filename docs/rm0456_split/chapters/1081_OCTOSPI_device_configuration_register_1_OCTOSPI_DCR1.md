RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bit 0 EN: Enable
This bit enables the OCTOSPI.
0: OCTOSPI disabled
1: OCTOSPI enabled
Note: The DMA request can be aborted without having received the ACK in case this EN bit is
cleared during the operation.
In case this bit is set to 0 during a DMA transfer, the REQ signal to DMA returns to
inactive state without waiting for the ACK signal from DMA to be active.

28.7.2

OCTOSPI device configuration register 1 (OCTOSPI_DCR1)
Address offset: 0x0008
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

31

30

29

28

27

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

Res.

Res.

26

25

24

MTYP[2:0]
rw

rw

rw

10

9

8

CSHT[5:0]
rw

rw

rw

rw

23

22

21

Res.

Res.

Res.

7

6

5

Res.
rw

rw

Res.

Res.

20

19

18

17

16

rw

DEVSIZE[4:0]
rw

rw

rw

rw

4

3

2

1

0

Res.

DLY
BYP

FRCK

CKMO
DE

rw

rw

rw

Res.

Bits 31:27 Reserved, must be kept at reset value.
Bits 26:24 MTYP[2:0]: Memory type
This bitfield indicates the type of memory to be supported.
000: Micron mode, D0/D1 ordering in DTR 8-data-bit mode. regular-command protocol in
single-, dual-, quad- and octal-SPI modes.
Note: In this mode, DQS signal polarity is inverted with respect to the memory clock signal.
This is the default value and care must be taken to change MTYP[2:0] for memories different
from Micron.
001: Macronix mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command protocol
in single-, dual-, quad-, and octal-SPI modes.
010: Standard mode
011: Macronix RAM mode, D1/D0 ordering in DTR 8-data-bit mode. Regular-command
protocol in single-, dual-, quad-, and octal-SPI modes with dedicated address mapping
(address is built with row and column to fit with Macronix requirements).
100: HyperBus memory mode, the protocol follows the HyperBus specification.
101: HyperBus register mode, addressing register space. The memory-mapped accesses in
this mode must be noncacheable, or indirect-read/write modes must be used.
Others: Reserved
Bits 23:21 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

Bits 20:16 DEVSIZE[4:0]: Device size
This bitfield defines the size of the external device using the following formula:
Number of bytes in device = 2[DEVSIZE+1].
DEVSIZE + 1 is effectively the number of address bits required to address the external
device. The device capacity can be up to 4 Gbytes (addressed using 32-bits) in indirect
mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes.
In regular-command protocol, if DMM = 1, DEVSIZE[4:0] must reflect the total capacity of the
two devices together considering the above formula (DEVSIZE[4:0] value is so equal to one
of the two memory capacities).
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:8 CSHT[5:0]: Chip-select high time
CSHT + 1 defines the minimum number of CLK cycles where the chip-select (NCS) must
remain high between commands issued to the external device.
0x0: NCS stays high for at least 1 cycle between external device commands.
0x1: NCS stays high for at least 2 cycles between external device commands.
...
0x3F: NCS stays high for at least 64 cycles between external device commands.
Bits 7:4 Reserved, must be kept at reset value.
Bit 3 DLYBYP: Delay block bypass
0: The internal sampling clock (called feedback clock) or the DQS data strobe external signal
is delayed by the delay block (for more details on this block, refer to the dedicated section of
the reference manual as it is not part of the OCTOSPI peripheral).
1: The delay block is bypassed, so the internal sampling clock or the DQS data strobe
external signal is not affected by the delay block. The delay is shorter than when the delay
block is not bypassed, even with the delay value set to minimum value in delay block.
Bit 2 Reserved, must be kept at reset value.
Bit 1 FRCK: Free running clock
This bit configures the free running clock.
0: CLK is not free running.
1: CLK is free running (always provided).
Note: Free running clock mode is intended for delay calibration only. No memory or other
device access is possible when FRCK is set. Once the bit has been set to 1, the BSY bit
is set, the NCS signal remains set to 1 and the clock is sent. The only way to stop the
clock is to perform an abort (which clears the BSY bit). Then, the FRCK bit must be
cleared by software to allow controller transactions to take place with the external
memory.
Bit 0 CKMODE: Clock mode 0/mode 3
This bit indicates the level taken by the CLK between commands (when NCS = 1).
0: CLK must stay low while NCS is high (chip-select released). This is referred to as
clock mode 0.
1: CLK must stay high while NCS is high (chip-select released). This is referred to as
clock mode 3.

<!-- pagebreak -->

