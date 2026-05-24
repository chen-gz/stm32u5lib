3631

Debug support (DBG)

RM0456

Bits 1:0 TXMODE[1:0]: protocol used for trace output
0x0: parallel trace port mode
0x1: asynchronous SWO using Manchester encoding
0x2: asynchronous SWO using NRZ encoding
0x3: reserved

TPIU formatter and flush status register (TPIU_FFSR)
Address offset: 0x300
Reset value: 0x0000 0008
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

FTNON TCPRE FTSTO
STOP SENT PPED
r

r

r

0
FLINP
ROG
r

Bits 31:4 Reserved, must be kept at reset value.
Bit 3 FTNONSTOP: formatter stop
Indicates whether formatter can be stopped or not.
1: The formatter cannot be stopped.
Bit 2 TCPRESENT: TRACECTL output pin availability
Indicates whether the optional TRACECTL output pin is available for use.
0: TRACECTL pin is not present in this device.
Bit 1 FTSTOPPED: formatter stop
The formatter has received a stop request signal and all trace data and post-amble is sent.
Any additional trace data on the ATB interface is ignored.
0: The formatter has not stopped.
Bit 0 FLINPROG: flush in progress
Indicates whether a flush on the ATB slave port is in progress. This bit reflects the status of
the AFVALIDS output. A flush can be initiated by the flush control bits in the TPIU_FFCR
register.
0: no flush in progress
1: flush in progress

TPIU formatter and flush control register (TPIU_FFCR)
Address offset: 0x304
Reset value: 0x0000 0102
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

FONM
AN

Res.

ENFCO
NT

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

TRIGIN
r

<!-- pagebreak -->

RM0456 Rev 6

rw

Res.

Res.

Res.

rw

RM0456

Debug support (DBG)

Bits 31:9 Reserved, must be kept at reset value.
Bit 8 TRIGIN: trigger on trigger in
1: Indicates a trigger in the trace stream when the TRIGIN input is asserted.
Bit 7 Reserved, must be kept at reset value.
Bit 6 FONMAN: flush on manual
0: flush completed
1: Generates a flush.
Bits 5:2 Reserved, must be kept at reset value.
Bit 1 ENFCONT: continuous formatting enable
Setting this bit to zero in SWO mode bypasses the formatter and only ITM/DWT trace is
output, ETM trace is discarded.
0: continuous formatting disabled
1: continuous formatting enabled
Bit 0 Reserved, must be kept at reset value.

TPIU periodic synchronization counter register (TPIU_PSCR)
Address offset: 0x308
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

15

14

13

Res.

Res.

Res.

PSCOUNT[12:0]
rw

rw

rw

rw

rw

rw

rw

Bits 31:13 Reserved, must be kept at reset value.
Bits 12:0 PSCOUNT[12:0]: formatter frames counter
Enables effective use of different sized TPAs without wasting large amounts of the storage
capacity of the capture device. This counter contains the number of formatter frames since
the last synchronization packet of 128 bits. It is a 12-bit counter with a maximum count value
of 4096. This equates to synchronization every 65536 bytes, that is, 4096 packets x 16 bytes
per packet. The default is set up for a synchronization packet every 1024 bytes, that is, every
64 formatter frames. If the formatter is configured for continuous mode, full and half-word
synchronization frames are inserted during normal operation. Under these circumstances,
the count value is the maximum number of complete frames between full synchronization
packets.

RM0456 Rev 6

<!-- pagebreak -->

3631

Debug support (DBG)

RM0456

TPIU claim tag set register (TPIU_CLAIMSETR)
Address offset: 0xFA0
Reset value: 0x0000 000F
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

3

2

1

0

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

CLAIMSET[3:0]
rw

rw

rw

rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 CLAIMSET[3:0]: claim tag bits setting
Write:
0000: no effect
xxx1: Sets bit 0.
xx1x: Sets bit 1.
x1xx: Sets bit 2.
1xxx: Sets bit 3.
Read:
0xF: Indicates there are four bits in claim tag.

TPIU claim tag clear register (TPIU_CLAIMCLR)
Address offset: 0xFA4
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

3

2

1

0

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

CLAIMCLR[3:0]
rw

Bits 31:4 Reserved, must be kept at reset value.
Bits 3:0 CLAIMCLR[3:0]: claim tag bits reset
Write:
0000: no effect
xxx1: Clears bit 0.
xx1x: Clears bit 1.
x1xx: Clears bit 2.
1xxx: Clears bit 3.
Read: Returns current value of claim tag.

<!-- pagebreak -->

