RM0456 Rev 6

RM0456

31.5.7

Secure digital input/output MultiMediaCard interface (SDMMC)

AHB and SDMMC_CK clock relation
The AHB must at least have 3x more bandwidth than the SDMMC bus bandwidth: for
example, for SDR50 4-bit mode (50 Mbyte/s), the minimum sdmmc_hclk frequency is
37.5 MHz (150 Mbyte/s).
Table 291. AHB and SDMMC_CK clock frequency relation
SDMMC bus mode

SDMMC bus
width

Maximum SDMMC_CK
[MHz]

Minimum AHB clock
[MHz]

e•MMC DS

8

26

19.5

e•MMC HS

8

52

39

e•MMC DDR52

8

52

78

e•MMC HS200

8

200

150

SD DS / SDR12

4

25

9.4

SD HS / SDR25

4

50

18.8

SD DDR50

4

50

37.5

SD SDR50

4

100

37.5

SD SDR104

4

208

78

31.6

Card functional description

31.6.1

SD I/O mode
The following features are SDMMC specific operations:
•

SDIO interrupts

•

SDIO suspend/resume operation (write and read suspend)

•

SDIO Read Wait operation by stopping the clock

•

SDIO Read Wait operation by SDMMC_D2 signaling

SDIOEN

RWMOD

RWSTOP

RWSTART

DTDIR

Table 292. SDIO special operation control

Interrupt detection

1

X

X

X

X

Suspend/Resume operation

X

X

X

X

X

Read Wait SDMMC_CK clock stop (START)

X

1

0

1

1

Read Wait SDMMC_CK clock stop (STOP)

X

1

1

1

1

Read Wait SDMMC_D2 signaling (START)

X

0

0

1

1

Read Wait SDMMC_D2 signaling (STOP)

X

0

1

1

1

Operation mode

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

SD I/O interrupts
To allow the SD I/O card to interrupt the host, an interrupt function is available on pin 8
(shared with SDMMC_D1 in 4-bit mode) on the SD interface. The use of the interrupt is
optional for each card or function within a card. The SD I/O interrupt is level-sensitive, which
means that the interrupt line must be held active (low) until it is either recognized and acted
upon by the host or deasserted due to the end of the interrupt period. After the host has
serviced the interrupt, the interrupt status bit is cleared via an I/O write to the appropriate bit
in the SD I/O card internal registers. The interrupt output of all SD I/O cards is active low and
the application must provide external pull-up resistors on all data lines (SDMMC_D[3:0]).
In SD 1-bit mode pin 8 is dedicated to the interrupt function (IRQ), and there are no timing
constraints on interrupts.
In SD 4-bit mode the host samples the level of pin 8 (SDMMC_D1/IRQ) into the interrupt
detector only during the interrupt period. At all other times, the host interrupt ignores this
value. The interrupt period begins when interrupts are enabled at the card and SDIOEN bit
is set see register settings in Table 292.
In 4-bit mode the card can generate a synchronous or asynchronous interrupt as indicated
by the card CCCR register SAI and EAI bits.
•

Synchronous interrupt, require the SDMMC_CK to be active.

•

Asynchronous interrupt, can be generated when the SDMMC_CK is stopped, 4 cycles
after the start of the card interrupt period following the last data block.
Figure 202. Asynchronous interrupt generation
SDMMC_CK
Data block

SDMMC_CMD

S Command E

SDMMC_D0

S

Last data

E

S

CRC

E

SDMMC_D1

S

Last data

E

S

CRC

E

SDMMC_D2
SDMMC_D3

S

Last data

E

S

CRC

E

Synchronous
INT

Data1

2 CK

Asynchronous INT

Data1

NEAI

4 CK
Interrupt period

MSv40191V3

The timing of the interrupt period is depending on the bus speed mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
In DS, HS, SDR12, and SDR25 mode, selected by register bit BUSSPEED, the interrupt
period is synchronous to the SD clock.

