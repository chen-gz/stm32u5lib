3086

FD controller area network (FDCAN)

RM0456

Bits 27:24 LSE[3:0]: Number of extended filter elements in the list
0: No extended message ID filter
1 to 8: Number of extended message ID filter elements
> 8: Values greater than 8 are interpreted as 8.
This bitfield is write-protected (P), which means that write access is possible only when the
CCE and INIT bits of the FDCAN_CCCR register are both set.
Bits 23:21 Reserved, must be kept at reset value.
Bits 20:16 LSS[4:0]: Number of standard filter elements in the list
0: No standard message ID filter
1 to 28: Number of standard message ID filter elements
> 28: Values greater than 28 are interpreted as 28.
This bitfield is write protected (P), which means that write access by the bits is possible only
when the CCE and INIT bits of the FDCAN_CCCR register are both set.
Bits 15:10 Reserved, must be kept at reset value.
Bit 9 F0OM: FIFO 0 operation mode (overwrite or blocking)
This bit is write-protected (P), which means that write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.
Bit 8 F1OM: FIFO 1 operation mode (overwrite or blocking)
This bit is write-protected (P), which means that write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 ANFS[1:0]: Accept Non-matching frames standard
Defines how received messages with 11-bit IDs that do not match any element of the filter list
are treated.
00: Accept in Rx FIFO 0
01: Accept in Rx FIFO 1
10: Reject
11: Reject
This bitfield is write-protected (P), which means write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.
Bits 3:2 ANFE[1:0]: Accept non-matching frames extended
Defines how received messages with 29-bit IDs that do not match any element of the filter list
are treated.
00: Accept in Rx FIFO 0
01: Accept in Rx FIFO 1
10: Reject
11: Reject
This bitfield is write-protected (P), which means that write access is possible only when the
CCE and INIT bits of the FDCAN_CCCR register are both set.
Bit 1 RRFS: Reject remote frames standard
0: Filter remote frames with 11-bit standard IDs
1: Reject all remote frames with 11-bit standard IDs
This bit is write-protected (P), which means that write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Bit 0 RRFE: Reject remote frames extended
0: Filter remote frames with 29-bit standard IDs
1: Reject all remote frames with 29-bit standard IDs
This bit is write-protected (P), which means that write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.

70.4.20

FDCAN extended ID and mask register (FDCAN_XIDAM)
Address offset: 0x0084
Reset value: 0x1FFF FFFF

31

30

29

Res.

Res.

Res.

15

14

13

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

12

11

10

9

8

7

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

rw

EIDM[28:16]

