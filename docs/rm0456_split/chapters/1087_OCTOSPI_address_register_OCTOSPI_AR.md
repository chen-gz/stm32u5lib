rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bits 31:0 DL[31:0]: Data length
Number of data to be retrieved (value+1) in indirect and automatic status-polling modes. A
value not greater than three (indicating 4 bytes) must be used for automatic status-polling
mode.
All 1’s in indirect mode means undefined length, where OCTOSPI continues until the end of
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
This bitfield has no effect in memory-mapped mode.

28.7.9

OCTOSPI address register (OCTOSPI_AR)
Address offset: 0x0048
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

Bits 31:0 ADDRESS[31:0]: Address
Address to be sent to the external device. In HyperBus protocol, this field must be even as
this protocol is 16-bit word oriented. In dual-memory configuration, AR[0] is forced to 0.

28.7.10

OCTOSPI data register (OCTOSPI_DR)
Address offset: 0x0050
Reset value: 0x0000 0000

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

DATA[31:16]

DATA[15:0]
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

1104

Octo-SPI interface (OCTOSPI)

RM0456

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

28.7.11

OCTOSPI polling status mask register (OCTOSPI_PSMKR)
Address offset: 0x0080
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

28.7.12

OCTOSPI polling status match register (OCTOSPI_PSMAR)
Address offset: 0x0088
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

28.7.13

OCTOSPI polling interval register (OCTOSPI_PIR)
Address offset: 0x0090
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

28.7.14

OCTOSPI communication configuration register (OCTOSPI_CCR)
Address offset: 0x0100
Reset value: 0x0000 0000
This register can be modified only when BUSY = 0.

31

30

29

28

27

SIOO

Res.

DQSE

Res.

DDTR

rw
15
Res.

rw
14
Res.

26

25

24

DMODE[2:0]

rw

rw

rw

rw

12

11

10

9

8

ADSIZE[1:0]

AD
DTR

rw

rw

13

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

1104

Octo-SPI interface (OCTOSPI)

RM0456

Bit 31 SIOO: Send instruction only once mode
This bit has no effect when IMODE = 00 (see Sending the instruction only once (SIOO)).
0: Send instruction on every transaction
1: Send instruction only for the first command
Bit 30 Reserved, must be kept at reset value.
Bit 29 DQSE: DQS enable
This bit enables the data strobe management.
0: DQS disabled
1: DQS enabled
Bit 28 Reserved, must be kept at reset value.
Bit 27 DDTR: Data double transfer rate
This bit sets the DTR mode for the data phase.
0: DTR mode disabled for data phase
1: DTR mode enabled for data phase
Bits 26:24 DMODE[2:0]: Data mode
This bitfield defines the data phase mode of operation.
000: No data
001: Data on a single line
010: Data on two lines
011: Data on four lines
100: Data on eight lines
Others: Reserved
Bits 23:22 Reserved, must be kept at reset value.
Bits 21:20 ABSIZE[1:0]: Alternate-byte size
This bitfield defines the alternate-byte size.
00: 8-bit alternate bytes
01: 16-bit alternate bytes
10: 24-bit alternate bytes
11: 32-bit alternate bytes
Bit 19 ABDTR: Alternate- byte double transfer rate
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

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
This bitfield defines instruction size.
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

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

28.7.15

RM0456

OCTOSPI timing configuration register (OCTOSPI_TCR)
Address offset: 0x0108
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

DHQC

Res.

Res.

Res.

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

DCYC[4:0]
rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 SSHIFT: Sample shift
By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the
external device.
This bit allows the data to be sampled later in order to consider the external signal delays.
0: No shift
1: 1/2 cycle shift
The software must ensure that SSHIFT = 0 when the data phase is configured in DTR mode
(when DDTR = 1.)
Bit 29 Reserved, must be kept at reset value.
Bit 28 DHQC: Delay hold quarter cycle
0: No delay hold
1: 1/4 cycle hold
Bits 27:5 Reserved, must be kept at reset value.
Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31).
It is recommended to have at least six dummy cycles when using memories with DQS
activated.

28.7.16

OCTOSPI instruction register (OCTOSPI_IR)
Address offset: 0x0110
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

INSTRUCTION[31:16]

