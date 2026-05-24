RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
Figure 219. Voltage switch transceiver typical application

SD Host

Voltage transceiver
GPIO

SDMMC
SDMMC_CK
SDMMC_CKIN
SDMMC_CDIR

SDMMC_CMD

SDMMC_D0DIR

SDMMC_D0

SDMMC_D123DIR

SDMMC_D1

SDMMC_D2

SDMMC_D3

EN
SEL

CLK.h

CLKB

CLK-f
CMD.dir

CMD.h

CMDB

DAT0.dir

DAT0.h

DAT123.dir

Level converter

GPIO

SD Card

DAT0B

DAT1.h

DAT1B

DAT2.h

DAT2B

DAT3.h

DAT3B

MSv40951V2

To interface with an external driver (a voltage switch transceiver), next to the standard
signals the SDMMC uses the following signals:
•

SDMMC_CKIN feedback input clock

•

SDMMC_CDIR I/O direction control for the CMD signal

•

SDMMC_D0DIR I/O direction control for the SDMMC_D0 signal

•

SDMMC_D123DIR I/O direction control for the SDMMC_D1, SDMMC_D2 and
SDMMC_D3 signals

The voltage transceiver signals EN and SEL are to be handled through general-purpose
I/O.
The polarity of the SDMMC_CDIR, SDMMC_D0DIR and SDMMC_D123DIR signals can be
selected through SDMMC_POWER.DIRPOL control bit.

31.9

SDMMC interrupts

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Table 295. SDMMC interrupts
Interrupt
acronym

Interrupt event

Event flag

Enable control
bit

Interrupt clear
method

Exit from
Sleep
mode

SDMMC

Command response CRC fail

CCRCFAIL

CCRCFAILIE

CCRCFAILC

Yes

SDMMC

Data block CRC fail

DCRCFAIL

DCRCFAILIE

DCRCFAILC

Yes

SDMMC

Command response timeout

CTIMEOUT

CTIMEOUTIE

CTIMEOUTC

Yes

SDMMC

Data timeout

DTIMEOUT

DTIMEOUTIE

DTIMEOUTC

Yes

SDMMC

Transmit FIFO underrun

TXUNDERR

TXUNDERRIE

TXUNDERRC

Yes

SDMMC

Receive FIFO overrun

RXOVERR

RXOVERRIE

RXOVERRC

Yes

SDMMC

Command reponse received

CMDREND

CMDRENDIE

CMDRENDC

Yes

SDMMC

Comand sent

CMDSENT

CMDSENTIE

CMDSENTC

Yes

SDMMC

Data transfer ended

DATAEND

DATAENDIE

DATAENDC

Yes

SDMMC

Data transfer hold

DHOLD

DHOLDIE

DHOLDC

Yes

SDMMC

Data block sent or received

DBCKEND

DBCKENDIE

DBCKENDC

Yes

SDMMC

Data transfer aborted

DABORT

DABORTIE

DABORTC

Yes

SDMMC

Transmit FIFO half empty

TXFIFOHE

TXFIFOHEIE

N/A

Yes

SDMMC

Receive FIFO half full

RXFIFOHF

RXFIFOHFIE

N/A

Yes

SDMMC

Transmit FIFO full

TXFIFOF

N/A

N/A

Yes

SDMMC

Receive FIFO full

RXFIFOF

RXFIFOFIE

N/A

Yes

SDMMC

Transmit FIFO empty

TXFIFOE

TXFIFOEIE

N/A

Yes

SDMMC

Receive FIFO empty

RXFIFOE

N/A

N/A

Yes

SDMMC

Command response end of busy

BUSYD0END

BUSYD0ENDIE

BUSYD0ENDC

Yes

SDMMC

SDIO interrupt

SDIOIT

SDIOITIE

SDIOITC

Yes

SDMMC

Boot acknowledgment fail

ACKFAIL

ACKFAILIE

ACKFAILC

Yes

SDMMC

Boot acknowledgment timeout

ACKTIMEOUT

SDMMC

Voltage switch timing

VSWEND

VSWENDIE

VSWENDC

Yes

SDMMC

SDMM_CK stopped in voltage
switch

CKSTOP

CKSTOPIE

CKSTOPC

Yes

SDMMC

IDMA transfer error

IDMATE

IDMATEIE

IDMATEC

Yes

SDMMC

IDMA buffer transfer complete

IDMABTC

IDMABTCIE

IDMABTCC

Yes

<!-- pagebreak -->

RM0456 Rev 6

ACKTIMEOUTIE ACKTIMEOUTC

Yes

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10

SDMMC registers
The device communicates to the system via 32-bit control registers accessible via AHB
slave interface.
The peripheral registers have to be accessed by words (32-bit). Byte (8-bit) and half-word
(16-bit) accesses trigger an AHB bus error.

31.10.1

SDMMC power control register (SDMMC_POWER)
Address offset: 0x000
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

DIR
POL

VSWI
TCHEN

VSWI
TCH

rw

rw

rw

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

PWRCTRL[1:0]
rw

rw

Bits 31:5 Reserved, must be kept at reset value.
Bit 4 DIRPOL: Data and command direction signals polarity selection
This bit can only be written when the SDMMC is in the power-off state (PWRCTRL = 00).
0: Voltage transceiver I/Os driven as output when direction signal is low.
1: Voltage transceiver I/Os driven as output when direction signal is high.
Bit 3 VSWITCHEN: Voltage switch procedure enable
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
This bit is used to stop the SDMMC_CK after the voltage switch command response:
0: SDMMC_CK clock kept unchanged after successfully received command response.
1: SDMMC_CK clock stopped after successfully received command response.
Bit 2 VSWITCH: Voltage switch sequence start
This bit is used to start the timing critical section of the voltage switch sequence:
0: Voltage switch sequence not started and not active.
1: Voltage switch sequence started or active.
Bits 1:0 PWRCTRL[1:0]: SDMMC state control bits
These bits can only be written when the SDMMC is not in the power-on state
(PWRCTRL ≠ 11).
These bits are used to define the functional state of the SDMMC signals:
00: After reset, Reset: the SDMMC is disabled and the clock to the Card is stopped,
SDMMC_D[7:0], and SDMMC_CMD are HiZ and SDMMC_CK is driven low.
When written 00, power-off: the SDMMC is disabled and the clock to the card is
stopped, SDMMC_D[7:0], SDMMC_CMD and SDMMC_CK are driven high.
01: Reserved (When written 01, PWRCTRL value does not change)
10: Power-cycle, the SDMMC is disabled and the clock to the card is stopped,
SDMMC_D[7:0], SDMMC_CMD and SDMMC_CK are driven low.
11: Power-on: the card is clocked, The first 74 SDMMC_CK cycles the SDMMC is still
disabled. After the 74 cycles the SDMMC is enabled and the SDMMC_D[7:0],
SDMMC_CMD and SDMMC_CK are controlled according the SDMMC operation.
Any further write is ignored, PWRCTRL value keeps 11.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.2

