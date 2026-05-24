RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.8

HSPI data length register (HSPI_DLR)
Address offset: 0x040
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

18

17

16

DL[31:16]
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

DL[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DL[31:0]: Data length
Number of data to be retrieved (value + 1) in indirect and automatic status-polling modes.
A value not greater than three (indicating 4 bytes) must be used for automatic status-polling
mode.
All 1’s in indirect mode means undefined length, where HSPI continues until the end of
the memory, as defined by DEVSIZE.
0x0000_0000: 1 byte is to be transferred.
0x0000_0001: 2 bytes are to be transferred.
0x0000_0002: 3 bytes are to be transferred.
0x0000_0003: 4 bytes are to be transferred.
...
0xFFFF_FFFD: 4,294,967,294 (4G-2) bytes are to be transferred.
0xFFFF_FFFE: 4,294,967,295 (4G-1) bytes are to be transferred.
0xFFFF_FFFF: undefined length; all bytes, until the end of the external device, (as defined
by DEVSIZE) are to be transferred. Continue reading indefinitely if DEVSIZE = 0x1F.
DL[0] is stuck at 1 in dual-memory configuration (DMM = 1) even when 0 is written to this bit,
thus assuring that each access transfers an even number of bytes.
This bitfield has no effect when in memory-mapped mode.

30.7.9

HSPI address register (HSPI_AR)
Address offset: 0x048
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0 and FMODE ≠ 11.

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

ADDRESS[31:16]
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

ADDRESS[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 31:0 ADDRESS[31:0]: Address
Address to be sent to the external device. In HyperBus protocol, this bitfield must be even as
this protocol is 16-bit word oriented. In dual-memory configuration, AR[0] is forced to 0.
Caution: Some memory specifications consider that each address corresponds to a 16-bit
value. HSPI considers that each address corresponds to an 8-bit value. So the
software needs to multiply the address by two when accessing the memory
registers.

30.7.10

HSPI data register (HSPI_DR)
Address offset: 0x050
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

DATA[31:16]
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

rw

rw

rw

rw

rw

rw

rw

rw

DATA[15:0]
rw

Bits 31:0 DATA[31:0]: Data
Data to be sent/received to/from the external SPI device
In indirect-write mode, data written to this register is stored on the FIFO before it is sent to the
external device during the data phase. If the FIFO is too full, a write operation is stalled until
the FIFO has enough space to accept the amount of data being written.
In indirect-read mode, reading this register gives (via the FIFO) the data that was received
from the external device. If the FIFO does not have as many bytes as requested by the read
operation and if BUSY = 1, the read operation is stalled until enough data is present or until
the transfer is complete, whichever happens first.
In automatic status-polling mode, this register contains the last data read from the external
device (without masking).
Word, half-word, and byte accesses to this register are supported. In indirect-write mode,
a byte write adds 1 byte to the FIFO, a half-word write 2 bytes, and a word write 4 bytes.
Similarly, in indirect-read mode, a byte read removes 1 byte from the FIFO, a halfword read
2 bytes, and a word read 4 bytes. Accesses in indirect mode must be aligned to the bottom of
this register: A byte read must read DATA[7:0] and a half-word read must read DATA[15:0].

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.11

HSPI polling status mask register (HSPI_PSMKR)
Address offset: 0x080
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

18

17

16

MASK[31:16]
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

MASK[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 MASK[31:0]: Status mask
Mask to be applied to the status bytes received in automatic status-polling mode
For bit n:
0: Bit n of the data received in automatic status-polling mode is masked and its value is not
considered in the matching logic.
1: Bit n of the data received in automatic status-polling mode is unmasked and its value is
considered in the matching logic.

30.7.12

HSPI polling status match register (HSPI_PSMAR)
Address offset: 0x088
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

18

17

16

MATCH[31:16]
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

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

MATCH[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 MATCH[31:0]: Status match
Value to be compared with the masked status register to get a match

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

30.7.13

RM0456

HSPI polling interval register (HSPI_PIR)
Address offset: 0x090
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

INTERVAL[15:0]
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
Bits 15:0 INTERVAL[15:0]: Polling interval
Number of CLK cycles between a read during the automatic status-polling phases

30.7.14

HSPI communication configuration register (HSPI_CCR)
Address offset: 0x100
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

31

30

29

28

27

Res.

Res.

DQSE

Res.

DDTR

15

14

13

Res.

Res.

ADSIZE[1:0]

rw

rw

12

rw

26

25

24

DMODE[2:0]

rw

rw

rw

rw

11

10

9

8

ADDTR
rw

ADMODE[2:0]
rw

rw

23

22

21

Res.

Res.

ABSIZE[1:0]

7

6

Res.

Res.

rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 DQSE: DQS enable
This bit enables the data strobe management.
0: DQS disabled
1: DQS enabled
Bit 28 Reserved, must be kept at reset value.
Bit 27 DDTR: Data double transfer rate
This bit sets the DTR mode for the data phase.
0: DTR mode disabled for the data phase
1: DTR mode enabled for the data phase

<!-- pagebreak -->

RM0456 Rev 6

20

19

18

ABDTR

17

16

ABMODE[2:0]

rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

ISIZE[1:0]

IDTR

rw

rw

rw

IMODE[2:0]
rw

rw

rw

RM0456

Hexadeca-SPI interface (HSPI)

Bits 26:24 DMODE[2:0]: Data mode
This bitfield defines the data phase mode of operation.
000: No data
001: Data on a single line
010: Data on two lines
011: Data on four lines
100: Data on eight lines
101: Data on 16 lines
Others: Reserved
Bits 23:22 Reserved, must be kept at reset value.
Bits 21:20 ABSIZE[1:0]: Alternate-byte size
This bitfield defines the alternate-byte size.
00: 8-bit alternate bytes
01: 16-bit alternate bytes
10: 24-bit alternate bytes
11: 32-bit alternate bytes
Bit 19 ABDTR: Alternate-byte double transfer rate
This bit sets the DTR mode for the alternate-byte phase.
0: DTR mode disabled for the alternate-byte phase
1: DTR mode enabled for the alternate-byte phase
Bits 18:16 ABMODE[2:0]: Alternate-byte mode
This bitfield defines the alternate-byte phase mode of operation.
000: No alternate bytes
001: Alternate bytes on a single line
010: Alternate bytes on two lines
011: Alternate bytes on four lines
100: Alternate bytes on eight lines
Others: Reserved
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:12 ADSIZE[1:0]: Address size
This bitfield defines the address size.
00: 8-bit address
01: 16-bit address
10: 24-bit address
11: 32-bit address
Bit 11 ADDTR: Address double transfer rate
This bit sets the DTR mode for the address phase.
0: DTR mode disabled for the address phase
1: DTR mode enabled for the address phase
Bits 10:8 ADMODE[2:0]: Address mode
This bitfield defines the address phase mode of operation.
000: No address
001: Address on a single line
010: Address on two lines
011: Address on four lines
100: Address on eight lines
Others: Reserved

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 ISIZE[1:0]: Instruction size
This bitfield defines the instruction size.
00: 8-bit instruction
01: 16-bit instruction
10: 24-bit instruction
11: 32-bit instruction
Bit 3 IDTR: Instruction double transfer rate
This bit sets the DTR mode for the instruction phase.
0: DTR mode disabled for the instruction phase
1: DTR mode enabled for the instruction phase
Bits 2:0 IMODE[2:0]: Instruction mode
This bitfield defines the instruction phase mode of operation.
000: No instruction
001: Instruction on a single line
010: Instruction on two lines
011: Instruction on four lines
100: Instruction on eight lines
Others: Reserved

30.7.15

HSPI timing configuration register (HSPI_TCR)
Address offset: 0x108
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

18

17

16

Res.

SSHIF
T

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DCYC[4:0]
rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 SSHIFT: Sample shift
By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external
device.
This bit allows the data to be sampled later in order to consider the external signal delays.
0: No shift
1: 1/2 cycle shift
The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode
(when DDTR = 1.)
Bits 29:5 Reserved, must be kept at reset value.
Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.16

HSPI instruction register (HSPI_IR)
Address offset: 0x110
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

18

17

16

INSTRUCTION[31:16]
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

22

21

20

19

18

17

16

INSTRUCTION[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 INSTRUCTION[31:0]: Instruction
Instruction to be sent to the external SPI device

30.7.17

HSPI alternate bytes register (HSPI_ABR)
Address offset: 0x120
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

ALTERNATE[31:16]

ALTERNATE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 ALTERNATE[31:0]: Alternate bytes
Optional data to be sent to the external SPI device right after the address.

30.7.18

HSPI low-power timeout register (HSPI_LPTR)
Address offset: 0x130
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

rw

rw

rw

rw

rw

rw

rw

TIMEOUT[15:0]
rw

rw

Bits 31:16 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 15:0 TIMEOUT[15:0]: Timeout period
After each access in memory-mapped mode, the HSPI prefetches the subsequent bytes and
hold them in the FIFO.
This bitfield indicates how many CLK cycles the HSPI waits after the clock becomes inactive
and until it raises the NCS, putting the external device in a lower-consumption state.

30.7.19

HSPI wrap communication configuration register
(HSPI_WPCCR)
Address offset: 0x140
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

31

30

29

28

27

Res.

Res.

DQSE

Res.

DDTR

15

14

13

Res.

Res.

ADSIZE[1:0]

rw

rw

12

rw

26

25

24

DMODE[2:0]

rw

rw

rw

rw

11

10

9

8

ADDTR
rw

ADMODE[2:0]
rw

rw

23

22

21

Res.

Res.

ABSIZE[1:0]

7

6

Res.

Res.

rw

16

rw

rw

rw

rw

4

3

2

1

0

ISIZE[1:0]

IDTR

rw

rw

Bit 27 DDTR: Data double transfer rate
This bit sets the DTR mode for the data phase.
0: DTR mode disabled for the data phase
1: DTR mode enabled for the data phase
Bits 26:24 DMODE[2:0]: Data mode
This bitfield defines the data phase mode of operation.
000: No data
001: Data on a single line
010: Data on two lines
011: Data on four lines
100: Data on eight lines
101: Data on 16 lines
Others: Reserved

RM0456 Rev 6

17
ABMODE[2:0]

rw

Bit 28 Reserved, must be kept at reset value.

<!-- pagebreak -->

18

5

Bit 29 DQSE: DQS enable
This bit enables the data strobe management.
0: DQS disabled
1: DQS enabled

Bits 21:20 ABSIZE[1:0]: Alternate-byte size
This bitfield defines the alternate-byte size.
00: 8-bit alternate bytes
01: 16-bit alternate bytes
10: 24-bit alternate bytes
11: 32-bit alternate bytes

19
ABDTR

rw

Bits 31:30 Reserved, must be kept at reset value.

Bits 23:22 Reserved, must be kept at reset value.

20

rw

IMODE[2:0]
rw

rw

rw

RM0456

Hexadeca-SPI interface (HSPI)

Bit 19 ABDTR: Alternate-byte double transfer rate
This bit sets the DTR mode for the alternate-byte phase.
0: DTR mode disabled for the alternate-byte phase
1: DTR mode enabled for the alternate-byte phase
Bits 18:16 ABMODE[2:0]: Alternate-byte mode
This bitfield defines the alternate byte phase mode of operation.
000: No alternate bytes
001: Alternate bytes on a single line
010: Alternate bytes on two lines
011: Alternate bytes on four lines
100: Alternate bytes on eight lines
Others: Reserved
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:12 ADSIZE[1:0]: Address size
This bitfield defines the address size.
00: 8-bit address
01: 16-bit address
10: 24-bit address
11: 32-bit address
Bit 11 ADDTR: Address double transfer rate
This bit sets the DTR mode for the address phase.
0: DTR mode disabled for the address phase
1: DTR mode enabled for the address phase
Bits 10:8 ADMODE[2:0]: Address mode
This bitfield defines the address phase mode of operation.
000: No address
001: Address on a single line
010: Address on two lines
011: Address on four lines
100: Address on eight lines
Others: Reserved
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 ISIZE[1:0]: Instruction size
This bitfield defines the instruction size.
00: 8-bit instruction
01: 16-bit instruction
10: 24-bit instruction
11: 32-bit instruction
Bit 3 IDTR: Instruction double transfer rate
This bit sets the DTR mode for the instruction phase.
0: DTR mode disabled for the instruction phase
1: DTR mode enabled for the instruction phase

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 2:0 IMODE[2:0]: Instruction mode
This bitfield defines the instruction phase mode of operation.
000: No instruction
001: Instruction on a single line
010: Instruction on two lines
011: Instruction on four lines
100: Instruction on eight lines
Others: Reserved

30.7.20

HSPI wrap timing configuration register (HSPI_WPTCR)
Address offset: 0x148
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

18

17

16

Res.

SSHIF
T

Res.

Res.

Res.

Res.

Res.

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
rw

rw

rw

rw

rw

DCYC[4:0]
rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 SSHIFT: Sample shift
By default, the HSPI samples data 1/2 of a CLK cycle after the data is driven by the external
device.
This bit allows the data to be sampled later in order to consider the external signal delays.
0: No shift
1: 1/2 cycle shift
The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode
(when DDTR = 1).
Bits 29:5 Reserved, must be kept at reset value.
Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.21

HSPI wrap instruction register (HSPI_WPIR)
Address offset: 0x150
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

18

17

16

INSTRUCTION[31:16]
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

INSTRUCTION[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 INSTRUCTION[31:0]: Instruction
Instruction to be sent to the external SPI device

30.7.22

HSPI wrap alternate byte register (HSPI_WPABR)
Address offset: 0x160
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

23

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

rw

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

ALTERNATE[31:16]

ALTERNATE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 ALTERNATE[31:0]: Alternate bytes
Optional data to be sent to the external SPI device right after the address

30.7.23

HSPI write communication configuration register
(HSPI_WCCR)
Address offset: 0x180
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0. Its content has a meaning only when
requesting write operations in memory-mapped mode.

31

30

29

28

27

Res.

Res.

DQSE

Res.

DDTR

15

14

13

Res.

Res.

ADSIZE[1:0]

rw

rw

12

rw

26

25

24

DMODE[2:0]

rw

rw

rw

rw

11

10

9

8

ADDTR
rw

ADMODE[2:0]
rw

rw

23

22

21

Res.

Res.

ABSIZE[1:0]

7

6

Res.

Res.

rw

RM0456 Rev 6

20

19

18

ABDTR

17

16

ABMODE[2:0]

rw

rw

rw

rw

rw

rw

5

4

3

2

1

0

ISIZE[1:0]

IDTR

rw

rw

rw

IMODE[2:0]
rw

rw

rw

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 DQSE: DQS enable
This bit enables the data strobe management.
0: DQS disabled
1: DQS enabled
Bit 28 Reserved, must be kept at reset value.
Bit 27 DDTR: Data double transfer rate
This bit sets the DTR mode for the data phase.
0: DTR mode disabled for the data phase
1: DTR mode enabled for the data phase
Bits 26:24 DMODE[2:0]: Data mode
This bitfield defines the data phase mode of operation.
000: No data
001: Data on a single line
010: Data on two lines
011: Data on four lines
100: Data on eight lines
101: Data on 16 lines
Others: Reserved
Bits 23:22 Reserved, must be kept at reset value.
Bits 21:20 ABSIZE[1:0]: Alternate-byte size
This bitfield defines the alternate-byte size:
00: 8-bit alternate bytes
01: 16-bit alternate bytes
10: 24-bit alternate bytes
11: 32-bit alternate bytes
Bit 19 ABDTR: Alternate-byte double-transfer rate
This bit sets the DTR mode for the alternate-byte phase.
0: DTR mode disabled for the alternate-byte phase
1: DTR mode enabled for the alternate-byte phase
Bits 18:16 ABMODE[2:0]: Alternate-byte mode
This bitfield defines the alternate-byte phase mode of operation.
000: No alternate bytes
001: Alternate bytes on a single line
010: Alternate bytes on two lines
011: Alternate bytes on four lines
100: Alternate bytes on eight lines
Others: Reserved
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:12 ADSIZE[1:0]: Address size
This bitfield defines the address size.
00: 8-bit address
01: 16-bit address
10: 24-bit address
11: 32-bit address

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

Bit 11 ADDTR: Address double transfer rate
This bit sets the DTR mode for the address phase.
0: DTR mode disabled for the address phase
1: DTR mode enabled for the address phase
Bits 10:8 ADMODE[2:0]: Address mode
This bitfield defines the address phase mode of operation.
000: No address
001: Address on a single line
010: Address on two lines
011: Address on four lines
100: Address on eight lines
Others: Reserved
Bits 7:6 Reserved, must be kept at reset value.
Bits 5:4 ISIZE[1:0]: Instruction size
This bitfield defines the instruction size:
00: 8-bit instruction
01: 16-bit instruction
10: 24-bit instruction
11: 32-bit instruction
Bit 3 IDTR: Instruction double transfer rate
This bit sets the DTR mode for the instruction phase.
0: DTR mode disabled for the instruction phase
1: DTR mode enabled for the instruction phase
Bits 2:0 IMODE[2:0]: Instruction mode
This bitfield defines the instruction phase mode of operation.
000: No instruction
001: Instruction on a single line
010: Instruction on two lines
011: Instruction on four lines
100: Instruction on eight lines
Others: Reserved

30.7.24

HSPI write timing configuration register (HSPI_WTCR)
Address offset: 0x188
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0. Its content has a meaning only when
requesting write operations in memory-mapped mode.

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

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DCYC[4:0]
rw

rw

rw

Bits 31:5 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).

30.7.25

HSPI write instruction register (HSPI_WIR)
Address offset: 0x190
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0. Its content has a meaning only when
requesting write operations in memory-mapped mode.

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

INSTRUCTION[31:16]
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

rw

rw

rw

rw

rw

rw

rw

INSTRUCTION[15:0]
rw

rw

Bits 31:0 INSTRUCTION[31:0]: Instruction
Instruction to be sent to the external SPI device

30.7.26

HSPI write alternate byte register (HSPI_WABR)
Address offset: 0x1A0
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0. Its content has a meaning only when
requesting write operations in memory-mapped mode.

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

ALTERNATE[31:16]
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

rw

rw

rw

rw

rw

rw

rw

rw

rw

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

ALTERNATE[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 ALTERNATE[31:0]: Alternate bytes
Optional data to be sent to the external SPI device right after the address

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.27

HSPI HyperBus latency configuration register (HSPI_HLCR)
Address offset: 0x200
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

TACC[7:0]
rw

rw

rw

rw

rw

rw

rw

23

22

21

20

19

18

17

16

rw

TRWR[7:0]
rw

rw

rw

rw

rw

rw

rw

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

WZL

LM

rw

rw

rw

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 TRWR[7:0]: Read-write minimum recovery time
Device Read-to-write/write-to-read minimum recovery time expressed in number of
communication clock cycles
Bits 15:8 TACC[7:0]: Access time
Device access time according to the memory latency, expressed in number of
communication clock cycles
Bits 7:2 Reserved, must be kept at reset value.
Bit 1 WZL: Write zero latency
This bit enables zero latency on write operations.
0: Latency on write accesses
1: No latency on write accesses
Bit 0 LM: Latency mode
This bit selects the latency mode.
0: Variable initial latency
1: Fixed latency
Note: This bit must be set when using the dual-octal HyperBus configuration.

30.7.28

HSPI full-cycle calibration configuration (HSPI_CALFCR)
Address offset: 0x210
Reset value: 0x0000 0000
This read-only register gives the calibration code needed by the DLL master so that its delay
is equivalent to a full memory-clock cycle. The value of this register is updated every time
that auto-calibration finishes.

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

CALMA
X

Res.

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
r

r

r

r

r

r

r

RM0456 Rev 6

20

19

18

17

16

COARSE[4:0]

FINE[6:0]
r

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

Bit 31 CALMAX: Max value
This bit gets set when the memory-clock period is outside the range of DLL master, in which
case HSPI_CALFCR and HSPI_CALSR are updated with the values for the maximum delay.
Bits 30:21 Reserved, must be kept at reset value.
Bits 20:16 COARSE[4:0]: Coarse calibration
The delay unitary value for this bitfield depends on product technology (see the datasheet).
Bits 15:7 Reserved, must be kept at reset value.
Bits 6:0 FINE[6:0]: Fine calibration
The delay unitary value for this bitfield depends on product technology (see the datasheet).

30.7.29

HSPI DLL master calibration configuration (HSPI_CALMR)
Address offset: 0x218
Reset value: 0x0000 0000
The DLL Master is used for delaying the feedback clock when reading without DQS.
The delay of the master DLL is determined by the value in this register.
This register can always be read by software and can be modified only when BUSY = 0.
This register never gets updated automatically by hardware.

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

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

6

5

20

19

rw

rw

4

3

18

17

16

rw

rw

rw

2

1

0

rw

rw

rw

COARSE[4:0]

FINE[6:0]
rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 COARSE[4:0]: Coarse calibration
The delay unitary value for this bitfield depends on product technology (see the datasheet).
Bits 15:7 Reserved, must be kept at reset value.
Bits 6:0 FINE[6:0]: Fine calibration
The delay unitary value for this bitfield depends on product technology (see the datasheet).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

30.7.30

HSPI DLL slave output calibration configuration
(HSPI_CALSOR)
Address offset: 0x220
Reset value: 0x0000 0000
The DLL output slave is used to delay the output data in DDR mode for write operations.
The delay of the output slave DLL is determined by the value in this register.
This register is updated automatically by hardware at the end of calibration (at the same
moment that HSPI_CALFCR is updated).
If this register is written after the last write to HSPI_DCR2 or HSPI_CCR, then
auto-calibration is not executed on the next transfer (auto-calibration is not performed for
HSPI_CALSIR as well).
This register can always be read by software and can be modified only when BUSY = 0.

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

Res.

Res.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

20

19

18

17

16

COARSE[4:0]
rw

rw

rw

rw

rw

4

3

2

1

0

rw

rw

rw

FINE[6:0]
rw

rw

rw

rw

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 COARSE[4:0]: Coarse calibration
The delay unitary value for this bitfield depends on product technology.
Bits 15:7 Reserved, must be kept at reset value.
Bits 6:0 FINE[6:0]: Fine calibration
The delay unitary value for this bitfield depends on product technology.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

30.7.31

RM0456

HSPI DLL slave input calibration configuration (HSPI_CALSIR)
Address offset: 0x228
Reset value: 0x0000 0000
The DLL input slave is used to delay the DQS input for sampling the data when DQS is
enabled for read operations. The delay of the input slave DLL is determined by the value in
this register.
This register is updated automatically by hardware at the end of calibration (at the same
moment that HSPI_CALFCR is updated).
If this register is written after the last write to HSPI_DCR2 or HSPI_CCR, then
auto-calibration is not executed on the next transfer (auto-calibration is not performed for
HSPI_CALSOR as well).
This register can always be read by software and can be modified only when BUSY = 0.

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

20

19

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

rw

rw

18

17

16

rw

rw

rw

2

1

0

rw

rw

rw

COARSE[4:0]

FINE[6:0]
rw

Bits 31:21 Reserved, must be kept at reset value.
Bits 20:16 COARSE[4:0]: Coarse calibration
The delay unitary value for this bitfield depends on product technology.
Bits 15:7 Reserved, must be kept at reset value.
Bits 6:0 FINE[6:0]: Fine calibration
The delay unitary value for this bitfield depends on product technology.

30.7.32

HSPI register map

2

1

0

DMAEN

ABORT

EN

FRCK

3

0

CKMODE

4

TCEN

0

0

Res.

5

Res.

0

0

Res.

6

0

Res.

0

7

0

DMM

8

0

Res.

14
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

RM0456 Rev 6

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

CSHT[5:0]

Res.

Res.

Res.

DEVSIZE[4:0]

Res.

Res.

Res.

MTYP
[2:0]

Res.

Reserved
Res.

HSPI_DCR1

0

9

15

0

11

16

Res.

0

10

17

TEIE

0

FTHRES[5:0]

18

TCIE

0

12

19

FTIE

0

13

20

21
Res.

0

TOIE

22

0

0

SMIE

23

APMS

25
Res.

24

26
Res.

PMM

27

Res.

0

Res.

28

29

FMDOE[1:0]
0

Reserved

Reset value

<!-- pagebreak -->

30

0

Res.

0x008

Reset value

Res.

0x0004

HSPI_CR

Res.

0x000

MSEL[1:0]

Register name

Res.

Offset

31

Table 271. HSPI register map and reset values

RM0456

Hexadeca-SPI interface (HSPI)

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

0

0

Res.

TEF

Res.

TCF

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

HSPI_AR

ADDRESS[31:0]

Reset value

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

Reserved

Reserved

HSPI_DR

DATA[31:0]

Reset value

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

Reserved
HSPI_PSMKR
Reset value

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

Reserved

HSPI_PSMAR

MATCH[31:0]
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

0

0

Res.

Res.

Res.

Res.

Reserved
Res.

Reserved
HSPI_PIR

0

MASK[31:0]
0

Reserved

Reset value

0

Reserved

Reset value
0x0940x0FC

0

Reserved

Res.

0x090

0

Res.

0x08C

0

Res.

0x088

0

Res.

0x084

0

Res.

0x080

0

Res.

0x0540x07C

0

Res.

0x050

0

0

CTEF

Res.

0

CTCF

Res.

FTF

Res.

0

Res.

Res.

SMF

Res.

0

CSMF

0

TOF

0

CTOF

0

BUSY

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.
Res.

Res.

FLEVEL[6:0]

Reserved

Res.

0x04C

DL[31:0]

Res.

0x048

0

Res.

0

HSPI_DLR
Reset value

1

Res.

0

Reserved

Res.

0x044

2

Res.

0

Reserved

Res.

0x040

3

Res.

0

Res.

0

Res.

0

Res.

0

Reset value
0x0280x03C

4

5

8
Res.

6

9
Res.

7

11

10
Res.

12

Res.

13

19
Res.

Res.

20
Res.

Res.

21
Res.
Res.

14

22
Res.
Res.

Res.

23
Res.
Res.

0

Reserved

Res.

HSPI_FCR

0

0

Reset value

0x024

Res.

24
Res.
Res.

15

25
Res.

16

26
Res.

Res.

Res.

Res.

Res.

27
Res.

0

Reserved

HSPI_SR

0

Res.

0

0
Res.

0

PRESCALER[7:0]

REFRESH[31:0]

Res.

0x020

0

HSPI_DCR4
Reset value

0

CSBOUND[4:0]
0

Res.

0x0180x01C

0

Res.

28
Res.
Res.

Reset value
0x014

17

29
Res.
Res.

HSPI_DCR3

Res.

0x010

0
Res.

Reset value

18

30

HSPI_DCR2

WRAPSIZE[2:0]

31

0x00C

Register name

Res.

Offset

Res.

Table 271. HSPI register map and reset values

INTERVAL[15:0]
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

Reserved

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

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

Reserved
HSPI_ABR
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

1

2

IMODE[2:0]

IDTR

3

4

5

6

ISIZE[1:0]

7

Res.

8

Res.

9

Res.

0

0

0

Res.

Res.

Res.

DCYC[4:0]
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

IDTR

0

ISIZE
[1:0]

0

Res.

0

Res.

TIMEOUT[15:0]

0

0

0

0

0

0

Res.

Res.

0

IMODE
[2:0]

0

Res.

ADMODE
[2:0]

ADDTR

0

Res.

0

ADSIZE
[1:0]

0

Res.

0

Res.

0

0

Res.

0

ABMODE
[2:0]

ABDTR

Res.

Res.

0

ABSIZE
[1:0]

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

Res.

Res.

Res.

Res.

SSHIFT

Reserved

0

DCYC[4:0]
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

RM0456 Rev 6

0

0

0

0

0

0

0

IMODE
[2:0]

ADMODE
[2:0]

0

0

Res.

0

ABMODE
[2:0]

Res.
0

ABDTR

0

ABSIZE
[1:0]

Reserved

Res.

Reserved

0

0

ADDTR

0

0

ADSIZE
[1:0]

ALTERNATE[31:0]

0

Res.

HSPI_WPABR

0

IDTR

0

ISIZE
[1:0]

0

Res.

0

Res.

INSTRUCTION[31:0]
0

Reserved

HSPI_WCCR

ADMODE
[2:0]
Res.

11

0

Reserved

Reset value

10
Res.

12

ADDTR
Res.

13

Res.

ADSIZE
[1:0]
Res.

14
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

DDTR

Res.

0

HSPI_WPIR

Reset value

<!-- pagebreak -->

0

DMODE
[2:0]

0x180

DMODE
[2:0]

DDTR

0x1640x17C

DQSE

Res.

0

Res.

0x160

0

Reserved

Reserved

Reset value

0

0

Reserved
HSPI_WPTCR

0

Reserved

Reserved

HSPI_WPCCR

0

ALTERNATE[31:0]
0

Reserved
HSPI_LPTR

0

Res.

Reset value

0

Reserved

Res.

Reset value

DQSE

0x1540x15C

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved

Res.

0x150

0

INSTRUCTION[31:0]

Reset value
0x14C

0

HSPI_IR

Res.

0x148

0

Reserved

Reset value
0x144

0

0

0

Res.

0x140

0

0

Reset value
0x1340x13C

Res.

19

0

15

20

ABDTR

16

21

ABSIZE
[1:0]

0

Res.

22

17

23

Res.

0

18

24

Res.

0

ABMODE
[2:0]

26

25

27
DDTR
Res.

Res

Res.

Res.

SSHIFT

HSPI_TCR

Res.

0x130

0

Res.

0x1240x12C

0

Res.

0x120

0

Res.

0x1140x11C

0

Reserved

Res.

0x110

0

0

Reset value
0x10C

DMODE
[2:0]

Reserved

Res.

0x108

28

29

Reset value
0x104

Res.

30

DQSE

HSPI_CCR

Res.

0x100

31

Register name

Res.

Offset

Res.

Table 271. HSPI register map and reset values

0

0

0

RM0456

Hexadeca-SPI interface (HSPI)

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

5
Res.

0

6
Res.

1

7
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

LM

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

0

TACC[7:0]

Res.

0

0

0

0

0

Res.

Res.

Res.

Res.

COARSE[4:0]

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

FINE[6:0]
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

0

FINE[6:0]
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

0

FINE[6:0]
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

COARSE[4:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved
Res.

Reserved

0

Res.

COARSE[4:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved
Res.

Reserved

0

Res.

COARSE[4:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Reserved
Res.

Reserved

Reset value

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

0

Res.

CALMAX

Reset value

HSPI_CALSIR

0

Reserved

HSPI_CALFCR

HSPI_CALSOR

0

TRWR[7:0]
0

Res.

0x228

0

Reserved

HSPI_CALMR

0

WZL

Res.

Res.

Res.

HSPI_HLCR

0

Reserved

Reset value
0x224

0

Reserved

Res.

0x220

0

ALTERNATE[31:0]
0

Reset value
0x22C

0

Res.

Reset value

Res.

0x218

0

HSPI_WABR

Res.

0x214

0

Reserved

Res.

0x210

0

Reserved

Reset value
0x2040x20C

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

0

Res.

0

Res.

0x200

0

Res.

0x1A40x1FC

Reset value

Res.

0x1A0

Reserved
INSTRUCTION[31:0]

Res.

0x1940x19C

Reserved
HSPI_WIR

Res.

0x190

2

8
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

DCYC[4:0]

Reset value
0x18C

3

9
Res.

4

11

10
Res.

12
Res.

Res.

13
Res.

14

15

16

17

18

19

20

21

22

23

24

25

26

28

29

27
Res.

HSPI_WTCR

Reserved
Res.

0x188

30

Reserved
Res.

0x184

Res.

Register name

Res.

Offset

31

Table 271. HSPI register map and reset values

FINE[6:0]
0

0

0

0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