INSTRUCTION[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bits 31:0 INSTRUCTION[31:0]: Instruction
Instruction to be sent to the external SPI device

28.7.17

OCTOSPI alternate bytes register (OCTOSPI_ABR)
Address offset: 0x0120
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

ALTERNATE[31:16]
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

28.7.18

OCTOSPI low-power timeout register (OCTOSPI_LPTR)
Address offset: 0x00130
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

TIMEOUT[15:0]
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
Bits 15:0 TIMEOUT[15:0]: Timeout period
After each access in memory-mapped mode, the OCTOSPI prefetches the subsequent bytes
and hold them in the FIFO.
This bitfield indicates how many CLK cycles the OCTOSPI waits after the clock becomes
inactive and until it raises the NCS, putting the external device in a lower-consumption state.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

28.7.19

RM0456

OCTOSPI wrap communication configuration register
(OCTOSPI_WPCCR)
Address offset: 0x0140
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

Res.

Res.

ADSIZE[1:0]

AD
DTR

rw

rw

rw

rw

26

25

24

DMODE[2:0]

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

18

17

16

ABMODE[2:0]

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

RM0456 Rev 6

19
ABDTR

rw

Bits 31:30 Reserved, must be kept at reset value.

<!-- pagebreak -->

20

rw

IMODE[2:0]
rw

rw

rw

RM0456

Octo-SPI interface (OCTOSPI)

Bits 18:16 ABMODE[2:0]: Alternate-byte mode
This bitfield defines the alternate-byte phase mode of operation.
000: no alternate bytes
001: alternate bytes on a single line
010: alternate bytes on two lines
011: alternate bytes on four lines
100: alternate bytes on eight lines
Others: reserved
Bits 15:14 Reserved, must be kept at reset value.
Bits 13:12 ADSIZE[1:0]: Address size
This bitfield defines the address size.
00: 8-bit address
01: 16-bit address
10: 24-bit address
11: 32-bit address
Bit 11 ADDTR: Address double transfer rate
This bit sets the DTR mode for the address phase.
0: DTR mode disabled for address phase
1: DTR mode enabled for address phase
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
Bits 2:0 IMODE[2:0]: Instruction mode
This bitfield defines the instruction phase mode of operation.
000: No instruction
001: Instruction on a single line
010: Instruction on two lines
011: Instruction on four lines
100: Instruction on eight lines
Others: Reserved

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

28.7.20

RM0456

OCTOSPI wrap timing configuration register (OCTOSPI_WPTCR)
Address offset: 0x0148
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

S
SHIFT

Res.

DHQC

Res.

Res.

Res.

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

DCYC[4:0]
rw

rw

rw

Bit 31 Reserved, must be kept at reset value.
Bit 30 SSHIFT: Sample shift
By default, the OCTOSPI samples data 1/2 of a CLK cycle after the data is driven by the
external device.
This bit allows the data to be sampled later in order to consider the external signal delays.
0: No shift
1: 1/2 cycle shift
The firmware must assure that SSHIFT=0 when the data phase is configured in DTR mode
(when DDTR = 1).
Bit 29 Reserved, must be kept at reset value.
Bit 28 DHQC: Delay hold quarter cycle
Add a quarter cycle delay on the outputs in DTR communication to match hold requirement.
0: No quarter cycle delay
1: 1/4 cycle delay inserted
Bits 27:5 Reserved, must be kept at reset value.
Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended
to have at least 5 dummy cycles when using memories with DQS activated.

28.7.21

OCTOSPI wrap instruction register (OCTOSPI_WPIR)
Address offset: 0x0150
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

INSTRUCTION[31:16]

INSTRUCTION[15:0]
rw

rw

<!-- pagebreak -->

rw

rw

rw

rw

rw

rw

rw

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bits 31:0 INSTRUCTION[31:0]: Instruction
Instruction to be sent to the external SPI device

28.7.22

OCTOSPI wrap alternate bytes register (OCTOSPI_WPABR)
Address offset: 0x0160
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

ALTERNATE[31:16]
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

28.7.23

OCTOSPI write communication configuration register
(OCTOSPI_WCCR)
Address offset: 0x0180
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

rw
15

14

13

Res.

Res.

ADSIZE[1:0]
rw

12

rw

26

rw

rw

11

10

ADDTR
rw

25

24

DMODE[2:0]
rw

rw

9

8

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

20

rw

rw

5

4

19

rw

3

2

IDTR

rw

rw

17

16

ABMODE[2:0]

rw

ISIZE[1:0]
rw

18

ABDTR

rw

rw

1

0

IMODE[2:0]
rw

rw

rw

Bits 31:30 Reserved, must be kept at reset value.
Bit 29 DQSE: DQS enable
This bit enables the data strobe management.
0: DQS disabled
1: DQS enabled
Bit 28 Reserved, must be kept at reset value.
Bit 27 DDTR: data double transfer rate
This bit sets the DTR mode for the data phase.
0: DTR mode disabled for the data phase
1: DTR mode enabled for the data phase

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

Bits 26:24 DMODE[2:0]: Data mode
This bitfield defines the data phase mode of operation.
000: No data
001: Data on a single line
010: Data on two lines
011: Data on four lines
100: Data on eight lines
Others: Reserved
Bits 23:22 Reserved, must be kept at reset value.
Bits 21:20 ABSIZE[1:0]: Alternate-byte size
This bitfield defines the alternate-byte size.
00: 8-bit alternate bytes
01: 16-bit alternate bytes
10: 24-bit alternate bytes
11: 32-bit alternate bytes
Bit 19 ABDTR: Alternate bytes double transfer rate
This bit sets the DTR mode for the alternate-bytes phase.
0: DTR mode disabled for alternate-bytes phase
1: DTR mode enabled for alternate-bytes phase
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
Bits 7:6 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

Bits 5:4 ISIZE[1:0]: Instruction size
This bitfield defines the instruction size.
00: 8-bit instruction
01: 16-bit instruction
10: 24-bit instruction
11: 32-bit instruction
Bit 3 IDTR: Instruction double transfer rate
This bit sets the DTR mode for the instruction phase.
0: DTR mode disabled for instruction phase
1: DTR mode enabled for instruction phase
Bits 2:0 IMODE[2:0]: Instruction mode
This bitfield defines the instruction phase mode of operation.
000: No instruction
001: Instruction on a single line
010: Instruction on two lines
011: Instruction on four lines
100: Instruction on eight lines
Others: Reserved

28.7.24

OCTOSPI write timing configuration register (OCTOSPI_WTCR)
Address offset: 0x0188
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

DCYC[4:0]
rw

Bits 31:5 Reserved, must be kept at reset value.
Bits 4:0 DCYC[4:0]: Number of dummy cycles
This bitfield defines the duration of the dummy phase according to the memory latency.
In both SDR and DTR modes, it specifies a number of CLK cycles (0-31). It is recommended
to have at least 5 dummy cycles when using memories with DQS activated.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

28.7.25

RM0456

OCTOSPI write instruction register (OCTOSPI_WIR)
Address offset: 0x0190
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

28.7.26

OCTOSPI write alternate bytes register (OCTOSPI_WABR)
Address offset: 0x01A0
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)