RM0456

SDMMC clock control register (SDMMC_CLKCR)
Address offset: 0x004
Reset value: 0x0000 0000
This register controls the SDMMC_CK output clock, the sdmmc_rx_ck receive clock, and
the bus width.

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

Res.

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

Res.

PWR
SAV

Res.

Res.

WID
BUS[1:0]
rw

rw

rw

21

20

19

BUS
SELCLKRX[1:0]
SPEED

18

17

16

DDR

HWFC
_EN

NEG
EDGE

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

rw

rw

rw

rw

CLKDIV[9:0]
rw

rw

rw

rw

rw

rw

Bits 31:22 Reserved, must be kept at reset value.
Bits 21:20 SELCLKRX[1:0]: Receive clock selection
These bits can only be written when the CPSM and DPSM are not active (CPSMACT = 0
and DPSMACT = 0)
00: sdmmc_io_in_ck selected as receive clock
01: SDMMC_CKIN feedback clock selected as receive clock
10: sdmmc_fb_ck tuned feedback clock selected as receive clock
11: Reserved (select sdmmc_io_in_ck)
Bit 19 BUSSPEED: Bus speed for selection of SDMMC operating modes
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0)
0: DS, HS, SDR12, SDR25, Legacy compatible, High speed SDR, High speed DDR bus
speed mode selected
1: SDR50, DDR50, SDR104, HS200 bus speed mode selected
Bit 18 DDR: Data rate signaling selection
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0)
DDR rate must only be selected with 4-bit or 8-bit wide bus mode. (WIDBUS > 00). DDR = 1
has no effect when WIDBUS = 00 (1-bit wide bus).
DDR rate must only be selected with clock division > 1 (CLKDIV > 0).
0: SDR Single data rate signaling
1: DDR double data rate signaling
Bit 17 HWFC_EN: Hardware flow control enable
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0)
0: Hardware flow control is disabled
1: Hardware flow control is enabled
When Hardware flow control is enabled, the meaning of the TXFIFOE and RXFIFOF flags
change, see SDMMC status register definition in Section 31.10.11.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Bit 16 NEGEDGE: SDMMC_CK dephasing selection bit for data and command
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0).
When clock division = 1 (CLKDIV = 0), this bit has no effect. Data and Command change on
SDMMC_CK falling edge.
0: When clock division >1 (CLKDIV > 0) and DDR = 0:
– Command and data changed on the sdmmc_ker_ck falling edge succeeding the rising
edge of SDMMC_CK.
– SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
When clock division >1 (CLKDIV > 0) and DDR = 1:
– Command changed on the sdmmc_ker_ck falling edge succeeding the rising edge of
SDMMC_CK.
– Data changed on the sdmmc_ker_ck falling edge succeeding a SDMMC_CK edge.
– SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
1: When clock division >1 (CLKDIV > 0) and DDR = 0:
– Command and data changed on the same sdmmc_ker_ck rising edge generating the
SDMMC_CK falling edge.
When clock division >1 (CLKDIV > 0) and DDR = 1:
– Command changed on the same sdmmc_ker_ck rising edge generating the
SDMMC_CK falling edge.
– Data changed on the SDMMC_CK falling edge succeeding a SDMMC_CK edge.
– SDMMC_CK edge occurs on sdmmc_ker_ck rising edge.
Bits 15:14 WIDBUS[1:0]: Wide bus mode enable bit
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0)
00: Default 1-bit wide bus mode: SDMMC_D0 used (Does not support DDR)
01: 4-bit wide bus mode: SDMMC_D[3:0] used
10: 8-bit wide bus mode: SDMMC_D[7:0] used
Bit 13 Reserved, must be kept at reset value.
Bit 12 PWRSAV: Power saving configuration bit
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0)
For power saving, the SDMMC_CK clock output can be disabled when the bus is idle by
setting PWRSAV:
0: SDMMC_CK clock is always enabled
1: SDMMC_CK is only enabled when the bus is active
Bits 11:10 Reserved, must be kept at reset value.
Bits 9:0 CLKDIV[9:0]: Clock divide factor
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0).
This field defines the divide factor between the input clock (sdmmc_ker_ck) and the output
clock (SDMMC_CK): SDMMC_CK frequency = sdmmc_ker_ck / [2 * CLKDIV].
0x000: SDMMC_CK frequency = sdmmc_ker_ck / 1 (Does not support DDR)
0x001: SDMMC_CK frequency = sdmmc_ker_ck / 2
0x002: SDMMC_CK frequency = sdmmc_ker_ck / 4
0x0XX: ..
0x080: SDMMC_CK frequency = sdmmc_ker_ck / 256
0xXXX: ..
0x3FF: SDMMC_CK frequency = sdmmc_ker_ck / 2046

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)
Note:

RM0456

While the SD/SDIO card or e•MMC is in identification mode, the SDMMC_CK frequency
must be less than 400 kHz.
The clock frequency can be changed to the maximum card bus frequency when relative
card addresses are assigned to all cards.
At least seven sdmmc_hclk clock periods are needed between two write accesses to this
register. SDMMC_CK can also be stopped during the Read Wait interval for SD I/O cards: in
this case the SDMMC_CLKCR register does not control SDMMC_CK.

31.10.3

SDMMC argument register (SDMMC_ARGR)
Address offset: 0x008
Reset value: 0x0000 0000
This register contains a 32-bit command argument, which is sent to a card as part of a
command message.

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

CMDARG[31:16]
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

CMDARG[15:0]
rw

rw

Bits 31:0 CMDARG[31:0]: Command argument
These bits can only be written by firmware when CPSM is disabled (CPSMEN = 0).
Command argument sent to a card as part of a command message. If a command contains
an argument, it must be loaded into this register before writing a command to the command
register.

31.10.4

SDMMC command register (SDMMC_CMDR)
Address offset: 0x00C
Reset value: 0x0000 0000
This register contains the command index and command type bits. The command index is
sent to a card as part of a command message. The command type bits control the
command path state machine (CPSM).

31
Res.

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25
Res.

24
Res.

23
Res.

22
Res.

21
Res.

20
Res.

19
Res.

18
Res.

17

16

Res.

CMD
SUS
PEND
rw

15

14

13

12

11

10

BOOT
EN

BOOT
MODE

DT
HOLD

CPSM
EN

WAITP
END

WAIT
INT

rw

rw

rw

rw

rw

rw

<!-- pagebreak -->

9

8

WAITRESP[1:0]
rw

rw

7

6

CMD
STOP

CMD
TRANS

rw

rw

RM0456 Rev 6

5

4

3

2

1

0

rw

rw

CMDINDEX[5:0]
rw