EIDM[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bits 28:0 EIDM[28:0]: Extended ID mask
For acceptance filtering of extended frames the extended ID AND mask is AND-ed with the
message ID of a received frame. Intended for masking of 29-bit IDs in SAE J1939. With the
reset value of all bits set, the mask is not active.
This bitfield is write-protected (P), which means that write access is possible only when the
CCE and INIT bits of the FDCAN_CCCR register are both set.

70.4.21

FDCAN high-priority message status register (FDCAN_HPMS)
This register is updated every time a message ID filter element configured to generate a
priority event match. This can be used to monitor the status of incoming high priority
messages and to enable fast access to these messages.
Address offset: 0x0088
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

FLST

Res.

Res.

Res.

Res.

Res.

r

r

r

r

r

r

FIDX[4:0]
r

MSI[1:0]
r

BIDX[2:0]
r

r

r

Bits 31:16 Reserved, must be kept at reset value.
Bit 15 FLST: Filter list
Indicates the filter list of the matching filter element:
0: Standard filter list
1: Extended filter list
Bits 14:13 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Bits 12:8 FIDX[4:0]: Filter index
Index of matching filter element.
Range: 0 to LSS[4:0] - 1 or LSE[3:0] - 1 in FDCAN_RXGFC.
Bits 7:6 MSI[1:0]: Message storage indicator
00: No FIFO selected
01: FIFO overrun
10: Message stored in FIFO 0
11: Message stored in FIFO 1
Bits 5:3 Reserved, must be kept at reset value.
Bits 2:0 BIDX[2:0]: Buffer index
Index of Rx FIFO element to which the message was stored. Only valid when MSI[1] = 1.

70.4.22

FDCAN Rx FIFO 0 status register (FDCAN_RXF0S)
Address offset: 0x0090
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

Res.

Res.

Res.

Res.

Res.

Res.

RF0L

F0F

Res.

Res.

Res.

Res.

Res.

Res.

F0PI[1:0]

r

r
8

15

14

13

12

11

10

9

Res.

Res.

Res.

Res.

Res.

Res.

F0GI[1:0]
r

7

6

5

4

Res.

Res.

Res.

Res.

r

3

2

16

r

r

1

0

F0FL[3:0]
r

r

r

r

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 RF0L: Rx FIFO 0 message lost
This bit is a copy of the RF0L interrupt flag of the FDCAN_IR register. When RF0L is cleared,
this bit is also cleared.
0: No Rx FIFO 0 message lost
1: Rx FIFO 0 message lost, also set after write attempt to Rx FIFO 0 of size 0
Bit 24 F0F: Rx FIFO 0 full
0: Rx FIFO 0 not full
1: Rx FIFO 0 full
Bits 23:18 Reserved, must be kept at reset value.
Bits 17:16 F0PI[1:0]: Rx FIFO 0 put index
Rx FIFO 0 write index pointer.
Range: 0 to 2.
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:8 F0GI[1:0]: Rx FIFO 0 get index
Rx FIFO 0 read index pointer.
Range: 0 to 2.
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 F0FL[3:0]: Rx FIFO 0 fill level
Number of elements stored in Rx FIFO 0.
Range: 0 to 3.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

70.4.23

CAN Rx FIFO 0 acknowledge register (FDCAN_RXF0A)
Address offset: 0x0094
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

F0AI[2:0]
rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 F0AI[2:0]: Rx FIFO 0 acknowledge index
After the host has read a message or a sequence of messages from Rx FIFO 0, it has to
write the buffer index of the last element read from Rx FIFO 0 to F0AI[2:0]. This sets the Rx
FIFO 0 get index (F0GI[1:0] of FDCAN_RXF0S) to F0AI[2:0] + 1 and updates the FIFO 0 fill
level (F0FL[3:0] FDCAN_RXF0S).

70.4.24

FDCAN Rx FIFO 1 status register (FDCAN_RXF1S)
Address offset: 0x0098
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

Res.

Res.

Res.

Res.

Res.

Res.

RF1L

F1F

Res.

Res.

Res.

Res.

Res.

Res.

r

r
8

15

14

13

12

11

10

9

Res.

Res.

Res.

Res.

Res.

Res.

F1GI[1:0]
r

7

6

5

4

Res.

Res.

Res.

Res.

r

3

2

17

16

F1PI[1:0]
r

r

1

0

F1FL[3:0]
r

r

r

r

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 RF1L: Rx FIFO 1 message lost
This bit is a copy of the RF1L interrupt flag of the FDCAN_IR register. When RF1L is cleared,
this bit is also cleared.
0: No Rx FIFO 1 message lost
1: Rx FIFO 1 message lost, also set after write attempt to Rx FIFO 1 of size 0
Bit 24 F1F: Rx FIFO 1 full
0: Rx FIFO 1 not full
1: Rx FIFO 1 full
Bits 23:18 Reserved, must be kept at reset value.
Bits 17:16 F1PI[1:0]: Rx FIFO 1 put index
Rx FIFO 1 write index pointer.
Range: 0 to 2.
Bits 15:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Bits 9:8 F1GI[1:0]: Rx FIFO 1 get index
Rx FIFO 1 read index pointer.
Range: 0 to 2.
Bits 7:4 Reserved, must be kept at reset value.
Bits 3:0 F1FL[3:0]: Rx FIFO 1 fill level
Number of elements stored in Rx FIFO 1.
Range: 0 to 3.

70.4.25

FDCAN Rx FIFO 1 acknowledge register (FDCAN_RXF1A)
Address offset: 0x009C
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

3

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

F1AI[2:0]
rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 F1AI[2:0]: Rx FIFO 1 acknowledge index
After the host has read a message or a sequence of messages from Rx FIFO 1, it has to
write the buffer index of the last element read from Rx FIFO 1 to F1AI[2:0]. This sets the Rx
FIFO 1 get index (F1GI[1:0] of FDCAN_RXF1S) to F1AI[2:0] + 1 and updates the FIFO 1 fill
level (F1FL[3:0] FDCAN_RXF1S).

70.4.26

FDCAN Tx buffer configuration register (FDCAN_TXBC)
Address offset: 0x00C0
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

TFQM

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

Res.

Res.

Res.

rw

Bits 31:25 Reserved, must be kept at reset value.
Bit 24 TFQM: Tx FIFO/queue mode
0: Tx FIFO operation
1: Tx queue operation.
This bit is write-protected (P), which means that write access is possible only when the CCE
and INIT bits of the FDCAN_CCCR register are both set.
Bits 23:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

70.4.27

FDCAN Tx FIFO/queue status register (FDCAN_TXFQS)
The Tx FIFO/queue status is related to the pending Tx requests listed in the
FDCAN_TXBRP register. Therefore, the effect of add/cancellation requests can be delayed
due to a running Tx scan (FDCAN_TXBRP not yet updated).
Address offset: 0x00C4
Reset value: 0x0000 0003

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

TFQF

Res.

Res.

Res.

TFQPI[1:0]

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

r

TFGI[1:0]
r

r

16

r

r

1

0

TFFL[2:0]
r

r

r

Bits 31:22 Reserved, must be kept at reset value.
Bit 21 TFQF: Tx FIFO/queue full
0: Tx FIFO/queue not full
1: Tx FIFO/queue full
Bits 20:18 Reserved, must be kept at reset value.
Bits 17:16 TFQPI[1:0]: Tx FIFO/queue put index
Tx FIFO/queue write index pointer, range 0 to 3
Bits 15:10 Reserved, must be kept at reset value.
Bits 9:8 TFGI[1:0]: Tx FIFO get index
Tx FIFO read index pointer, range 0 to 3. Read as 0 when Tx queue operation is configured
(TFQM = 1 in FDCAN_TXBC)
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 TFFL[2:0]: Tx FIFO free level
Number of consecutive free Tx FIFO elements starting from TFGI, range 0 to 3. Read as 0
when Tx queue operation is configured (TFQM = 1 in FDCAN_TXBC).

70.4.28

FDCAN Tx buffer request pending register (FDCAN_TXBRP)
Address offset: 0x00C8
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

3

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

TRP[2:0]
r

r

r

Bits 31:3 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Bits 2:0 TRP[2:0]: Transmission request pending
Each Tx buffer has its own transmission request pending bit. The bits are set via the
FDCAN_TXBAR register. The bits are cleared after a requested transmission has completed
or has been canceled via the FDCAN_TXBCR register.
After the FDCAN_TXBRP bit has been set, a Tx scan is started to check for the pending Tx
request with the highest priority (Tx buffer with lowest message ID).
A cancellation request resets the corresponding transmission request pending bit of the
FDCAN_TXBRP register. In case a transmission has already been started when a
cancellation is requested, this is done at the end of the transmission, regardless whether the
transmission was successful or not. The cancellation request bits are directly cleared after the
corresponding FDCAN_TXBRP bit has been cleared.
After a cancellation has been requested, a finished cancellation is signaled via the
FDCAN_TXBCF in the following cases:
–
after successful transmission together with the corresponding TXBTO bit
–
when the transmission has not yet been started at the point of cancellation
–
when the transmission has been aborted due to lost arbitration
–
when an error occurred during frame transmission
In DAR mode, all transmissions are automatically canceled if they are not successful. The
corresponding FDCAN_TXBCF bit is set for all unsuccessful transmissions.
0: No transmission request pending
1: Transmission request pending

Note:

FDCAN_TXBRP bits set while a Tx scan is in progress are not considered during this
particular Tx scan. In case a cancellation is requested for such a Tx buffer, this add request
is canceled immediately. The corresponding FDCAN_TXBRP bit is cleared.

70.4.29

FDCAN Tx buffer add request register (FDCAN_TXBAR)
Address offset: 0x00CC
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

3

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

AR[2:0]
rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 AR[2:0]: Add request
Each Tx buffer has its own add request bit. Writing a 1 sets the corresponding add request
bit; writing a 0 has no impact. This enables the host to set transmission requests for multiple
Tx buffers with one write to FDCAN_TXBAR. When no Tx scan is running, the bits are
cleared immediately, else the bits remain set until the Tx scan process has completed.
0: No transmission request added
1: Transmission requested added.

Note:

<!-- pagebreak -->

If an add request is applied for a Tx buffer with pending transmission request
(corresponding FDCAN_TXBRP bit already set), the request is ignored.

RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

70.4.30

FDCAN Tx buffer cancellation request register (FDCAN_TXBCR)
Address offset: 0x00D0
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

CR[2:0]
rw

rw

rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 CR[2:0]: Cancellation request
Each Tx buffer has its own cancellation request bit. Writing a 1 sets the corresponding CR bit;
writing a 0 has no impact.
This enables the host to set cancellation requests for multiple Tx buffers with one write to
FDCAN_TXBCR. The bits remain set until the corresponding FDCAN_TXBRP bit is cleared.
0: No cancellation pending
1: Cancellation pending

70.4.31

FDCAN Tx buffer transmission occurred register (FDCAN_TXBTO)
Address offset: 0x00D4
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

TO[2:0]
r

r

r

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 TO[2:0]: Transmission occurred.
Each Tx buffer has its own TO bit. The bits are set when the corresponding FDCAN_TXBRP
bit is cleared after a successful transmission. The bits are cleared when a new transmission
is requested by writing a 1 to the corresponding bit of register FDCAN_TXBAR.
0: No transmission occurred
1: Transmission occurred

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

70.4.32

RM0456

FDCAN Tx buffer cancellation finished register (FDCAN_TXBCF)
Address offset: 0x00D8
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

CF[2:0]
r

r

r

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 CF[2:0]: Cancellation finished
Each Tx buffer has its own CF bit. The bits are set when the corresponding FDCAN_TXBRP
bit is cleared after a cancellation was requested via FDCAN_TXBCR. In case the
corresponding FDCAN_TXBRP bit was not set at the point of cancellation, CF is set
immediately. The bits are cleared when a new transmission is requested by writing a 1 to the
corresponding bit of the FDCAN_TXBAR register.
0: No transmit buffer cancellation
1: Transmit buffer cancellation finished

70.4.33

FDCAN Tx buffer transmission interrupt enable register
(FDCAN_TXBTIE)
Address offset: 0x00DC
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

TIE[2:0]
rw

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 TIE[2:0]: Transmission interrupt enable
Each Tx buffer has its own TIE bit.
0: Transmission interrupt disabled
1: Transmission interrupt enable

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

RM0456

FD controller area network (FDCAN)

70.4.34

FDCAN Tx buffer cancellation finished interrupt enable register
(FDCAN_TXBCIE)
Address offset: 0x00E0
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

3

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

CFIE[2:0]
rw

rw

rw

16

Bits 31:3 Reserved, must be kept at reset value.
Bits 2:0 CFIE[2:0]: Cancellation finished interrupt enable.
Each Tx buffer has its own CFIE bit.
0: Cancellation finished interrupt disabled
1: Cancellation finished interrupt enabled

70.4.35

FDCAN Tx event FIFO status register (FDCAN_TXEFS)
Address offset: 0x00E4
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

Res.

Res.

Res.

Res.

Res.

Res.

TEFL

EFF

Res.

Res.

Res.

Res.

Res.

Res.

EFPI[1:0]

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

4

3

2

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

EFGI[1:0]
r

r

r

r

1

0

EFFL[2:0]
r

r

r

Bits 31:26 Reserved, must be kept at reset value.
Bit 25 TEFL: Tx event FIFO element lost
This bit is a copy of the TEFL interrupt flag of the FDCAN_IR. When TEFL is cleared, this bit
is also cleared.
0 No Tx event FIFO element lost
1 Tx event FIFO element lost, also set after write attempt to Tx event FIFO of size 0.
Bit 24 EFF: Event FIFO full
0: Tx event FIFO not full
1: Tx event FIFO full
Bits 23:18 Reserved, must be kept at reset value.
Bits 17:16 EFPI[1:0]: Event FIFO put index
Tx event FIFO write index pointer.
Range: 0 to 3.
Bits 15:10 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

RM0456

Bits 9:8 EFGI[1:0]: Event FIFO get index
Tx event FIFO read index pointer.
Range: 0 to 3.
Bits 7:3 Reserved, must be kept at reset value.
Bits 2:0 EFFL[2:0]: Event FIFO fill level
Number of elements stored in Tx event FIFO.
Range: 0 to 3.

70.4.36

FDCAN Tx event FIFO acknowledge register (FDCAN_TXEFA)
Address offset: 0x00E8
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

3

2

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

EFAI[1:0]
rw

rw

Bits 31:2 Reserved, must be kept at reset value.
Bits 1:0 EFAI[1:0]: Event FIFO acknowledge index
After the host has read an element or a sequence of elements from the Tx event FIFO, it has
to write the index of the last element read from Tx event FIFO to EFAI[1:0]. This sets the Tx
event FIFO get index (EFGI[1:0] of FDCAN_TXEFS) to EFAI[1:0] + 1 and updates the FIFO 0
fill level (EFFL[2:0] of FDCAN_TXEFS).

70.4.37

FDCAN CFG clock divider register (FDCAN_CKDIV)
Address offset: 0x0100
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

PDIV[3:0]
rw

Bits 31:4 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

rw

RM0456

FD controller area network (FDCAN)

Bits 3:0 PDIV[3:0]: input clock divider
The CAN kernel clock can be divided prior to be used by the CAN subsystem. The rate must
be computed using the divider output clock.
0000: Divide by 1
0001: Divide by 2
0010: Divide by 4
0011: Divide by 6
0100: Divide by 8
0101: Divide by 10
0110: Divide by 12
0111: Divide by 14
1000: Divide by 16
1001: Divide by 18
1010: Divide by 20
1011: Divide by 22
1100: Divide by 24
1101: Divide by 26
1110: Divide by 28
1111: Divide by 30
This bitfield is write-protected (P): which means that write access is possible only when the
CCE bit of the FDCAN_CCCR register is set.
Note: The clock divider is common to all FDCAN instances. Only FDCAN1 instance has
FDCAN_CKDIV register, which changes clock divider for all instances.

70.4.38

FDCAN register map

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

1

1

0

1

0

0

0

1

0

1

0

0

0

0

1

1

0

0

1

0

0

0

0

1

0

1

0

1

0

0

0

1

1

0

0

1

1

Res.

Res.

Res.

RX

TX[1:0]

LBCK

Res.

Res.

Res.

Res.

12

1

Res.

13

1

Res.

14

1

0

0

1

1

0

0

0

0

0

FDCAN_ENDN

DAY[7:0]

MON[7:0]

YEAR[3:0]

1

Res.

0

15

1

Res

0

16

1

17

1

18

1

19

1

20

22

0

21

23

0

SUBSTEP [3:0]

0

24

0

25

26

0

STEP[3:0]

27

0

28

0

FDCAN_CREL
0x0000

Reset value

29

30

Register name

REL[3:0]

Offset

31

Table 731. FDCAN register map and reset values

ETV[31:0]

0x0004
0

0

0

0

1

1

1

0

1

1

0

0

1

0

0

0

0

Res.

Res.

Res.

Res.

Res.

Reset value

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

FDCAN_RWD
0x0014

Res.

Reset value

0

WDV[7:0]
0

RM0456 Rev 6

DSJW[3:0]

0

DTSEG2[3:0]

0

DTSEG1[4:0]

0

Res.

0

Res.

Res.

0

Res.

Res.

Res.

0

Res.

DBRP[4:0]

TDC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FDCAN_TEST
0x0010

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

FDCAN_DBTP
0x000C

1

Reserved

Res.

Reserved

Res.

0x0008

1

Res.

Reset value

0

0

0

0

0
WDC[7:0]

0

0

0

0

0

0

0

0

0

<!-- pagebreak -->

3086

0x0058

FDCAN_ILS

Reset value

0

<!-- pagebreak -->

PEDE

PEAE

WDIE

BOE

EWE

EPE

ELOE

TOOE

MRAFE

TSWE

TEFLE

TEFFE

TEFNE

TFEE

TCFE

TCE

HPME

RF1LE

RF1FE

RF1NE

RF0LE

RF0FE

RF0NE

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

Res.

Res.

Res.

PERR

BERR

MISC

TFERR

SMSG

RXFIFO1

RXFIFO0

Reset value
ARA
PED
PEA
WDI
BO
EW
EP
ELO
TOO
MRAF
TSW
TEFL
TEFF
TEFN
TFE
TCF
TC
HPM
RF1L
RF1F
RF1N
RF0L
RF0F
RF0N

Reset value

ARAE

Res.
Res.
Res.

0
0
0
0
0

FDCAN_TSCV
Res.
Res.
Res.
Res.
Res.

Reset value
0
0
0
0
0
0
0

Reset value
1
1
1
1
1
1
1

FDCAN_TOCV
Res.
Res.
Res.
Res.

0

0
0
0
0

1
1
1
1
1
1
1
1
1

Reset value

Reserved

CEL[7:0]

0
0
0

0
0
0
0

Reserved

RM0456 Rev 6
Res.
Res.
Res.

0
0
0
0
0
0
0

Res.
Res.
Res.
Res.
Res.
Res.
Res.

0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0

TOC[15:0]

1

Reset value
1
1
1
1

0
0
0
0
0

0
0
0
0

1

0

0

1

0

0

0
0
0
0
0
0

1

0

0

TDCO[6:0]

1
1

Reserved

REC[6:0]
TEC[7:0]

0
0
0

0
0
0
0

0
0
0
0
0
0

1
1
1
1
1
1

0
0
0
0

0
1

0

ETOC

1

TSC[15:0]

TSS[1:0]

0
TOS[1:0]

0

0

LEC[2;0]

0
Res.

1
Res.

0

Res.

0
Res.

0

Res.

0
Res.

0

Res.

Res.

NTSEG1[7:0]

Res.

0

Res.

0x001C

Res.

TCP[3:0]

17

Res.

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

BRSE
FDOE
TEST
DAR
MON
CSR
CSA
ASM
CCE
INT

11

18

Res.

10

19

Res.

Res.

20

Res.

12

21

Res.

Res.

22

Res.

13

23

Res.

PXHD

24

Res.

14

25

Res.

EFBI

26

Res.

15

27

Res.

TXP

28

Res.

16

29

Res.

NISO

30

Res.

Res.

31

Res.

0

Res.

0

Res.

Res.
0

Res.

Res.
0

Res.

Res.
0

Res.

Res.
0

Res.

0

Res.

Res.
0

Res.

0

Res.

Res.

0

Res.

0

Res.

Res.

0

Res.

0

0

Res.

Res.

0

Res.

0

0

Res.

Res.

0

Res.

0

0

ACT[1:0]

Res.

0

0

EP

Res.

0

0

EW

Res.

0

0

BO

Res.

0

0

Res.

Res.

0

Res.

NBRP[8:0]

DLEC[2:0]

Res.

Reset value
0

RESI

FDCAN_TSCC
0

RBRSRESI1

0

REDL

0

Res.

Reset value

PXE

0

RP

0

Res.

FDCAN_CCCR

Res.

0

Res.

Register name

Res.

1

Res.

1

Res.

0

Res.

0

Res.

0

TOP[15:0]

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.
NSJW[6:0]

Res.

TDCV[6:0]

0

Res.

Reset value
0

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

FDCAN_TOCC

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0x0028

Res.

Res.

FDCAN_IE

Res.

0x0054

Res.

FDCAN_IR

Res.

0x0050

Res.

0x004C

Res.

FDCAN_TDCR

Res.

0x0048

Res.

FDCAN_PSR
Res.

FDCAN_NBTP

Res.

0x0044
Res.

FDCAN_ECR

Res.

0x0040

Res.

0x00300x003C

Res.

0x002C

Res.

0x0024

Res.

0x0020

Res.

0x0018

Res.

Offset

Res.

FD controller area network (FDCAN)
RM0456

Table 731. FDCAN register map and reset values (continued)

0
0
0
0
0
0
0
0
0
1

NTSEG2[6:0]
1
1

0
0
0
0
0
0
0

0
0
0

1
1

TDCF[6:0]

0
0
0
0
0
0
0
0

Reserved

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0x00C8

FDCAN
_TXBRP

0

RM0456 Rev 6

0

0

0

1

1

TRP0

TFFL[2:0]

Reset value

TRP1

Res.

Res.

0
0

Res.

Res.

F0FL[3:0]

Res.
0

0

0

Reserved
0
0

Res.

F1FL[3:0]

Res.

Res.

RRFS
RRFE

0
0
0
0

1
1
1
1
1
1

Res.
Res.
BIDX[2:0]

0
ANFE[1:0]

0

Res.

ANFS[1:0]

Res.

Reset value

Res.

0
0

Res.

Res.

Reset value

Res.

Res.

Res.

0
Res.

0

Res.

0

Res.

Res.

0

1

Res.

Res.

1

Res.

1

Res.

MSI[1:0]

F1OM

Res.
F0OM
F0GI[1:0]

Res.

Res.
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

LSS[4:0]

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

EIDM[28:0]
0

Res.

Res.

Res.

0

Res.

0

Res.

F1GI[1:0]

0

Res.

0

Res.

Res.

FIDX[4:0]

FLST

0

Res.

Res.

Res.

0

Res.

Res.

Res.

0

1

Res.

Res.

Res.

1

TFGI[1:0]

Res.

Res.

Res.

Res.

1

Res.

0

Res.

1

Res.

1

Res.

Res.

Res.

1

Res.

Res.

1

Res.

Res.

Res.

1

Res.

Res.

Res.

1

Res.

0

Res.

Res.

Res.

Res.

Res.

1

Res.

Res.

Res.

Res.

1

Res.

Res.

2
1
0

EINT0

Res.

EINT1

12

Res.

3

13

Res.

Res.

14

Res.

4

15

Res.

Res.

16

Res.

5

17

Res.

Res.

18

Res.

6

19

Res.

Res.

20

Res.

7

21

Res.

Res.

22

Res.

8

23

Res.

Res.

24

Res.

9

25

Res.

Res.

26

Res.

Res.

27

Res.

11

28

Res.

10

29

Res.

Res.

30

Res.

Res.

31

FDCAN_ILE

Res.

Res.

Res.

Res.

F0PI[1:0]

1

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

Res.

Register name

TRP2

Reset value
Res.

F1PI[1:0]
0

Res.

0

Res.

0

Res.

Res.

Res.

Reset value

Res.

Reserved

Res.

Res.

Res.

Res.

1

Res.

0

Res.

TFQPI[1:0]

0

Res.

Res.

Res.

Res.

Res.

1

Res.

LSE[3:0].

Res.

Res.

Res.

Res.

Reserved

Res.

0

Res.

0
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

1

Res.

Res.

0

Res.

Res.

TFQF

Res.

Res.

F0F

Res.

1

Res.

Res.

0

Res.

Reset value

Res.

Res.

Res.

0

Res.

Res.

0

Res.

F1F

0

Res.
0

Res.

TFQM

Reset value

Res.

RF0L

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

RF1L

Res.

Res.

Res.

0

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

FDCAN
_TXFQS
Res.

Reset value

Res.

0x00C4
FDCAN
_TXBC
Res.

Reset value

Res.

0x00C0
Res.

0x00A00x00BC
FDCAN
_RXF1A

Res.

0x009C
FDCAN
_RXF1S

Res.

0x0098
FDCAN
_RXF0A

Res.

0x0094
FDCAN
_RXF0S

Res.

FDCAN_HPMS

Res.

0x0088

Res.

0x0090
FDCAN
_XIDAM

Res.

0x0084
FDCAN
_RXGFC

Res.

0x0080

Res.

0x00600x007C

Res.

0x005C

Res.

Offset

Res.

RM0456
FD controller area network (FDCAN)

Table 731. FDCAN register map and reset values (continued)

0
0

Reserved

F0AI[2:0]

0
0

0
0

F1AI[2:0]

0
0

0
0

0

0

0

<!-- pagebreak -->

3086

0x0100
FDCAN
_CKDIV

<!-- pagebreak -->

RM0456 Rev 6

Reset value

Refer to Section 2.3 for the register boundary addresses.
Res.

Res.

Res.

Res.

0

0

Reset value

0

EFAI[1:0]

TIE1
TIE0

0
0

CFIE0

Reset value
0
CFIE1

Reset value
CF1
CF0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CF2

Reset value

TIE2

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

TO1
TO0

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

Res.

Res.

Res.

Res.

TO2

Reset value

CFIE2

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

CR1
CR0

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

CR2

Reset value

EFFL[2:0]

Res.

Res.

Res.

Res.

Res.

EFG[1:0]

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

Res.

Res.

Res.

Res.

Res.

Res.

Reset value

Res.

Res.

Res.

Res.

Res.

Res.

0

Res.

Res.
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

2
1
0

AR0

Res.

AR1

12

Res.

3

13

Res.

AR2

14

Res.

4

15

Res.

Res.

16

Res.

5

17

Res.

Res.

18

Res.

6

19

Res.

Res.

20

Res.

7

21

Res.

Res.

22

Res.

8

23

Res.

Res.

24

Res.

9

25

Res.

Res.

26

Res.

Res.

27

Res.

11

28

Res.

10

29

Res.

Res.

30

Res.

Res.

31

FDCAN
_TXBAR

Res.

Res.

Res.

Res.

Res.

EFPI[1:0]

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

Register name

Res.

Res.

Res.

Res.

Res.
Res.

0

Res.

Res.

Res.

Res.

Res.

Res.

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

EFF

0

Res.

Res.

TEFL

Res.

Res.

Res.

0

Res.

Res.

Res.

Reset value

Res.

FDCAN
_TXEFA

Res.

0x00E8

Res.

FDCAN
_TXEFS

Res.

0x00E4

Res.

FDCAN
_TXBCIE

Res.

0x00E0

Res.

FDCAN
_TXBTIE

Res.

0x00DC
Res.

FDCAN
_TXBCF

Res.

0x00D8

Res.

FDCAN
_TXBTO

Res.

0x00D4

Res.

FDCAN
_TXBCR

Res.

0x00D0

Res.

0x00CC

Res.

Offset

Res.

FD controller area network (FDCAN)
RM0456

Table 731. FDCAN register map and reset values (continued)

0
0
0

0
0
0

0
0
0

0
0
0

0
0
0

0

0

0

0

0

PDIV[3:0]

0

RM0456

71

Universal serial bus full-speed host/device interface (USB)

Universal serial bus full-speed host/device interface
(USB)
This section applies to STM32U535/545 devices only.

71.1

Introduction
The USB peripheral implements an interface between a full-speed USB 2.0 bus and the
APB2 bus.
USB suspend/resume are supported, which permits to stop the device clocks for low-power
consumption.

71.2

71.3

USB main features
•

USB specification version 2.0 full-speed compliant

•

Supports both Host and Device modes

•

Configurable number of endpoints from 1 to 8

•

Dedicated packet buffer memory (SRAM) of 2048 bytes

•

Cyclic redundancy check (CRC) generation/checking, Non-return-to-zero Inverted
(NRZI) encoding/decoding and bit-stuffing

•

Isochronous transfers support

•

Double-buffered bulk/isochronous endpoint/channel support

•

USB Suspend/Resume operations

•

Frame locked clock pulse generation

•

USB 2.0 Link Power Management support (Device mode only)

•

Battery Charging Specification Revision 1.2 support (Device mode only)

•

USB connect / disconnect capability (controllable embedded pull-up resistor on
USB_DP line)

USB implementation
Table 732 describes the USB implementation in the devices.
Table 732. STMU535/545 USB implementation
USB features(1)

USB

Host mode

X

Number of endpoints

8

Size of dedicated packet buffer memory SRAM

2048 bytes

Dedicated packet buffer memory SRAM access scheme

32 bits

USB 2.0 Link Power Management (LPM) support in device

X

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Table 732. STMU535/545 USB implementation (continued)
USB features(1)

USB

Battery Charging Detection (BCD) support for device

X

Embedded pull-up resistor on USB_DP line

X

1. X= supported

71.4

USB functional description

71.4.1

USB block diagram
Figure 891 shows the block diagram of the USB peripheral.
Figure 891. USB peripheral block diagram
USB_SOF

USB_DP

USB_DM

USB_NOE

USB PHY

USB_ker_ck (48 MHz)
Embedded
pull-up

Analog
transceiver

PCLK

BCD

Serial interface engine (SIE)

Suspend timer

RX-TX
Control

Control
registers and logic

Clock recovery
Endpoint/
channel
selection

Interrupt
registers and logic

Host frame
scheduler

Packet buffer
interface

Endpoint/channel
registers

Arbiter

Packet buffer
memory

Endpoint/channel
registers

Register
Register
mapper
mapper

Interrupt
mapper

APB wrapper

APB interface

USB_ker_ck

<!-- pagebreak -->

PCLK

APB bus

RM0456 Rev 6

IRQs to NVIC

MSv73152V2

RM0456

71.4.2

Universal serial bus full-speed host/device interface (USB)

USB pins and internal signals
Table 733. USB input/output pins
Pin name

71.4.3

Pin type

Description

USB_DP

Digital input/output

D+ line

USB_DM

Digital input/output

D- line

USB_NOE

Digital output

NOE (output enable of data
lines)

USB_SOF

Digital output

SOF (start of frame indicator)

USB reset and clocks
A single reset is present on USB. The RCC allows a reset to be forced by software.
There are two clocks:

71.4.4

•

PCLK for the APB bus interface and registers.

•

USB_ker_ck (48 MHz) for the main protocol logic including notably the serial interface
engine (SIE), see USB_ker_ck clock domain in block diagram.

General description and Device mode functionality
The USB peripheral provides a USB-compliant connection between the function
implemented by the microcontroller and an external USB function which can be a host PC
but also a USB Device. Data transfer between the external USB host or device and the
system memory occurs through a dedicated packet buffer memory accessed directly by the
USB peripheral. This dedicated memory size is 2048 bytes, and up to 16 mono-directional
or 8 bidirectional endpoints can be used. The USB peripheral interfaces with the external
USB Host or Device, detecting token packets, handling data transmission/reception, and
processing handshake packets as required by the USB standard. Transaction formatting is
performed by the hardware, including CRC generation and checking.
Each endpoint/channel is associated with a buffer description block indicating where the
endpoint/channel-related memory area is located, how large it is or how many bytes must
be transmitted. When a token for a valid function/endpoint pair is recognized by the USB
peripheral, the related data transfer (if required and if the endpoint/channel is configured)
takes place. The data buffered by the USB peripheral are loaded in an internal 16-bit
register and memory access to the dedicated buffer is performed. When all the data have
been transferred, if needed, the proper handshake packet over the USB is generated or
expected according to the direction of the transfer.
At the end of the transaction, an endpoint/channel-specific interrupt is generated, reading
status registers and/or using different interrupt response routines. The microcontroller can
determine:
•

which endpoint/channel has to be served,

•

which type of transaction took place, if errors occurred (bit stuffing, format, CRC,
protocol, missing ACK, over/underrun, etc.).

Special support is offered to isochronous transfers and high throughput bulk transfers,
implementing a double buffer usage, which permits to always have an available buffer for
the USB peripheral while the microcontroller uses the other one.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

A special bit THR512 in register USB_ISTR allows notification of 512 bytes being received
in (or transmitted from) the buffer. This bit must be used for long ISO packets (from 512 to
1023 bytes) as it facilitates early start or read/write of data. In this way, the first 512 bytes
can be handled by software while avoiding use of double buffer mode. This bit works when
only one ISO endpoint is configured.
The unit can be placed in low-power mode (SUSPEND mode), by writing in the control
register, whenever required. At this time, all static power dissipation is avoided, and the USB
clock can be slowed down or stopped. The detection of activity at the USB inputs, while in
low-power mode, wakes the device up asynchronously. A special interrupt source can be
connected directly to a wake-up line to permit the system to immediately restart the normal
clock generation and/or support direct clock start/stop.

Host mode and specific functionality
A single bit, HOST, in register USB_CNTR permits Host mode to be activated. Host mode
functionality permits the USB to talk to a remote peripheral. Supported functionality is
aligned to Device mode and uses the same register structures to manage the buffers. The
same number of endpoints can be supported in Host mode, however in Host mode the
terminology “channel” is preferred, as each channel is in reality a combination of the
connected device and the endpoint on that device. The basic mechanisms for packet
transmission and reception are the same as those supported in Device mode.
When operating in Host mode, the USB is in charge of the bus and in order to do this must
issue transaction requests corresponding to active periodic and non-periodic endpoints. A
host frame scheduler assures efficient use of the frame. Connection to hubs is supported.
Connection to low speed devices is supported, both with a direct connection and through a
hub.
Double-buffered mode, as previously described in Device mode, is also supported in Host
mode, in both bulk and isochronous channels. The THR512 functionality is also supported
(but as in Device mode) only for ISO traffic.
Note:

Unlike in Device mode, where there is a detection of battery charging capability (in order to
facilitate fast charging), there is no integrated support in Host mode to present battery
charging capability (CDP or DCP cases in the standard), the host port is always presented
as a default standard data port (SDP).
For LPM (link power management) this feature is not supported in Host mode.

71.4.5

Description of USB blocks used in both Device and Host modes
The USB peripheral implements all the features related to USB interfacing, which include
the following blocks:

<!-- pagebreak -->

•

USB physical interface (USB PHY): this block is maintaining the electrical interface to
an external USB host. It contains the differential analog transceiver itself, controllable
embedded pull-up resistor (connected to USB_DP line) and support for battery
charging detection (BCD), multiplexed on same USB_DP and USB_DM lines. The
output enable control signal of the analog transceiver (active low) is provided externally
on USB_NOE. It can be used to drive some activity LED or to provide information about
the actual communication direction to some other circuitry.

•

Serial interface engine (SIE): the functions of this block include: synchronization
pattern recognition, bit-stuffing, CRC generation and checking, PID
verification/generation, and handshake evaluation. It must interface with the USB
transceivers and uses the virtual buffers provided by the packet buffer interface for

RM0456 Rev 6

RM0456

Universal serial bus full-speed host/device interface (USB)
local data storage. This unit also generates signals according to USB peripheral
events, such as start of frame (SOF), USB_Reset, data errors etc. and to endpoint
related events like end of transmission or correct reception of a packet; these signals
are then used to generate interrupts.

Note:

•

Timer: this block generates a start-of-frame locked clock pulse and detects a global
suspend (from the host) when no traffic has been received for 3 ms.

•

Packet buffer interface: this block manages the local memory implementing a set of
buffers in a flexible way, both for transmission and reception. It can choose the proper
buffer according to requests coming from the SIE and locate them in the memory
addresses pointed by the endpoint/channel registers. It increments the address after
each exchanged byte until the end of packet, keeping track of the number of
exchanged bytes and preventing the buffer to overrun the maximum capacity.

•

Endpoint/channel-related registers: each endpoint/channel has an associated register
containing the endpoint/channel type and its current status. For monodirectional/single-buffer endpoints, a single register can be used to implement two
distinct endpoints. The number of registers is 8, allowing up to 16 monodirectional/single-buffer or up to 7 double-buffer endpoints in any combination. For
example the USB peripheral can be programmed to have 4 double buffer endpoints
and 8 single-buffer/mono-directional endpoints.

•

Control registers: these are the registers containing information about the status of the
whole USB peripheral and used to force some USB events, such as resume and
power-down.

•

Interrupt registers: these contain the interrupt masks and a record of the events. They
can be used to inquire an interrupt reason, the interrupt status or to clear the status of a
pending interrupt.

* Endpoint/channel 0 is always used for control transfer in single-buffer mode.
The USB peripheral is connected to the APB2 bus through an APB2 interface, containing
the following blocks:

71.4.6

•

Packet memory: this is the local memory that physically contains the packet buffers. It
can be used by the packet buffer interface, which creates the data structure and can be
accessed directly by the application software. The size of the packet memory is
2048 bytes, structured as 512 words of 32 bits.

•

Arbiter: this block accepts memory requests coming from the APB2 bus and from the
USB interface. It resolves the conflicts by giving priority to APB2 accesses, while
always reserving half of the memory bandwidth to complete all USB transfers. This
time-duplex scheme implements a virtual dual-port SRAM that allows memory access,
while an USB transaction is happening. Multiword APB2 transfers of any length are
also allowed by this scheme.

•

Register mapper: this block collects the various byte-wide and bit-wide registers of the
USB peripheral in a structured 32-bit wide word set addressed by the APB2.

•

APB2 wrapper: this provides an interface to the APB2 for the memory and register. It
also maps the whole USB peripheral in the APB2 address space.

•

Interrupt mapper: this block is used to select how the possible USB events can
generate interrupts and map them to the NVIC.

Description of host frame scheduler (HFS) specific to Host mode
The host frame scheduler is the hardware machine in charge to submit host channel
requests on the bus according to the USB priority order and bandwidth access rules.

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

Host channels are divided in two categories:
–

Periodic channels: isochronous and interrupt traffic types. With guaranteed
bandwith access.

–

Non-periodic channels: bulk and control traffic types. With best effort service.

The host frame scheduler organizes the full-speed frame in 3 sequential windows
–

Periodic service window

–

Non-periodic service window

–

Black security window

At the start of a new frame, the host scheduler:

71.5

1.

First considers all periodic channels which were active (STAT bits VALID) at the start of
frame

2.

Executes single round of service of periodic channels, the periodic service window, in
hardware priority order from CH#1 to CH#8. For bidirectional channels it executes the
OUT direction first

3.

When the periodic round is finished, HFS closes the periodic service window and stops
servicing periodic traffic even if some periodic channel was re-enabled or some new
channel was enabled after the SOF.

4.

Starts servicing all non-periodic channels which are currently active (STAT bits VALID)
in hardware priority order from CH#1 to CH#8. For bidirectional channels it executes
the OUT direction first.

5.

Executes multiple round-robin service cycles of non-periodic channels until almost the
end of frame

6.

Non-periodic traffic can be requested at any time and is serviced by HFS with best
effort latency, with the exception of a black security window at the end of the frame
where new injected requests are directly postponed to the next frame to avoid babbles.
This is also true for pending transactions which have not been serviced ahead of the
security window.

Programming considerations for Device and Host modes
In the following sections, the expected interactions between the USB peripheral and the
application program are described, in order to ease application software development.

71.5.1

Generic USB Device programming
This part describes the main tasks required of the application software in order to obtain
USB compliant behavior. The actions related to the most general USB events are taken into
account and paragraphs are dedicated to the special cases of double-buffered endpoints
and isochronous transfers. Apart from system reset, an action is always initiated by the USB
peripheral, driven by one of the USB events described below.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

71.5.2

Universal serial bus full-speed host/device interface (USB)

System and power-on reset
Upon system and power-on reset, the first operation the application software must perform
is to provide all required clock signals to the USB peripheral and subsequently de-assert its
reset signal so to be able to access its registers. The whole initialization sequence is
hereafter described.
As a first step application software needs to activate register macrocell clock and de-assert
macrocell specific reset signal using related control bits provided by device clock
management logic.
After that, the analog part of the device related to the USB transceiver must be switched on
using the PDWN bit in CNTR register, which requires a special handling. This bit is intended
to switch on the internal voltage references that supply the port transceiver. This circuit has
a defined startup time (tSTARTUP specified in the datasheet) during which the behavior of the
USB transceiver is not defined. It is thus necessary to wait this time, after setting the PDWN
bit in the CNTR register, before removing the reset condition on the USB part (by clearing
the USBRST bit in the CNTR register). Clearing the ISTR register removes any spurious
pending interrupt before any other macrocell operation is enabled.
At system reset, the microcontroller must initialize all required registers and the packet
buffer description table, to make the USB peripheral able to properly generate interrupts and
data transfers. All registers not specific to any endpoint/channel must be initialized
according to the needs of application software (choice of enabled interrupts, chosen
address of packet buffers, etc.). Then the process continues as for the USB reset case (see
further paragraph).

USB bus reset (RST_DCON interrupt) in Device mode
When this event occurs, the USB peripheral is put in the same conditions it is left by the
system reset after the initialization described in the previous paragraph: communication is
disabled in all endpoint registers (the USB peripheral does not respond to any packet). As a
response to the USB reset event, the USB function must be enabled, having as USB
address 0, implementing only the default control endpoint (endpoint address is 0 too). This
is accomplished by setting the enable function (EF) bit of the USB_DADDR register and
initializing the CHEP0R register and its related packet buffers accordingly. During USB
enumeration process, the host assigns a unique address to this device, which must be
written in the ADD[6:0] bits of the USB_DADDR register, and configures any other
necessary endpoint.
When a RST_DCON interrupt is received, the application software is responsible to enable
again the default endpoint of USB function 0 within 10 ms from the end of the reset
sequence which triggered the interrupt.

USB bus reset in Host mode
In Host mode a bus reset is activated by setting the USBRST bit of the USB_CNTR register.
It must subsequently be cleared by software once the minimum active reset time from the
standard has been respected.

Structure and usage of packet buffers
Each bidirectional endpoint can receive or transmit data over the bus. The received data is
stored in a dedicated memory buffer reserved for that endpoint, while another memory
buffer contains the data to be transmitted by the endpoint. Access to this memory is
performed by the packet buffer interface block, which delivers a memory access request

RM0456 Rev 6

<!-- pagebreak -->

3134

Universal serial bus full-speed host/device interface (USB)

RM0456

and waits for its acknowledgment. Since the packet buffer memory has also to be accessed
by the microcontroller, an arbitration logic takes care of the access conflicts, using half
APB2 cycle for microcontroller access and the remaining half for the USB peripheral
access. In this way, both agents can operate as if the packet memory would be a dual-port
SRAM, without being aware of any conflict even when the microcontroller is performing
back-to-back accesses. The USB peripheral logic uses a dedicated clock. The frequency of
this dedicated clock is fixed by the requirements of the USB standard at 48 MHz, and this
can be different from the clock used for the interface to the APB2 bus. Different clock
configurations are possible where the APB2 clock frequency can be higher or lower than the
USB peripheral one.
Note:

Due to USB data rate and packet memory interface requirements, the APB2 clock must
have a minimum frequency of 12 MHz to avoid data overrun/underrun problems.
Each endpoint is associated with two packet buffers (usually one for transmission and the
other one for reception). Buffers can be placed anywhere inside the packet memory
because their location and size is specified in a buffer description table, which is also
located in the packet memory. Each table entry is associated to an endpoint register and it is
composed of two 32-bit words so that table start address must always be aligned to an 8byte boundary. Buffer descriptor table entries are described in Section 71.7: USBSRAM
registers. If an endpoint is unidirectional and it is neither an isochronous nor a doublebuffered bulk, only one packet buffer is required (the one related to the supported transfer
direction). Other table locations related to unsupported transfer directions or unused
endpoints, are available to the user. Isochronous and double-buffered bulk endpoints have
special handling of packet buffers (Refer to Section 71.5.5: Isochronous transfers in Device
mode and Section 71.5.3: Double-buffered endpoints and usage in Device mode
respectively). The relationship between buffer description table entries and packet buffer
areas is depicted in Figure 892.
For Host mode different sections explain the buffer usage model, notably Section 71.5.6:
Isochronous transfers in Host mode and Section 71.5.4: Double buffered channels: usage in
Host mode.

<!-- pagebreak -->

