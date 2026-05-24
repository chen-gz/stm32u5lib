RM0456 Rev 6

RM0456

Digital camera interface (DCMI)

Bit 1 OVR_ISC: Overrun interrupt status clear
Setting this bit clears the OVR_RIS flag in the DCMI_RIS register.
Bit 0 FRAME_ISC: Capture complete interrupt status clear
Setting this bit clears the FRAME_RIS flag in the DCMI_RIS register.

41.5.7

DCMI embedded synchronization code register (DCMI_ESCR)
Address offset: 0x18
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

FEC[7:0]

20

19

18

17

16

LEC[7:0]

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

LSC[7:0]
rw

rw

rw

rw

rw

FSC[7:0]
rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:24 FEC[7:0]: Frame end delimiter code
This byte specifies the code of the frame end delimiter. The code consists of 4 bytes in the
form of 0xFF, 0x00, 0x00, FEC.
If FEC is programmed to 0xFF, all the unused codes (0xFF0000XY) are interpreted as frame
end delimiters.
Bits 23:16 LEC[7:0]: Line end delimiter code
This byte specifies the code of the line end delimiter. The code consists of 4 bytes in the form
of 0xFF, 0x00, 0x00, LEC.
Bits 15:8 LSC[7:0]: Line start delimiter code
This byte specifies the code of the line start delimiter. The code consists of 4 bytes in the
form of 0xFF, 0x00, 0x00, LSC.
Bits 7:0 FSC[7:0]: Frame start delimiter code
This byte specifies the code of the frame start delimiter. The code consists of 4 bytes in the
form of 0xFF, 0x00, 0x00, FSC.
If FSC is programmed to 0xFF, no frame start delimiter is detected. But, the first occurrence
of LSC after an FEC code is interpreted as a start of frame delimiter.

41.5.8

DCMI embedded synchronization unmask register (DCMI_ESUR)
Address offset: 0x1C
Reset value: 0x0000 0000

31

30

29

28

rw

rw

rw

rw

15

14

13

12

27

26

25

24

23

22

21

20

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

FEU[7:0]

rw

rw

rw

rw

18

17

16

rw

rw

rw

rw

3

2

1

0

rw

rw

rw

LEU[7:0]

LSU[7:0]
rw

19

FSU[7:0]
rw

rw

rw

rw

RM0456 Rev 6

rw

rw

rw

rw

<!-- pagebreak -->

