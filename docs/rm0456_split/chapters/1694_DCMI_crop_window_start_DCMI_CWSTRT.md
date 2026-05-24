1696

Digital camera interface (DCMI)

RM0456

Bits 31:24 FEU[7:0]: Frame end delimiter unmask
This byte specifies the mask to be applied to the code of the frame end delimiter.
0: The corresponding bit in the FEC byte in DCMI_ESCR is masked while comparing the
frame end delimiter with the received data.
1: The corresponding bit in the FEC byte in DCMI_ESCR is compared while comparing the
frame end delimiter with the received data.
Bits 23:16 LEU[7:0]: Line end delimiter unmask
This byte specifies the mask to be applied to the code of the line end delimiter.
0: The corresponding bit in the LEC byte in DCMI_ESCR is masked while comparing the line
end delimiter with the received data.
1: The corresponding bit in the LEC byte in DCMI_ESCR is compared while comparing the
line end delimiter with the received data.
Bits 15:8 LSU[7:0]: Line start delimiter unmask
This byte specifies the mask to be applied to the code of the line start delimiter.
0: The corresponding bit in the LSC byte in DCMI_ESCR is masked while comparing the line
start delimiter with the received data.
1: The corresponding bit in the LSC byte in DCMI_ESCR is compared while comparing the
line start delimiter with the received data.
Bits 7:0 FSU[7:0]: Frame start delimiter unmask
This byte specifies the mask to be applied to the code of the frame start delimiter.
0: The corresponding bit in the FSC byte in DCMI_ESCR is masked while comparing the
frame start delimiter with the received data.
1: The corresponding bit in the FSC byte in DCMI_ESCR is compared while comparing the
frame start delimiter with the received data.

41.5.9

DCMI crop window start (DCMI_CWSTRT)
Address offset: 0x20
Reset value: 0x0000 0000

31

30

29

Res.

Res.

Res.

28

27

26

25

24

23

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

Res.

Res.
rw

rw

rw

rw

rw

rw

22

21

20

19

18

17

16

rw

rw

rw

rw

rw

rw

rw

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

VST[12:0]

HOFFCNT[13:0]
rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:16 VST[12:0]: Vertical start line count
The image capture starts with this line number. Previous line data are ignored.
0x0000: line 1
0x0001: line 2
0x0002: line 3
....
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:0 HOFFCNT[13:0]: Horizontal offset count
This value gives the number of pixel clocks to count before starting a capture.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Digital camera interface (DCMI)

41.5.10

DCMI crop window size (DCMI_CWSIZE)
Address offset: 0x24
Reset value: 0x0000 0000

31

30

Res.

Res.

29

15

14

Res.

Res.

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

VLINE[13:0]
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

rw

rw

rw

rw

rw

CAPCNT[13:0]
rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bits 29:16 VLINE[13:0]: Vertical line count
This value gives the number of lines to be captured from the starting point.
0x0000: 1 line
0x0001: 2 lines
0x0002: 3 lines
....
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:0 CAPCNT[13:0]: Capture count
This value gives the number of pixel clocks to be captured from the starting point on the
same line. It value must corresponds to word-aligned data for different widths of parallel
interfaces.
0x0000: 1 pixel
0x0001: 2 pixels
0x0002: 3 pixels
....

41.5.11

DCMI data register (DCMI_DR)
Address offset: 0x28
Reset value: 0x0000 0000
The digital camera Interface packages all the received data in 32-bit format before
requesting a DMA transfer. A 8-word deep FIFO is available to leave enough time for DMA
transfers and avoid DMA overrun conditions.

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

BYTE3[7:0]
r

r

r

r

r

r

r

r

r

r

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

BYTE1[7:0]
r

r

r

r

19

18

17

16

BYTE2[7:0]

r

r

20

r

r

r

r

r

4

3

2

1

0

r

r

r

BYTE0[7:0]
r

r

r

r

r

r

r

r

Bits 31:24 BYTE3[7:0]: Data byte 3
Bits 23:16 BYTE2[7:0]: Data byte 2
Bits 15:8 BYTE1[7:0]: Data byte 1

RM0456 Rev 6

<!-- pagebreak -->