Note:

•

The interrupt period ends at the next clock from the end bit of a command that transfers
data block(s) (Command sent with the CMDTRANS bit is set), or when the DTEN bit is
set.

•

The interrupt period resumes 2 SDMMC_CK after the completion of the data block.

•

At the data block gap the interrupt period is limited to 2 SDMMC_CK cycles.

DTEN must not be used to start data transfer with SD and e•MMC cards.
Figure 203. Synchronous interrupt period data read
SDMMC_CK

SDMMC_CMD

S

Command data
E
R

SDMMC_D0

S

Data

E

S

Data

E

SDMMC_D1

S

Data

E

S

Data

E

SDMMC_D2
SDMMC_D3

S

Data

E

S

Data

E

Interrupt period

IRQ

Data1

IRQ
2 CK

Data1

2 CK
MSv40195V2

Figure 204. Synchronous interrupt period data write
SDMMC_CK

SDMMC_CMD

S

Command data
E
W

S

RSP

E

SDMMC_D0

S

Data

E

SDMMC_D1

S

Data

SDMMC_D2
SDMMC_D3

S

Data

Interrupt period

IRQ

S CRC status E

S

Data

E

E

S

Data

E

E

S

Data

E

IRQ

Data1

2 CK

Data1

2 CK
MSv40196V2

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

In SDR50, SDR104, and DDR50, selected by register bit BUSSPEED, due to propagation
delay from the card to host, the interrupt period is asynchronous.

Note:

•

The card interrupt period ends after 0 to 2 SDMMC_CK cycles after the end bit of a
command that transfers data block(s) (Command sent with the CMDTRANS bit is set),
or when the DTEN bit is set. At the host the interrupt period ends after the end bit of a
command that transfers data block(s). A card interrupt issued in the 1 to 2 cycles after
the command end bit are not detected by the host during this interrupt period.

•

The card interrupt period resumes 2 to 4 SDMMC_CK after the completion of the last
data block. The host resumes the interrupt period always 2 cycles after the last data
block.

•

There is NO interrupt period at the data block gap.

DTEN must not be used to start data transfer with SD and e•MMC cards.
Figure 205. Asynchronous interrupt period data read
SDMMC_CK

sdmmc_fb_ck

SDMMC_CMD

S

Command data
E
R

tOP

SDMMC_D0

S

Data

E

SDMMC_D1

S

Data

E

S

Data

E

SDMMC_D2
SDMMC_D3
Interrupt period

tOP
IRQ

IRQ

Data1

0 CK – 2 CK

2 CK
MSv40940V3

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
Figure 206. Asynchronous interrupt period data write
SDMMC_CK

sdmmc_fb_ck

SDMMC_CMD

S

Command data
E
W

S

RSP

E

tOP

tOP

SDMMC_D0

S

Data

E

SDMMC_D1

S

Data

E

S

Data

E

SDMMC_D2
SDMMC_D3
Interrupt period

S CRC status E

tOP

tOP

IRQ

IRQ

Data1

0 CK - 2 CK

2 CK – 4 CK
MSv40192V2

When transferring Open-ended multiple block data and using DTMODE “block data transfer
ending with STOP_TRANSMISSION command”, the SDMMC masks the interrupt period
after the last data block until the end of the CMD12 STOP_TRANSMISSION command.
The interrupt period is applicable for both memory and I/O operations.
In 4-bit mode interrupts can be differentiated from other signaling according Table 293.
Table 293. 4-bit mode Start, interrupt, and CRC-status Signaling detection
SDMMC data line

Start

Interrupt

CRC-status

SDMMC_D0

0

1 or CRC-status

0

SDMMC_D1

0

0

X

SDMMC_D2

0

1 or Read Wait

X

SDMMC_D3

0

1

X