28.7.27

OCTOSPI HyperBus latency configuration register
(OCTOSPI_HLCR)
Address offset: 0x0200
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

rw

rw

rw

rw

rw

rw

rw

23

21

20

19

18

17

16

TRWR[7:0]

TACC[7:0]
rw

22

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

Bits 31:24 Reserved, must be kept at reset value.
Bits 23:16 TRWR[7:0]: Read-write minimum recovery time
Device read-to-write/write-to-read minimum recovery time expressed in number of
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

28.7.28

OCTOSPI register map

DMAEN

ABORT

EN

0

0

0

0

Res.

CKMODE

Res.

0

Res.

0

FRCK

0

DMM

0

MSEL

0

Res.

0

TCEN

Reset value

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

0

0

DLYBYP

Res.

CSHT[5:0]

Res.

Res.

DEVSIZE[4:0]

Res.

Res.

Res.

MTYP
[2:0]

Res.

Res.

Res.

Res.

OCTOSPI_DCR1

0

Res.

0

Res.

0

Res.

0

Res.

TEIE

0

FTIE

0

TCIE

TOIE

SMIE

0

Res..

APMS

Res.

Res.

Res.

0

FTHRES[4:0]

Reserved
Res.

0x0008

0

Reserved
Res.

0x0004

0

PMM

Reset value

Res.

OCTOSPI_CR

FMDOE[1:0]

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

0x0000

Register
name

Res.

Offset

Res.

Table 259. OCTOSPI register map and reset values

0

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

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
0

Reserved

Reserved

OCTOSPI_DLR

DL[31:0]
0

0

0

0

0

0

0

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

<!-- pagebreak -->

0

0

0

0

0

Reserved

Reserved

OCTOSPI_DR

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

0

0

0

Reserved

Reserved

OCTOSPI_
PSMKR

MASK[31:0]
0

0

0

0

0

0

0

0

0

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

OCTOSPI_
PSMAR

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

TEF

0

0

0

0

0

CTEF

FTF

Res.

TCF

Res.

0

Res.

Res.

0

CTCF

Res.

TOF

Res.

0

SMF

0

CTOF

0

CSMF

0

BUSY

0

Res.

0

Res..

Res.

0
Res.