rw

rw

rw

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 CMDSUSPEND: The CPSM treats the command as a Suspend or Resume command and
signals interrupt period start/end
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
CMDSUSPEND = 1 and CMDTRANS = 0 Suspend command, start interrupt period when
response bit BS = 0.
CMDSUSPEND = 1 and CMDTRANS = 1 Resume command with data, end interrupt period
when response bit DF = 1.
Bit 15 BOOTEN: Enable boot mode procedure
0: Boot mode procedure disabled
1: Boot mode procedure enabled
Bit 14 BOOTMODE: Select the boot mode procedure to be used
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0)
0: Normal boot mode procedure selected
1: Alternative boot mode procedure selected
Bit 13 DTHOLD: Hold new data block transmission and reception in the DPSM
If this bit is set, the DPSM does not move from the Wait_S state to the Send state or from the
Wait_R state to the Receive state.
Bit 12

CPSMEN: Command path state machine (CPSM) enable bit
This bit is written 1 by firmware, and cleared by hardware when the CPSM enters the Idle
state.
If this bit is set, the CPSM is enabled.
When DTEN = 1, no command is transfered nor boot procedure is started. CPSMEN is
cleared to 0.
During Read Wait with SDMMC_CK stopped no command is sent and CPSMEN is kept 0.

Bit 11 WAITPEND: CPSM waits for end of data transfer (CmdPend internal signal) from DPSM
This bit when set, the CPSM waits for the end of data transfer trigger before it starts sending
a command.
WAITPEND is only taken into account when DTMODE = e•MMC stream data transfer,
WIDBUS = 1-bit wide bus mode, DPSMACT = 1 and DTDIR = from host to card.
Bit 10 WAITINT: CPSM waits for interrupt request
If this bit is set, the CPSM disables command timeout and waits for an card interrupt request
(Response).
If this bit is cleared in the CPSM Wait state, it causes the abort of the interrupt mode.
Bits 9:8 WAITRESP[1:0]: Wait for response bits
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
They are used to configure whether the CPSM is to wait for a response, and if yes, which
kind of response.
00: No response, expect CMDSENT flag
01: Short response, expect CMDREND or CCRCFAIL flag
10: Short response, expect CMDREND flag (No CRC)
11: Long response, expect CMDREND or CCRCFAIL flag

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bit 7 CMDSTOP: The CPSM treats the command as a Stop Transmission command and signals
abort to the DPSM
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
If this bit is set, the CPSM issues the abort signal to the DPSM when the command is sent.
Bit 6 CMDTRANS: The CPSM treats the command as a data transfer command, stops the interrupt
period, and signals DataEnable to the DPSM
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
If this bit is set, the CPSM issues an end of interrupt period and issues DataEnable signal to
the DPSM when the command is sent.
Bits 5:0 CMDINDEX[5:0]: Command index
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
The command index is sent to the card as part of a command message.

Note:

1

At least seven sdmmc_hclk clock periods are needed between two write accesses to this
register.

2

MultiMediaCard can send two kinds of response: short responses, 48 bits, or long
responses,136 bits. SD card and SD I/O card can send only short responses, the argument
can vary according to the type of response: the software distinguishes the type of response
according to the send command.

31.10.5

SDMMC command response register (SDMMC_RESPCMDR)
Address offset: 0x010
Reset value: 0x0000 0000
This register contains the command index field of the last command response received. If
the command response transmission does not contain the command index field (long or
OCR response), the RESPCMD field is unknown, although it must contain 111111b (the
value of the reserved field from the response).

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

5

4

3

2

1

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

RESPCMD[5:0]
r

r

r

r

Bits 31:6 Reserved, must be kept at reset value.
Bits 5:0 RESPCMD[5:0]: Response command index
Read-only bitfield. Contains the command index of the last command response received.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.6

SDMMC response x register (SDMMC_RESPxR)
Address offset: 0x010 + 0x004 * x, (x = 1 to 4)
Reset value: 0x0000 0000
These registers contain the status of a card, which is part of the received response.

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

CARDSTATUS[31:16]
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

4

3

2

1

0

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

r

r

r

r

CARDSTATUS[15:0]
r

r

Bits 31:0 CARDSTATUS[31:0]: Card status according table below
See Table 296.

The card status size is 32 or 128 bits, depending on the response type.
Table 296. Response type and SDMMC_RESPxR registers
Register(1)

Short response

Long response

SDMMC_RESP1R

Card status[31:0]

Card status [127:96]

SDMMC_RESP2R

all 0

Card status [95:64]

SDMMC_RESP3R

all 0

Card status [63:32]

SDMMC_RESP4R

all 0

Card status [31:0](2)

1. The most significant bit of the card status is received first.
2. The SDMMC_RESP4R register LSB is always 0.

31.10.7

SDMMC data timer register (SDMMC_DTIMER)
Address offset: 0x024
Reset value: 0x0000 0000
This register contains the data timeout period, in card bus clock periods.
A counter loads the value from this register, and starts decrementing when the data path
state machine (DPSM) enters the Wait_R or Busy state. If the timer reaches 0 while the
DPSM is in either of these states, the timeout status flag is set.

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

DATATIME[31:16]
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

DATATIME[15:0]
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

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bits 31:0 DATATIME[31:0]: Data and R1b busy timeout period
This bit can only be written when the CPSM and DPSM are not active (CPSMACT = 0 and
DPSMACT = 0).
Data and R1b busy timeout period expressed in card bus clock periods.

Note:

A data transfer must be written to the data timer register and the data length register before
being written to the data control register.

31.10.8

SDMMC data length register (SDMMC_DLENR)
Address offset: 0x028
Reset value: 0x0000 0000
This register contains the number of data bytes to be transferred. The value is loaded into
the data counter when data transfer starts.

31

30

29

28

27

26

25

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

24

23

22

21

20

19

18

17

16

DATALENGTH[24:16]
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

