RM0456 Rev 6

RM0456

Digital camera interface (DCMI)

41.5.2

DCMI status register (DCMI_SR)
Address offset: 0x04
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

Res.

Res.

Res.

Res.

FNE
r

VSYNC HSYNC
r

r

Bits 31:3 Reserved, must be kept at reset value.
Bit 2 FNE: FIFO not empty
This bit gives the status of the FIFO.
1: FIFO contains valid data.
0: FIFO empty
Bit 1 VSYNC: Vertical synchronization
This bit gives the state of the DCMI_VSYNC pin with the correct programmed polarity. When
embedded synchronization codes are used, the meaning of this bit is the following:
0: active frame
1: synchronization between frames
In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in
DCMI_CR is set.
Bit 0 HSYNC: Horizontal synchronization
This bit gives the state of the DCMI_HSYNC pin with the correct programmed polarity. When
embedded synchronization codes are used, the meaning of this bit is the following:
0: active line
1: synchronization between lines
In case of embedded synchronization, this bit is meaningful only if the CAPTURE bit in
DCMI_CR is set.

41.5.3

DCMI raw interrupt status register (DCMI_RIS)
DCMI_RIS gives the raw interrupt status and is accessible in read only. When read, this
register returns the status of the corresponding interrupt before masking with the DCMI_IER
register value.
Address offset: 0x08
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

LINE
_RIS

VSYNC
_RIS

ERR
_RIS

OVR
_RIS

FRAME
_RIS

r

r

r

r

r

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RM0456 Rev 6

Res.

<!-- pagebreak -->