FLEVEL[5:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

OCTOSPI_PIR

0

Reserved
Res.

Reserved

Reset value
0x00940x00FC

0

ADDRESS[31:0]

Res.

0x0090

0

OCTOSPI_AR

Reset value
0x008C

0

Res.

0x0088

0

Reserved

Reset value
0x0084

0

Res.

0x0080

0

Res.

0x00540x007C

0

Res.

0x0050

0

Res.

0x004C

0

Reserved

Res.

0x0048

Reset value

Res.

0x0044

Res.

0x0040

0

0

Reset value
0x00280x003C

0

MAXTRAN[7:0]

Res.

Res.
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OCTOSPI_FCR

Res.

Reset value

0x0024

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

OCTOSPI_SR

0

Reserved

Res.

Reserved

0x0020

0

Res.

0x00180x001C

0

REFRESH[31:0]

Res.

Reset value

0

0
Res.

CSBOUND[4:0]

OCTOSPI_DCR4

Res.

0x0014

0

PRESCALER[7:0]

Res..

Reset value

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

OCTOSPI_DCR3

0
Res.

Reset value

0x0010

WRAPSIZE
[2:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OCTOSPI_DCR2

Res.

0x000C

Res.

Register
name

Offset

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

Table 259. OCTOSPI register map and reset values (continued)

INTERVAL[15:0]
0

Reserved

Reserved

RM0456 Rev 6

0

0

0

0

0

0

0

0

0

RM0456

Octo-SPI interface (OCTOSPI)

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

IMODE
[2:0]

IDTR

ISIZE[1:0]

Res.

Res.

Res.

Res.

ADMODE
[2:0]

ADDTR

Res.

Res.

Res.

0

0

0

0

Res.

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

Res.

0

ADSIZE
[1:0]

0

Res.

0

0

Res.

0

ABMODE
[2:0]

ABDTR

0

ABSIZE
[1:0]

0

Res.

0

Res.

DDTR
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

ALTERNATE[31:0]
0

0

0

0

0

0

0

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

0

ADMODE
[2:0]

0

Res.

0

ABMODE
[2:0]

0

ABDTR

Reset value

DMODE
[2:0]

ABSIZE
[1:0]

OCTOSPI_WCCR

Res.

Reserved

Res.

Reserved

0

0

ADDTR

OCTOSPI_
WPABR

0

0

ADSIZE
[1:0]

Reserved

0

0

Res.

Reserved

0

IDTR

INSTRUCTION[31:0]

0

ISIZE
[1:0]

OCTOSPI_WPIR

0

Res.

Reserved

DCYC[4:0]

Res.

0

Reserved

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

0

DHQC.

Reserved

Res.

0x0180

0

DMODE
[2:0]

Reserved

Reset value
0x01640x017C

Res.

0

DQSE

Reset value

Res.

OCTOSPI_
WPCCR

Reset value

0

Reserved

Res.

Reserved

OCTOSPI_
WPTCR

0

0

DDTR

0x0160

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OCTOSPI_LPTR

0

Reserved
Res.

Reserved

Res.

Reset value

Res.

0x01540x015C

Res.

Res.

Res.

Res.

Res.

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

DQSE

0x0150

0

ALTERNATE[31:0]

Reset value
0x014C

0

OCTOSPI_ABR

Res.

0x0148

0

Reserved

SSHIFT

0x0144

0

Reserved

Res.

0x0140

0

INSTRUCTION[31:0]
0

Reset value
0x01340x013C

0

Reserved

OCTOSPI_IR
Reset value

0

Res.

0

ADSIZE
[1:0]

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

ABSIZE
[1:0]

Res.

Res.

DQSE

DDTR

0

Reserved

Res.

0x0130

0

0

Res.

0x01240x012C

0

Res.

0x0120

0

DHQC

OCTOSPI_TCR

Res.

0x01140x011C

0

Reserved

Res.

0x0110

0

DMODE
[2:0]

Reserved

Reset value
0x010C

Res.

SIOO
0

Res.

0x0108

Reset value

Res.

0x0104

OCTOSPI_CCR

SSHIFT

0x0100

Register
name

Res.

Offset

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

Table 259. OCTOSPI register map and reset values (continued)

0

0

0

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

OCTOSPI_WTCR

Res.

0x0188

Res.

Reserved
Res.

Reserved
Res.

0x0184

Res.

Register
name

Res.

Offset

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

Table 259. OCTOSPI register map and reset values (continued)

0

0

0

0

0

0

0

0

Reset value

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

0

0

0

0

0

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
ALTERNATE[31:0]
0

0

0

0

0

0

0

0

0

0

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

Reserved
Res.

Reserved
OCTOSPI_HLCR

0

LM

0

Res.

0

WZL

0

OCTOSPI_WABR

0x01A40x01FC

0x0200

0

Reserved

Res.

0x01A0

0

TRWR[7:0]
0

0

0

0

0

0

TACC[7:0]
0

0

0

0

0

0

0

0

Refer to Section 2.3: Memory organization for the register boundary addresses.

<!-- pagebreak -->