DATALENGTH[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 DATALENGTH[24:0]: Data length value
This register can only be written by firmware when DPSM is inactive (DPSMACT = 0).
Number of data bytes to be transferred.
When DDR = 1 DATALENGTH is truncated to a multiple of 2. (The last odd byte is not
transfered)
When DATALENGTH = 0 no data are transfered, when requested by a CPSMEN and
CMDTRANS = 1 also no command is transfered. DTEN and CPSMEN are cleared to 0.

Note:

For a block data transfer, the value in the data length register must be a multiple of the block
size (see SDMMC_DCTRL). A data transfer must be written to the data timer register and
the data length register before being written to the data control register.
For an SDMMC multibyte transfer the value in the data length register must be between 1
and 512.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.9

SDMMC data control register (SDMMC_DCTRL)
Address offset: 0x02C
Reset value: 0x0000 0000
This register controls the data path state machine (DPSM).

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

FIFO
RST

BOOT
ACK
EN

SDIO
EN

RW
MOD

RW
STOP

RW
START

DTDIR

DTEN

rw

rw

rw

rw

rw

rw

rw

rw

Res.

Res.

DBLOCKSIZE[3:0]
rw

rw

rw

DTMODE[1:0]
rw

rw

rw

Bits 31:14 Reserved, must be kept at reset value.
Bit 13 FIFORST: FIFO reset, flushes any remaining data
This bit can only be written by firmware when IDMAEN= 0 and DPSM is active
(DPSMACT = 1). This bit only takes effect when a transfer error or transfer hold occurs.
0: FIFO not affected.
1: Flush any remaining data and reset the FIFO pointers. This bit is automatically cleared to
0 by hardware when DPSM gets inactive (DPSMACT = 0).
Bit 12 BOOTACKEN: Enable the reception of the boot acknowledgment
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: Boot acknowledgment disabled, not expected to be received
1: Boot acknowledgment enabled, expected to be received
Bit 11 SDIOEN: SD I/O interrupt enable functions
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
If this bit is set, the DPSM enables the SD I/O card specific interrupt operation.
Bit 10 RWMOD: Read Wait mode
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: Read Wait control using SDMMC_D2
1: Read Wait control stopping SDMMC_CK
Bit 9 RWSTOP: Read Wait stop
This bit is written by firmware and auto cleared by hardware when the DPSM moves from the
R_W state to the Wait_R or Idle state.
0: No Read Wait stop
1: Enable for Read Wait stop when DPSM is in the R_W state.
Bit 8 RWSTART: Read Wait start
If this bit is set, Read Wait operation starts.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bits 7:4 DBLOCKSIZE[3:0]: Data block size
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
Define the data block length when the block data transfer mode is selected:
0000: Block length = 20 = 1 byte
0001: Block length = 21 = 2 bytes
0010: Block length = 22 = 4 bytes
0011: Block length = 23 = 8 bytes
0100: Block length = 24 = 16 bytes
0101: Block length = 25 = 32 bytes
0110: Block length = 26 = 64 bytes
0111: Block length = 27 = 128 bytes
1000: Block length = 28 = 256 bytes
1001: Block length = 29 = 512 bytes
1010: Block length = 210 = 1024 bytes
1011: Block length = 211 = 2048 bytes
1100: Block length = 212 = 4096 bytes
1101: Block length = 213 = 8192 bytes
1110: Block length = 214 = 16384 bytes
1111: Reserved
When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a
multiple of DBLOCKSIZE. (None of the remaining data are transfered.)
When DDR = 1, DBLOCKSIZE = 0000 must not be used. (No data are transfered)
Bits 3:2 DTMODE[1:0]: Data transfer mode selection
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
00: Block data transfer ending on block count.
01: SDIO multibyte data transfer.
10: e•MMC Stream data transfer. (WIDBUS must select 1-bit wide bus mode)
11: Block data transfer ending with STOP_TRANSMISSION command (not to be used with
DTEN initiated data transfers).
Bit 1 DTDIR: Data transfer direction selection
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: From host to card.
1: From card to host.
Bit 0 DTEN: Data transfer enable bit
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is
cleared by hardware when data transfer completes.
This bit must only be used to transfer data when no associated data transfer command is
used (must not be used with SD or e•MMC cards).
0: Do not start data transfer without CPSM data transfer command.
1: Start data transfer without CPSM data transfer command.

31.10.10 SDMMC data counter register (SDMMC_DCNTR)
Address offset: 0x030
Reset value: 0x0000 0000
This register loads the value from the data length register (see SDMMC_DLENR) when the
DPSM moves from the Idle state to the Wait_R or Wait_S state. As data is transferred, the
counter decrements the value until it reaches 0. The DPSM then moves to the Idle state and

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
when there has been no error, and no transmit data transfer hold, the data status end flag
(DATAEND) is set.

31

30

29

28

27

26

25

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

24

23

22

21

20

19

18

17

16

DATACOUNT[24:16]
r

r

r

r

r

r

r

r

r

8

7

6

5

4

3

2

1

0

r

r

r

r

r

r

r

DATACOUNT[15:0]
r

r

r

r

r

r

r

r

r

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 DATACOUNT[24:0]: Data count value
When read, the number of remaining data bytes to be transferred is returned. Write has no
effect.

Note:

This register must be read only after the data transfer is complete, or hold. When reading
after an error event the read data count value may be different from the real number of data
bytes transfered.

31.10.11 SDMMC status register (SDMMC_STAR)
Address offset: 0x034
Reset value: 0x0000 0000
This register is a read-only register. It contains two types of flag:

31

30

•

Static flags (bits [28, 21, 11:0]): these bits remain asserted until they are cleared by
writing to the SDMMC interrupt Clear register (see SDMMC_ICR)

•

Dynamic flags (bits [20:12]): these bits change state depending on the state of the
underlying logic (for example, FIFO full and empty flags are asserted and deasserted
as data while written to the FIFO)
29

28

27
IDMA
TE

26
CK
STOP

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

VSW
END

ACK
TIME
OUT

ACK
FAIL

SDIOIT

BUSY
D0END

BUSY
D0

RX
FIFOE

TX
FIFOE

RX
FIFOF

TX
FIFOF

Res.

Res.

Res.

IDMA
BTC
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

RX
FIFO
HF

TX
FIFO
HE

CPSM
ACT

DPSM
ACT

DA
BORT

DATA
END

CMD
SENT

D
TIME
OUT

C
TIME
OUT

DCRC
FAIL

CCRC
FAIL

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

r

DBCK
DHOLD
END
r

r

TX
CMDR
RX
UNDER
END OVERR
R
r

r

r

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 IDMABTC: IDMA buffer transfer complete
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 27 IDMATE: IDMA transfer error
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 26 CKSTOP: SDMMC_CK stopped in Voltage switch procedure
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bit 25 VSWEND: Voltage switch critical timing section completion
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 24 ACKTIMEOUT: Boot acknowledgment timeout
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 23 ACKFAIL: Boot acknowledgment received (boot acknowledgment check fail)
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 22 SDIOIT: SDIO interrupt received
The interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 21 BUSYD0END: end of SDMMC_D0 Busy following a CMD response detected
This indicates only end of busy following a CMD response. This bit does not signal busy due
to data transfer. Interrupt flag is cleared by writing corresponding interrupt clear bit in
SDMMC_ICR.
0: card SDMMC_D0 signal does NOT signal change from busy to not busy.
1: card SDMMC_D0 signal changed from busy to NOT busy.
Bit 20 BUSYD0: Inverted value of SDMMC_D0 line (Busy), sampled at the end of a CMD response
and a second time 2 SDMMC_CK cycles after the CMD response
This bit is reset to not busy when the SDMMCD0 line changes from busy to not busy. This bit
does not signal busy due to data transfer. This is a hardware status flag only, it does not
generate an interrupt.
0: card signals not busy on SDMMC_D0.
1: card signals busy on SDMMC_D0.
Bit 19 RXFIFOE: Receive FIFO empty
This is a hardware status flag only, does not generate an interrupt. This bit is cleared when
one FIFO location becomes full.
Bit 18 TXFIFOE: Transmit FIFO empty
This bit is cleared when one FIFO location becomes full.
Bit 17 RXFIFOF: Receive FIFO full
This bit is cleared when one FIFO location becomes empty.
Bit 16 TXFIFOF: Transmit FIFO full
This is a hardware status flag only, does not generate an interrupt. This bit is cleared when
one FIFO location becomes empty.
Bit 15 RXFIFOHF: Receive FIFO half full
There are at least half the number of words in the FIFO. This bit is cleared when the FIFO
becomes half+1 empty.
Bit 14 TXFIFOHE: Transmit FIFO half empty
At least half the number of words can be written into the FIFO. This bit is cleared when the
FIFO becomes half+1 full.
Bit 13 CPSMACT: Command path state machine active (not in Idle state)
This is a hardware status flag only, does not generate an interrupt.
Bit 12 DPSMACT: Data path state machine active (not in Idle state)
This is a hardware status flag only, does not generate an interrupt.
Bit 11 DABORT: Data transfer aborted by CMD12
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Bit 10 DBCKEND: Data block sent/received
DBCKEND is set when:
- CRC check passed and DPSM moves to the R_W state
or
- IDMAEN = 0 and transmit data transfer hold and DATACOUNT >0 and DPSM moves to
Wait_S.
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 9 DHOLD: Data transfer Hold
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 8 DATAEND: Data transfer ended correctly
DATAEND is set if data counter DATACOUNT is zero and no errors occur, and no transmit
data transfer hold.
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 7 CMDSENT: Command sent (no response required)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 6 CMDREND: Command response received (CRC check passed, or no CRC)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 5 RXOVERR: Received FIFO overrun error (masked by hardware when IDMA is enabled)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 4 TXUNDERR: Transmit FIFO underrun error (masked by hardware when IDMA is enabled)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 3 DTIMEOUT: Data timeout
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 2 CTIMEOUT: Command response timeout
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
The Command Timeout period has a fixed value of 64 SDMMC_CK clock periods.
Bit 1 DCRCFAIL: Data block sent/received (CRC check failed)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.
Bit 0 CCRCFAIL: Command response received (CRC check failed)
Interrupt flag is cleared by writing corresponding interrupt clear bit in SDMMC_ICR.

Note:

FIFO interrupt flags must be masked in SDMMC_MASKR when using IDMA mode.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

31.10.12 SDMMC interrupt clear register (SDMMC_ICR)
Address offset: 0x038
Reset value: 0x0000 0000
This register is a write-only register. Writing a bit with 1 clears the corresponding bit in the
SDMMC_STAR status register.
31

30

29

28

27

26

25

23

22

21

20

19

18

17

16

ACK
FAILC

SDIO
ITC

BUSY
D0
ENDC

Res.

Res.

Res.

Res.

Res.

4

3

Res.

Res.

Res.

IDMA
BTCC

IDMA
TEC

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

Res.

Res.

Res.

Res.

CK
VSW
STOPC ENDC

24
ACK
TIME
OUTC

D
DBCK DHOLD DATA
ABORT
ENDC
C
ENDC
C
rw

rw

rw

RX
TX
D
CMD CMDR
OVERR UNDER TIME
SENTC ENDC
C
RC
OUTC

rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 IDMABTCC: IDMA buffer transfer complete clear bit
Set by software to clear the IDMABTC flag.
0: IDMABTC not cleared
1: IDMABTC cleared
Bit 27 IDMATEC: IDMA transfer error clear bit
Set by software to clear the IDMATE flag.
0: IDMATE not cleared
1: IDMATE cleared
Bit 26 CKSTOPC: CKSTOP flag clear bit
Set by software to clear the CKSTOP flag.
0: CKSTOP not cleared
1: CKSTOP cleared
Bit 25 VSWENDC: VSWEND flag clear bit
Set by software to clear the VSWEND flag.
0: VSWEND not cleared
1: VSWEND cleared
Bit 24 ACKTIMEOUTC: ACKTIMEOUT flag clear bit
Set by software to clear the ACKTIMEOUT flag.
0: ACKTIMEOUT not cleared
1: ACKTIMEOUT cleared
Bit 23 ACKFAILC: ACKFAIL flag clear bit
Set by software to clear the ACKFAIL flag.
0: ACKFAIL not cleared
1: ACKFAIL cleared
Bit 22 SDIOITC: SDIOIT flag clear bit
Set by software to clear the SDIOIT flag.
0: SDIOIT not cleared
1: SDIOIT cleared

<!-- pagebreak -->

RM0456 Rev 6

rw

rw

rw

2

1

0

C
TIME
OUTC

DCRC
FAILC

CCRC
FAILC

rw

rw

rw

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Bit 21 BUSYD0ENDC: BUSYD0END flag clear bit
Set by software to clear the BUSYD0END flag.
0: BUSYD0END not cleared
1: BUSYD0END cleared
Bits 20:12 Reserved, must be kept at reset value.
Bit 11 DABORTC: DABORT flag clear bit
Set by software to clear the DABORT flag.
0: DABORT not cleared
1: DABORT cleared
Bit 10 DBCKENDC: DBCKEND flag clear bit
Set by software to clear the DBCKEND flag.
0: DBCKEND not cleared
1: DBCKEND cleared
Bit 9 DHOLDC: DHOLD flag clear bit
Set by software to clear the DHOLD flag.
0: DHOLD not cleared
1: DHOLD cleared
Bit 8 DATAENDC: DATAEND flag clear bit
Set by software to clear the DATAEND flag.
0: DATAEND not cleared
1: DATAEND cleared
Bit 7 CMDSENTC: CMDSENT flag clear bit
Set by software to clear the CMDSENT flag.
0: CMDSENT not cleared
1: CMDSENT cleared
Bit 6 CMDRENDC: CMDREND flag clear bit
Set by software to clear the CMDREND flag.
0: CMDREND not cleared
1: CMDREND cleared
Bit 5 RXOVERRC: RXOVERR flag clear bit
Set by software to clear the RXOVERR flag.
0: RXOVERR not cleared
1: RXOVERR cleared
Bit 4 TXUNDERRC: TXUNDERR flag clear bit
Set by software to clear TXUNDERR flag.
0: TXUNDERR not cleared
1: TXUNDERR cleared
Bit 3 DTIMEOUTC: DTIMEOUT flag clear bit
Set by software to clear the DTIMEOUT flag.
0: DTIMEOUT not cleared
1: DTIMEOUT cleared

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bit 2 CTIMEOUTC: CTIMEOUT flag clear bit
Set by software to clear the CTIMEOUT flag.
0: CTIMEOUT not cleared
1: CTIMEOUT cleared
Bit 1 DCRCFAILC: DCRCFAIL flag clear bit
Set by software to clear the DCRCFAIL flag.
0: DCRCFAIL not cleared
1: DCRCFAIL cleared
Bit 0 CCRCFAILC: CCRCFAIL flag clear bit
Set by software to clear the CCRCFAIL flag.
0: CCRCFAIL not cleared
1: CCRCFAIL cleared

31.10.13 SDMMC mask register (SDMMC_MASKR)
Address offset: 0x03C
Reset value: 0x0000 0000
This register determines which status flags generate an interrupt request by setting the
corresponding bit to 1.
31

30

29

28

27

26

25

24

Res.

CK
STOP
IE

VSW
ENDIE

ACK
TIME
OUTIE

rw

rw

10

9

Res.

Res.

Res.

IDMA
BTCIE

15

14

13

12

11

RX
FIFO
HFIE

TX
FIFO
HEIE

Res.

DA
BORT
IE

rw

rw

rw

Res.

rw

22

21

ACK
FAILIE

SDIO
ITIE

BUSY
D0
ENDIE

rw

rw

rw

rw

8

7

6

5

DBCK DHOLD DATA
ENDIE
IE
ENDIE
rw

rw

23

CMD
SENT
IE

CMDR
ENDIE

rw

rw

rw

20

19

18

17

16

Res.

Res.

TX
FIFO
EIE

RX
FIFO
FIE

Res.

rw

rw

4

3

2

1

0

C
TIME
OUTIE

DCRC
FAILIE

CCRC
FAILIE

rw

rw

rw

RX
TX
D
OVER UNDER TIME
RIE
RIE
OUTIE
rw

rw

rw

Bits 31:29 Reserved, must be kept at reset value.
Bit 28 IDMABTCIE: IDMA buffer transfer complete interrupt enable
Set and cleared by software to enable/disable the interrupt generated when the IDMA has
transferred all data belonging to a memory buffer.
0: IDMA buffer transfer complete interrupt disabled
1: IDMA buffer transfer complete interrupt enabled
Bit 27 Reserved, must be kept at reset value.
Bit 26 CKSTOPIE: Voltage switch clock stopped interrupt enable
Set and cleared by software to enable/disable interrupt caused by voltage switch clock
stopped.
0: Voltage switch clock stopped interrupt disabled
1: Voltage switch clock stopped interrupt enabled
Bit 25 VSWENDIE: Voltage switch critical timing section completion interrupt enable
Set and cleared by software to enable/disable the interrupt generated when voltage switch
critical timing section completion.
0: Voltage switch critical timing section completion interrupt disabled
1: Voltage switch critical timing section completion interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Bit 24 ACKTIMEOUTIE: Acknowledgment timeout interrupt enable
Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout.
0: Acknowledgment timeout interrupt disabled
1: Acknowledgment timeout interrupt enabled
Bit 23 ACKFAILIE: Acknowledgment fail interrupt enable
Set and cleared by software to enable/disable interrupt caused by acknowledgment fail.
0: Acknowledgment fail interrupt disabled
1: Acknowledgment fail interrupt enabled
Bit 22 SDIOITIE: SDIO mode interrupt received interrupt enable
Set and cleared by software to enable/disable the interrupt generated when receiving the
SDIO mode interrupt.
0: SDIO mode interrupt received interrupt disabled
1: SDIO mode interrupt received interrupt enabled
Bit 21 BUSYD0ENDIE: BUSYD0END interrupt enable
Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0
signal changes from busy to NOT busy following a CMD response.
0: BUSYD0END interrupt disabled
1: BUSYD0END interrupt enabled
Bits 20:19 Reserved, must be kept at reset value.
Bit 18 TXFIFOEIE: Tx FIFO empty interrupt enable
Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty.
0: Tx FIFO empty interrupt disabled
1: Tx FIFO empty interrupt enabled
Bit 17 RXFIFOFIE: Rx FIFO full interrupt enable
Set and cleared by software to enable/disable interrupt caused by Rx FIFO full.
0: Rx FIFO full interrupt disabled
1: Rx FIFO full interrupt enabled
Bit 16 Reserved, must be kept at reset value.
Bit 15 RXFIFOHFIE: Rx FIFO half full interrupt enable
Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full.
0: Rx FIFO half full interrupt disabled
1: Rx FIFO half full interrupt enabled
Bit 14 TXFIFOHEIE: Tx FIFO half empty interrupt enable
Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty.
0: Tx FIFO half empty interrupt disabled
1: Tx FIFO half empty interrupt enabled
Bits 13:12 Reserved, must be kept at reset value.
Bit 11 DABORTIE: Data transfer aborted interrupt enable
Set and cleared by software to enable/disable interrupt caused by a data transfer being
aborted.
0: Data transfer abort interrupt disabled
1: Data transfer abort interrupt enabled
Bit 10 DBCKENDIE: Data block end interrupt enable
Set and cleared by software to enable/disable interrupt caused by data block end.
0: Data block end interrupt disabled
1: Data block end interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bit 9 DHOLDIE: Data hold interrupt enable
Set and cleared by software to enable/disable the interrupt generated when sending new
data is hold in the DPSM Wait_S state.
0: Data hold interrupt disabled
1: Data hold interrupt enabled
Bit 8 DATAENDIE: Data end interrupt enable
Set and cleared by software to enable/disable interrupt caused by data end.
0: Data end interrupt disabled
1: Data end interrupt enabled
Bit 7 CMDSENTIE: Command sent interrupt enable
Set and cleared by software to enable/disable interrupt caused by sending command.
0: Command sent interrupt disabled
1: Command sent interrupt enabled
Bit 6 CMDRENDIE: Command response received interrupt enable
Set and cleared by software to enable/disable interrupt caused by receiving command
response.
0: Command response received interrupt disabled
1: command Response received interrupt enabled
Bit 5 RXOVERRIE: Rx FIFO overrun error interrupt enable
Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error.
0: Rx FIFO overrun error interrupt disabled
1: Rx FIFO overrun error interrupt enabled
Bit 4 TXUNDERRIE: Tx FIFO underrun error interrupt enable
Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error.
0: Tx FIFO underrun error interrupt disabled
1: Tx FIFO underrun error interrupt enabled
Bit 3 DTIMEOUTIE: Data timeout interrupt enable
Set and cleared by software to enable/disable interrupt caused by data timeout.
0: Data timeout interrupt disabled
1: Data timeout interrupt enabled
Bit 2 CTIMEOUTIE: Command timeout interrupt enable
Set and cleared by software to enable/disable interrupt caused by command timeout.
0: Command timeout interrupt disabled
1: Command timeout interrupt enabled
Bit 1 DCRCFAILIE: Data CRC fail interrupt enable
Set and cleared by software to enable/disable interrupt caused by data CRC failure.
0: Data CRC fail interrupt disabled
1: Data CRC fail interrupt enabled
Bit 0 CCRCFAILIE: Command CRC fail interrupt enable
Set and cleared by software to enable/disable interrupt caused by command CRC failure.
0: Command CRC fail interrupt disabled
1: Command CRC fail interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.14 SDMMC acknowledgment timer register (SDMMC_ACKTIMER)
Address offset: 0x040
Reset value: 0x0000 0000
This register contains the acknowledgment timeout period, in SDMMC_CK bus clock
periods.
A counter loads the value from this register, and starts decrementing when the data path
state machine (DPSM) enters the Wait_Ack state. If the timer reaches 0 while the DPSM is
in this states, the acknowledgment timeout status flag is set.
31

30

29

28

27

26

25

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

rw

rw

rw

rw

rw

rw

rw

24

23

22

21

20

19

18

17

16

ACKTIME[24:16]
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

ACKTIME[15:0]
rw

rw

Bits 31:25 Reserved, must be kept at reset value.
Bits 24:0 ACKTIME[24:0]: Boot acknowledgment timeout period
This bit can only be written by firmware when CPSM is disabled (CPSMEN = 0).
Boot acknowledgment timeout period expressed in card bus clock periods.

Note:

The data transfer must be written to the acknowledgment timer register before being written
to the data control register.

31.10.15 SDMMC DMA control register (SDMMC_IDMACTRLR)
Address offset: 0x050
Reset value: 0x0000 0000
The receive and transmit FIFOs can be read or written as 32-bit wide registers. The FIFOs
contain 32 entries on 32 sequential addresses. This enables the CPU to use its load and
store multiple operands to read from/write to the FIFO.
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

IDMAB
MODE

IDMA
EN

rw

rw

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

Res.

Res.

Res.

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bits 31:2 Reserved, must be kept at reset value.
Bit 1 IDMABMODE: Buffer mode selection
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: Single buffer mode.
1: Linked list mode.
Bit 0 IDMAEN: IDMA enable
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: IDMA disabled
1: IDMA enabled

31.10.16 SDMMC IDMA buffer size register (SDMMC_IDMABSIZER)
Address offset: 0x054
Reset value: 0x0000 0000
This register contains the buffer size when in linked list configuration.
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

IDMA
BNDT
[11]

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

rw

IDMABNDT[10:0]
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

Bits 31:17 Reserved, must be kept at reset value.
Bits 16:5 IDMABNDT[11:0]: Number of bytes per buffer
This 12-bit value must be multiplied by 8 to get the size of the buffer in 32-bit words and by
32 to get the size of the buffer in bytes.
Example: IDMABNDT = 0x001: buffer size = 8 words = 32 bytes.
Example: IDMABNDT = 0x800: buffer size = 16384 words = 64 Kbyte.
These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
Bits 4:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.17 SDMMC IDMA buffer base address register
(SDMMC_IDMABASER)
Address offset: 0x058
Reset value: 0x0000 0000
This register contains the memory buffer base address in single buffer configuration and
linked list configuration.
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

IDMABASE[31:16]
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

r

r

IDMABASE[15:0]
rw

rw

Bits 31:0 IDMABASE[31:0]: Buffer memory base address bits [31:2], must be word aligned (bit [1:0] are
always 0 and read only)
This register can be written by firmware when DPSM is inactive (DPSMACT = 0), and can
dynamically be written by firmware when DPSM active (DPSMACT = 1).

31.10.18 SDMMC IDMA linked list address register (SDMMC_IDMALAR)
Address offset: 0x064
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

ULA

ULS

ABR

Res.

Res.

Res.

Res.

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

IDMALA[13:0]
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

rw

rw

rw

rw

1

0

Res.

Res.

rw

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Bit 31 ULA: Update SDMMC_IDMALAR from linked list when in linked list mode
(SDMMC_IDMACTRLR.IDMABMODE select linked list mode)
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: SDMMC_IDMALAR is not to be updated, last linked list item.
1: SDMMC_IDMALAR is to be updated from linked list table.
Bit 30 ULS: Update SDMMC_IDMABSIZE from the next linked list when in linked list mode
(SDMMC_IDMACTRLR.IDMABMODE select linked list mode and ULA = 1)
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
0: SDMMC_IDMABSIZER is not to be updated from next linked list table.
1: SDMMC_IDMABSIZER is to be updated from next linked list table.
Bit 29 ABR: Acknowledge linked list buffer ready
This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0).
This bit is not taken into account when starting the first linked list buffer from the software
programmed register information. ABR is only taken into account on subsequent loaded
linked list items.
0: Loaded linked list buffer is not ready (this causes a linked list IDMA transfer error to be
generated).
1: Loaded linked list buffer ready acknowledge. Linked list buffer data are transfered by
IDMA.
Bits 28:16 Reserved, must be kept at reset value.
Bits 15:2 IDMALA[13:0]: Word aligned linked list item address offset
Linked list item offset pointer to the base of the next linked list item structure.
Linked list item base address is IDMABA + IDMALA.
These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
Bits 1:0 Reserved, must be kept at reset value.

