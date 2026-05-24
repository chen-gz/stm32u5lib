RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
1.

Set the SDMMC_CK frequency in accordance with the achievable SDMMC_CMD data
rate in Open-drain mode, CLKDIV must be set >1, and SETCLKRX must select the
sdmmc_io_in_ck.

2.

Load CMD40 (GO_IRQ_STATE) in the command registers.

3.

Enable wait for interrupt by setting WAITINT register bit.

4.

Configure the CPSM to send a command immediately.
–

5.

This causes the CMD40 to be sent and the CPSM to be halted in the Wait state,
waiting for a interrupt service request response.

To exit the wait for interrupt state (CPSM Wait state):
–

Upon the detection of an interrupt service request response start bit the CPSM
moves to the Receive state where the response is received. The complete
reception of the response is indicated by the CMDREND or the command CRC
error flags.

–

To abort the interrupt mode the host clears the WAITINT register bit, which causes
the host to send an interrupt service request response by itself. This moves the
CPSM to the Receive state. The complete reception of the response is indicated
by the CMDREND or the command CRC error flags.

Note:

On a simultaneous send interrupt service request response start bit collision the host loses
the bus access after the transmission bit.

31.6.5

Boot operation
In boot operation mode the host can read boot data from the card by either one of the two
boot operation functions:
•

Normal boot. (keeping CMD line low)

•

Alternative boot (sending CMD0 with argument 0xFFFFFFFA)

The boot data can be read according the following configuration options, depending on card
register settings:
•

The partition from which boot data is read (EXT_CSD Byte[179])

•

The boot data size (EXT_CSD Byte[226])

•

The bus configuration during boot (EXT_CSD Byte[177])

•

Receiving boot acknowledgment from the card. (EXT_CSD Byte[179])

If boot acknowledgment is enabled the card send pattern 010 on SDMMC_D0 within 50ms
after boot mode has been requested by either CMD line going low or after CMD0 with
argument 0xFFFFFFFA. A boot acknowledgment timeout (ACKTIMEOUT) and
acknowledgment status (ACKFAIL) is provided.

Normal boot operation
If the SDMMC_CMD line is held low for at least 74 clock cycles after card power-up or reset,
before the first command is issued, the card recognizes that boot mode is being initiated.
Within 1 second after the CMD line goes low, the card starts to sent the first boot code data
on the SDMMC_Dn line(s). The host must keep the SDMMC_CMD line low until after all
boot data has been read. The host can terminate boot mode by pulling the SDMMC_CMD
line high.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 213. Normal boot mode operation
SDMMC_CK

SDMMC_CMD

CMD1

SDMMC_Dn

S 010 E

S

Block read
+ CRC E

S

RESP

Block read
+ CRC E

50 ms max.

56 cycles min.

1 s max.
Boot completed
74 cycles
MSv40944V2

To perform the normal boot procedure the following steps needed:
1.

Reset the card.

2.

if a boot acknowledgment is requested enable the BOOTACKEN and set the ACKTIME
and enable the ACKFAIL and ACKTIMEOUT interrupt.

3.

enable the data reception by setting the DPSM in receive mode (DTDIR) and the
number of data bytes to be received in DATALENGTH.

4.

Enable the DTIMEOUT, DATAEND, and CMDSENT interrupts for end of boot
command confirmation.

5.

Select the normal boot operation mode in BOOTMODE, and enable boot in BOOTEN.
The boot procedure is started by enabling the CPSM with CPSMEN.This causes:

6.

7.

8.

–

the SDMMC_CMD to be driven low. (BOOTMODE = normal boot).

–

the ACK timeout to start.

–

DPSM to be enabled.

The incorrect reception of the boot acknowledgment can be detected with ACKFAIL
flag or ACKTIMEOUT flag when enabled.
–

when an incorrect boot acknowledgment is received the ACKFAIL flag occurs.

–

when the boot acknowledgment is not received in time the ACKTIMEOUT flag
occurs.

when all boot data has been received the DATAEND flag occurs.
–

when data CRC fails the DCRCFAIL flag is also generated.

–

when the data timeout occurs the DTIMEOUT flag is also generated.

When last data has been received, read data from the FIFO until FIFO is empty after
which end of data DATAEND flag is generated.
–

9.

SDMMC has completely received all data and the DPSM is disabled.

The boot procedure is terminated by firmware clearing BOOTEN, which causes the
SDMMC_CMD line to go high. The CMDSENT flag is generated 56 cycles later to
indicate that a new command can be sent.
–

If the boot procedure is aborted by firmware before all data has been received the
CPSM Abort signal stops data reception and disables the DPSM which triggers an
DABORT flag when enabled.

10. The CMDSENT flag signals the end of the boot procedure and the card is ready to
receive a new command.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)

Alternative boot operation
After card power-up or reset, if the host send CMD0 with the argument 0xFFFFFFFA after
74 clock cycles before CMD0 is issued, the card recognizes that boot mode is being
initiated. Within 1 second after the CMD0 with argument 0xFFFFFFFA has been sent, the
card starts to send the first boot code data on the SDMMC_Dn line(s). The master
terminates boot operation by sending CMD0 (Reset).
Figure 214. Alternative boot mode operation
SDMMC_CK

SDMMC_CMD

CMD0
boot

SDMMC_Dn

CMD0
reset

S 010 E

S

Block read
E
+ CRC

S

RESP

Block read
E
+ CRC

50 ms max.
1 s max.

CMD1

56 cycles min.
Boot completed

74 cycles
MSv40945V2

To perform the alternative boot procedure the following steps needed:
1.

Move the SDMMC to power-off state, and reset the card.

2.

Move the SDMMC to power-on state. This guarantees the 74 SCDMMC_CK cycles to
be clocked before any command.

3.

if a boot acknowledgment is requested enable the BOOTACKEN and set the ACKTIME
and enable the ACKTIMEOUT flag.

4.

enable the data reception by setting the DPSM in receive mode (DTDIR) and the
number of data to be received in DATALENGTH. Enable the DTIMEOUT and
DATAEND flags.

5.

Select the alternative boot operation mode in BOOTMODE, load the CMD0 with the
0xFFFFFFFA argument in the command registers. Enable CMDSENT flag for end of
boot command confirmation, and enable boot in BOOTEN. The boot procedure is
started by enabling the CPSM with CPSMEN. This causes:
–

the loaded command and argument to be sent out. (BOOTMODE = alternative
boot).

–

the ACK timeout to start.

–

DPSM to be enabled.

6.

When the command has been sent the CMDSENT flag is generated, at which time the
BOOTEN bit must be cleared.

7.

the reception of the boot acknowledgment can be detected with ACKFAIL flag when
enabled.
–

8.

when the boot acknowledgment is not received in time the ACKTIMEOUT flag
occurs.

when all boot data has been received the DATAEND flag occurs.
–

when data CRC fails the DCRCFAIL flag is also generated.

–

when the data timeout occurs the DTIMEOUT flag is also generated.

RM0456 Rev 6

<!-- pagebreak -->

