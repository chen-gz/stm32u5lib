RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

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
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 FRCK: Free running clock
This bit configures the free running clock.
0: CLK is not free running.
1: CLK is free running (always provided).
Note: Once the bit has been set to 1, the BSY bit is set, the NCS signal remains set to 1 and
the clock is sent. The only way to stop the clock is to perform an abort (which clears the
BSY bit). Then, the FRCK bit must be cleared by software to allow controller
transactions to take place with the external memory.
Bit 0 CKMODE: Clock mode 0/mode 3
This bit indicates the level taken by the CLK between commands (when NCS = 1).
0: CLK must stay low while NCS is high (chip-select released), referred to as clock mode 0.
1: CLK must stay high while NCS is high (chip-select released), referred to as clock mode 3.

30.7.3

HSPI device configuration register 2 (HSPI_DCR2)
Address offset: 0x00C
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

10

9

8

7

6

5

4

3

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

18

17

16

WRAPSIZE[2:0]
rw

rw

rw

2

1

0

rw

rw

rw

PRESCALER[7:0]
rw

rw

rw

rw

rw

Bits 31:19 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

