3020

Serial audio interface (SAI)

RM0456

Bits 31:16 SLOTEN[15:0]: Slot enable.
These bits are set and cleared by software.
Each SLOTEN bit corresponds to a slot position from 0 to 15 (maximum 16 slots).
0: Inactive slot.
1: Active slot.
The slot must be enabled when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.
Bits 15:12 Reserved, must be kept at reset value.
Bits 11:8 NBSLOT[3:0]: Number of slots in an audio frame.
These bits are set and cleared by software.
The value set in this bitfield represents the number of slots + 1 in the audio frame (including the
number of inactive slots). The maximum number of slots is 16.
The number of slots must be even if FSDEF bit in the SAI_xFRCR register is set.
The number of slots must be configured when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.
Bits 7:6 SLOTSZ[1:0]: Slot size
This bits is set and cleared by software.
The slot size must be higher or equal to the data size. If this condition is not respected, the behavior
of the SAI is undetermined.
Refer to Output data line management on an inactive slot for information on how to drive SD line.
These bits must be set when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.
00: The slot size is equivalent to the data size (specified in DS[3:0] in the SAI_xCR1 register).
01: 16-bit
10: 32-bit
11: Reserved
Bit 5 Reserved, must be kept at reset value.
Bits 4:0 FBOFF[4:0]: First bit offset
These bits are set and cleared by software.
The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents
an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception
mode, the extra received bits are discarded.
These bits must be set when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.

69.6.14

SAI interrupt mask register (SAI_BIM)
Address offset: 0x34
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

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LFSDET AFSDETI CNRDY FREQ WCKCFG MUTEDET OVRUDR
IE
E
IE
IE
IE
IE
IE
rw

<!-- pagebreak -->