31.10.19 SDMMC IDMA linked list memory base register
(SDMMC_IDMABAR)
Address offset: 0x068
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

rw

IDMABA[29:14]
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

Res.

Res.

IDMABA[13:0]
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

Bits 31:2 IDMABA[29:0]: Word aligned Linked list memory base address
Linked list memory base pointer.
These bits can only be written by firmware when DPSM is inactive (DPSMACT = 0).
Bits 1:0 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

31.10.20 SDMMC data FIFO registers x (SDMMC_FIFORx)
Address offset: 0x080 + 0x004 * x, (x =0 to 15)
Reset value: 0x0000 0000
The receive and transmit FIFOs can be only read or written as word (32-bit) wide registers.
The FIFOs contain 16 entries on sequential addresses. This enables the CPU to use its load
and store multiple operands to read from/write to the FIFO. The FIFO register interface
takes care of correct data alignment inside the FIFO, the FIFO register address used by the
CPU does matter.
When accessing SDMMC_FIFOR with half word or byte access an AHB bus fault is
generated.
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

FIFODATA[31:16]
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

FIFODATA[15:0]
rw

rw

Bits 31:0 FIFODATA[31:0]: Receive and transmit FIFO data
This register can only be read or written by firmware when the DPSM is active
(DPSMACT = 1).
The FIFO data occupies 16 entries of 32-bit words.

