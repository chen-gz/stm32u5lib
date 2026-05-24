3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.18 OTG host periodic transmit FIFO size register
(OTG_HPTXFSIZ)
Address offset: 0x100
Reset value: 0x0200 0400
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

PTXFSIZ[15:0]
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

PTXSA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 PTXFSIZ[15:0]: Host periodic Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Bits 15:0 PTXSA[15:0]: Host periodic Tx FIFO start address
This field configures the memory start address for periodic transmit FIFO RAM.

72.15.19 OTG device IN endpoint transmit FIFO x size register
(OTG_DIEPTXFx)
Address offset: 0x104 + 0x04 * (x - 1), (x = 1 to 5)
Reset value: Block 1: 0x0200 0400
Reset value: Block 2: 0x0200 0600
Reset value: Block 3: 0x0200 0800
Reset value: Block 4: 0x0200 0A00
Reset value: Block 5: 0x0200 0C00
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

INEPTXFD[15:0]
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

INEPTXSA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:16 INEPTXFD[15:0]: IN endpoint Tx FIFO depth
This value is in terms of 32-bit words.
Minimum value is 16
Bits 15:0 INEPTXSA[15:0]: IN endpoint FIFOx transmit RAM start address
This field contains the memory start address for IN endpoint transmit FIFOx. The address
must be aligned with a 32-bit memory location.

<!-- pagebreak -->

