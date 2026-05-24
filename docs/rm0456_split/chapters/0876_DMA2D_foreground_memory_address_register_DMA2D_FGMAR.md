891

Chrom-ART Accelerator controller (DMA2D)

RM0456

Bit 4 CCTCIF: Clear CLUT transfer complete interrupt flag
Programming this bit to 1 clears the CTCIF flag in DMA2D_ISR.
Bit 3 CAECIF: Clear CLUT access error interrupt flag
Programming this bit to 1 clears the CAEIF flag in DMA2D_ISR.
Bit 2 CTWIF: Clear transfer watermark interrupt flag
Programming this bit to 1 clears the TWIF flag in DMA2D_ISR.
Bit 1 CTCIF: Clear transfer complete interrupt flag
Programming this bit to 1 clears the TCIF flag in DMA2D_ISR.
Bit 0 CTEIF: Clear transfer error interrupt flag
Programming this bit to 1 clears the TEIF flag in DMA2D_ISR.

20.5.4

DMA2D foreground memory address register (DMA2D_FGMAR)
Address offset: 0x00C
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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

MA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

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

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

MA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 MA[31:0]: Memory address, address of the data used for the foreground image
The address alignment must match the image format selected: a 32-bit per pixel format must
be 32-bit aligned, a 16-bit per pixel format must be 16-bit aligned, and a 4-bit per pixel format
must be 8-bit aligned.

20.5.5

DMA2D foreground offset register (DMA2D_FGOR)
Address offset: 0x010
Reset value: 0x0000 0000
This register can only be written when data transfers are disabled. Once the data transfer
started, this register is read-only.

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

rw

rw

rw

rw

rw

rw

rw

LO[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