31.10.21 SDMMC register map

VSWITCH

PWRCTRL[1:0]

VSWITCHEN

Res.

0

0

0

CLKDIV[9:0]

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

CMDTRANS

0

0

CMDSTOP

0

0

WAITRESP[1:0]

0

Res.

0

Res.

0

PWRSAV

NEGEDGE

0

Res.

HWFC_EN

0

WIDBUS[1:0]

DDR

0

SDMMC_
ARGR
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

SDMMC_
CMDR

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

CMDSUSPEND

BOOTEN

BOOTMODE

DTHOLD

CPSMEN

WAITPEND

WAITINT

CMDARG[31:0]

Res.

0x00C

0

BUSSPEED

Reset value
0x008

SELCLKRX[1:0]

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SDMMC_
CLKCR

Res.

0x004

Res.

Reset value

DIRPOL

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SDMMC_
POWER

Res.

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

0x000

Register
name

Res.

Offset

Res.

Table 297. SDMMC register map

CMDINDEX[5:0]

Reset value

RM0456 Rev 6

<!-- pagebreak -->

1259

0x038
SDMMC_
ICR

0x03C

SDMMC_
MASKR

<!-- pagebreak -->

0

0

RM0456 Rev 6
DCRCFAIL
CCRCFAIL

0
0
0
0