SD I/O suspend and resume
This function is NOT supported in SDIO version 4.00 or later.
Within a multifunction SD I/O or a card with both I/O and memory functions, there are
multiple devices (I/O and memory) that share access to the e•MMC/SD bus. To share
access to the host among multiple devices, SD I/O and combo cards optionally implement
the concept of suspend/resume. When a card supports suspend/resume, the host can
temporarily halt (suspend) a data transfer operation to one function or memory to free the
bus for a higher-priority transfer to a different function or memory. After this higher-priority
transfer is complete, the original transfer is restarted (resume) where it left off.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

To perform the suspend/resume operation on the bus, the host performs the following steps:
1.

Determines the function currently using the SDMMC_D[3:0] line(s).

2.

Requests the lower-priority or slower transaction to suspend.

3.

Waits for the transaction suspension to complete.

4.

Begins the higher-priority transaction.

5.

Waits for the completion of the higher priority transaction.

6.

Restores the suspended transaction.

The card receiving a suspend command responds with its current bus status. Only when the
bus has been suspended by the card the bus status indicates suspension completed.
There are different suspend cases conditions:
•

Suspend request accepted prior to the start of data transfer.

•

Suspend request not accepted, (due to data being transfered at the same time), the
host keeps checking the request until it is accepted (data transfer has suspended).

•

Suspend request during write busy.

•

Suspend request with write multiple.

•

Suspend request during Read Wait.

For the host to know if the bus has been released it must check the status of the suspend
request, suspension completed.
When the bus status of the suspend request response indicates suspension completed, the
card has released the bus. At this time the state of the suspended operation must be saved
where after an other operation can start.
The suspend command must be sent with the CMDSUSPEND bit set. This makes possible
to start the interrupt period after the suspend command response when the bus is
suspended (response bit BS = 0).
The hardware does not save the number of remaining data to be transfered when resuming
the suspended operation. It is up to firmware to determine the data that has been
transferred and resume with the correct remaining number of data bytes.
While receiving data from the card, the SDMMC can suspend the read operation after the
read data block end (DPSM in Wait_R). After receiving the suspend acknowledgment
response from the card the following steps must be taken by firmware:
1.

The normal receive process must be stopped by setting DTHOLD bit.
a)

2.

The confirmation that all data has been read from the FIFO, and that the suspend is
completed is indicated by the DHOLD flag.
a)

Note:

<!-- pagebreak -->

The remaining number of data bytes in the FIFO must be read until the receive
FIFO is empty (RXFIFOE flag is set), and when IDMAEN = 0 the FIFO must be
reset with FIFORST.

The remaining number of data bytes (multiple of data blocks) still to be read when
resuming the operation must be determined from the remaining number of bytes
indicated by the DATACOUNT.

When a DTIMEOUT flag occurs during the suspend procedure, this must be ignored.

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
To resume receiving data from the card, the following steps must be taken by firmware:
1.

The remaining number of data bytes (multiple of data blocks) must be programmed in
DATALENGTH.

2.

The DPSM must be configured to receive data in the DTDIR bit.

3.

The resume command must be sent from the CPSM, with the CMDTRANS bit set and
the CMDSUSPEND bit set, which ends the interrupt period when data transfer is
resumed (response bit DF = 1) and enabled the DPSM, after which the card resumes
sending data.

While sending data to the card, the SDMMC can suspend the write operation after the write
data block CRC status end (DPSM in Busy). Before sending the suspend command to the
card the following steps must be taken by firmware:
1.

Enable DHOLD flag (and DBCKEND flag when IDMAEN = 0)

2.

The DPSM must be prevented from start sending a new data block by setting
DTHOLD.

3.

When IDMAEN = 0: When receiving the DBCKEND flag the data transfer is stopped.
Firmware can stop filling the FIFO, after which the FIFO must be reset with FIFORST.
Any bytes still in the FIFO need to be rewritten when resuming the operation.

4.

