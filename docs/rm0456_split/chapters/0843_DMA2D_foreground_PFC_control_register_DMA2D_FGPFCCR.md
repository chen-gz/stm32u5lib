RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)

19.5.8

DMA2D foreground PFC control register (DMA2D_FGPFCCR)
Address offset: 0x01C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

ALPHA[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

rw

rw

rw

rw

rw

rw

rw

CS[7:0]
rw

23

22

21

20

19

18

Res.

Res.

RBS

AI

Res.

Res.

rw

rw

7

6

5

4

3

2

Res.

Res.

START

CCM

rs

rw

rw

rw

17

16

AM[1:0]
rw

rw

1

0

rw

rw

CM[3:0]

Bits 31:24 ALPHA[7:0]: Alpha value
These bits define a fixed alpha channel value which can replace the original alpha value or
be multiplied by the original alpha value according to the alpha mode selected through the
AM[1:0] bits.
These bits can only be written when data transfers are disabled. Once a transfer has started,
they become read-only.
Bits 23:22 Reserved, must be kept at reset value.
Bit 21 RBS: Red Blue swap
This bit is used to swap the R and B to support BGR or ABGR color formats. Once the
transfer has started, this bit is read-only.
0: Regular mode (RGB or ARGB)
1: Swap mode (BGR or ABGR)
Bit 20 AI: Alpha Inverted
This bit inverts the alpha value. Once the transfer has started, this bit is read-only.
0: Regular alpha
1: Inverted alpha
Bits 19:18 Reserved, must be kept at reset value.
Bits 17:16 AM[1:0]: Alpha mode
These bits select the alpha channel value to be used for the foreground image. They can only
be written data the transfer are disabled. Once the transfer has started, they become readonly.
00: No modification of the foreground image alpha channel value
01: Replace original foreground image alpha channel value by ALPHA[7:0]
10: Replace original foreground image alpha channel value by ALPHA[7:0] multiplied with
original alpha channel value
Other: Reserved
Bits 15:8 CS[7:0]: CLUT size
These bits define the size of the CLUT used for the foreground image. Once the CLUT
transfer has started, this field is read-only.
The number of CLUT entries is equal to CS[7:0] + 1.
Bits 7:6 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