0

0

CCRCFAILC

0

DCRCFAILC

0

0

0

0

0

0

0

0

0

0

0

0

0

CCRCFAILIE

CTIMEOUT

0

DCRCFAILIE

DTIMEOUT

0

CTIMEOUTC

0

CTIMEOUTIE

TXUNDERR

0

DTIMEOUTC

0

TXUNDERRC

0

DTIMEOUTIE

0

TXUNDERRIE

RXOVERR

0

RXOVERRC

0

RXOVERRIE

0

CMDRENDC

0

CMDRENDIE

0

CMDSENTC

0

CMDSENTIE

0

DATAENDC

0
0
0
DTEN

0
DTDIR

0

DTMODE[1:0]

0

RWSTART

0

RWSTOP

0

RWMOD

0

SDIOEN

Reset value
0

BOOTACKEN

0

FIFORST

0

DATAENDIE
CMDREND

0
0

DHOLDC
CMDSENT

0
0

0

DHOLDIE
DATAEND

0
0

DBCKENDC
DHOLD

0
0

0

0

DBCKENDIE

DBCKEND

0
0

0

DABORTC

DABORT

0
0
0

0

0

DABORTIE

DPSMACT

0

Res.

0
0
0

0

Res.

CPSMACT

0

Res.

0
0
0