When receiving the DHOLD flag the data transfer is stopped. The remaining number of
data bytes still to be written when resuming must be determined from the remaining
number of bytes indicated by the DATACOUNT.

5.

To suspend the card the suspend command must be sent by the CPSM with the
CMDSUSPEND bit set. This makes possible to start the interrupt period after the
suspend command response when the bus is suspended (response bit BS = 0).

To resume sending data to the card, the following steps must be taken by firmware:
1.

The remaining number of data bytes must be programmed in DATALENGTH.

2.

The DPSM must be configured for transmission with DTDIR set and enabled by having
the CPSM send the resume command with the CMDTRANS bit set and the
CMDSUSPEND bit set. This ends the interrupt period and start the data transfer. The
DPSM either goes to the Wait_S state when SDMMC_D0 does not signal busy, or goes
to the Busy state when busy is signaled.

3.

When IDMAEN = 1: The IDMA needs to be reprogrammed for the remaining bytes to
be transfered.

4.

When IDMAEN = 0: Firmware must start filling the FIFO with the remaining data.

SD I/O Read Wait
There are two methods to pause the data transfer during the block gap:
1.

Stopping the SDMMC_CK.

2.

Using Read Wait signaling on SDMMC_D2.

The SDMMC can perform a Read Wait with register settings according Table 292.
Depending the SDMMC operation mode (DS, HS, SDR12, SDR25) or (SDR50, SDR104,
DDR) each method has a different characteristic.
The timing for pause read operation by stopping the SDMMC_CK for DS, HS, SDR12, and
SDR25, the SDMMC_CK may be stopped 2 SDMMC_CK cycles after the end bit. When
ready the host resumes by restarting clock (see Figure 207).

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 207. Clock stop with SDMMC_CK for DS, HS, SDR12, SDR25
2 CK

1 CK

2 CK

SDMMC_CK

SDMMC_Dn

Read data

Interrupt period

H

Read data
MSv40193V2

The timing for pause read operation by stopping the SDMMC_CK for SDR50, SDR104, and
DDR50, the SDMMC_CK may be stopped minimum 2 SDMMC_CK cycles and maximum 5
SDMMC_CK cycles, after the end bit. When ready the host resumes by restarting clock, see
Figure 208. (In DDR50 mode the SDMMC_CK must only be stopped after the falling edge,
when the clock line is low.)
Figure 208. Clock stop with SDMMC_CK for DDR50, SDR50, SDR104
NAC 8 CK min.
tOP
0

1

tOP
2

3

4

5

6

7

5

6

7

8

9

10

SDMMC_CK

SDMMC_Dn

Read
data

1

0

End

Start

Read
data

2 CK min.
5 CK max.
MSv40194V2

In Read Wait SDMMC_CK clock stopping, when RWSTART is set, the DSPM stops the
clock after the end bit of the current received data block CRC. The clock start again after
writing 1 to the RWSTOP bit, where after the DPSM waits for a start bit from the card.
As SDMMC_CK is stopped, no command can be issued to the card. During a Read Wait
interval, the SDMMC can still detect SDIO interrupts on SDMMC_D1.
The optional Read Wait signaling on SDMMC_D2 (RW) operation is defined only for the SD
1-bit and 4-bit modes. The Read Wait operation enables the host to signal a card that is
reading multiple registers (IO_RW_EXTENDED, CMD53) to temporarily stall the data
transfer while allowing the host to send commands to any function within the SD I/O device.
To determine when a card supports the Read Wait protocol, the host must test capability bits
in the internal card registers.
The timing for Read Wait with a SDMMC_CK less then 50MHz (DS, HS, SDR12, SDR25) is
based on the interrupt period generated by the card on SDMMC_D1. The host by asserting
SDMMC_D2 low during the interrupt period requests the card to enter Read Wait. To exit
Read Wait the host must raise SDMMC_D2 high during one SDMMC_CK cycles before
making it Hi-Z, see Figure 209.

<!-- pagebreak -->