0

0

Res.

SDMMC_
DTIMER
0

0

0

Res.

TXFIFOHE

0

Res.

0
0
0

0

Res.

0
0
0

0

Res.

SDMMC_
RESP4R
0

0

0

TXFIFOHEIE

RXFIFOHF

0

Res.

Reset value
0
0
0

0

0

RXFIFOHFIE

TXFIFOF

0

Res.

SDMMC_
RESP3R
0

Res.

0
0
0

0

0

0

Res.

RXFIFOF

0

Res.

SDMMC_
RESP2R

Res.

Reset value
0
0
0

0

0

RXFIFOFIE

TXFIFOE

0

Res.

SDMMC_
DLENR
0
0
0

0

0

Res.

SDMMC_
RESP1R

TXFIFOEIE

RXFIFOE

0

Res.

0
0
0

0

0

Res.

0
0

0

0

Res.

0
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

0

Res.

0

Res.

0
0

Res.

0

Res.
0

0

Res.

BUSYD0

0

Res.

0

0
0

Res.

0

Res.
0
0

Res.

0

Res.
0

Res.

Res.

Res.
0

Res.

Res.

Res.
0

Res.

BUSYD0END

BUSYD0ENDC

0

BUSYD0ENDIE

SDIOIT

SDIOITC

0

SDIOITIE

ACKFAIL

ACKFAILC

0

ACKFAILIE

ACKTIMEOUT

ACKTIMEOUTC

0

ACKTIMEOUTIE

VSWEND

VSWENDC

0

VSWENDIE

Res.

CKSTOP

CKSTOPC

Reset value
IDMATE

Reset value

CKSTOPIE

Reset value
0

IDMATEC

SDMMC_
STAR
0

Res.

0x034
0

IDMABTC

0x030
SDMMC_
DCNTR
0

Res.

SDMMC_
DCTRLR
0
0

IDMABTCC

0x02C
0

Res.

0x024
0
0

IDMABTCIE

0

Res.

0x028
Reset value

Res.

Reset value
0

Res.

0x020

Res.

Reset value
0

Res.

0x01C

Res.

Reset value
0

Res.

0x018

Res.

Reset value

Res.

0x014

Res.

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

SDMMC_
RESPCMDR
Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Register
name

Res.

0x010

Res.

Offset

Res.

Secure digital input/output MultiMediaCard interface (SDMMC)
RM0456

Table 297. SDMMC register map (continued)

Reset value
RESPCMD[5:0]

0
0
0
0
0
0

CARDSTATUS[31:0]

CARDSTATUS[31:0]

CARDSTATUS[31:0]

CARDSTATUS[31:0]

DATATIME[31:0]

DATALENGTH[24:0]

0
0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0

0
0
0
0
0
0
0
0
0
0
0

DBLOCK
SIZE[3:0]

DATACOUNT[24:0]

0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0
0

0

0

0

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

Secure digital input/output MultiMediaCard interface (SDMMC)

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

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

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

0

0

0

0

0

0

0

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

0

Res.

0

Res.

ABR

Reset value

Res.

SDMMC_
IDMALAR

ULS

Res.
ULA

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

IDMALA[13:0]
0

SDMMC_
IDMABAR

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

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

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

IDMABA[29:0]

Reset value

SDMMC_
FIFORx
Reset value

0

Res.

0x080 +
0x04 * x,
(x=0..15)

0

Res.

0x06C
- 0x07C

0

Res.

0x068

0

Res.

0x064

0

IDMABASE[31:0]

Res.

0x05C
- 0x060

0

Res.

Reset value

Res.

SDMMC_
IDMABASER

Res.

0x058

0

IDMABNDT[11:0]
0

0

Res.

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

Res.

Res.

Res.

SDMMC_
IDMABSIZER

Res.

0x054

Res.

Reset value

Res.

Res.

SDMMC_
IDMACTRLR

0

IDMAEN

Reserved

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

Res.

0x050

0

Res.

0x044
- 0x04C

0

Res.

Reset value

ACKTIME[24:0]

IDMABMODE Res.

Res.

Res.

Res.

SDMMC_
ACKTIMER

Res.

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

0x040

Register
name

Res.

Offset

Res.

Table 297. SDMMC register map (continued)

FIFODATA[31:0]
0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

Refer to Section 2.3 on page 140 for the register boundary addresses.

RM0456 Rev 6

<!-- pagebreak -->

