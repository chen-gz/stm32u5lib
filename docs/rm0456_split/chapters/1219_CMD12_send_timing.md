RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
Figure 209. Read Wait with SDMMC_CK < 50 MHz
SDMMC_CK
2 CK

SDMMC_D1

2 CK

Read data

Read data
2 CK

SDMMC_D2

Read data

SDMMC_D3
SDMMC_D0

Read data

Read Wait

Read data

Int period

Read data

SDMMC_CMD

CMD
MSv40941V2

For SDR50, SDR104 with a SDMMC_CK more than 50MHz, and DDR50, the card treats
the Read Wait request on SDMMC_D2 as an asynchronous event. The host by asserting
SDMMC_D2 low after minimum 2 SDMMC_CK cycles and maximum 5 SDMMC_CK cycles,
request the card to enter Read Wait. To exit Read Wait the host must raise SDMMC_D2
high during one SDMMC_CK cycles before making it Hi-Z. The host must raise SDMMC_D2
on the SDMMC_CK clock (see Figure 210).
Figure 210. Read Wait with SDMMC_CK > 50 MHz
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

8

8

8

8

9

10

SDMMC_CK

1

SDMMC_D2

Read data End

SDMMC_D3
SDMMC_D1
SDMMC_D0

Read data End

Read Wait

1

0
Read data
Start

0
Read data
Start

2 CK min.
5 CK max.
MSv40948V2

In Read Wait SDMMC_D2 signaling, when RWSTART is set, the DPSM drives SDMMC_D2
after the end bit of the current received data block CRC. The Read Wait signaling on
SDMMC_D2 is removed when writing 1 to the RWSTOP bit. The DPSM remains in R_W
state for two more SDMMC_CK clock cycles to drive SDMMC_D2 to 1 for one clock cycle
(in accordance with SDIO specification), where after the DPSM waits for a start bit from the
card.
During the Read Wait signaling on SDMMC_D2 commands can be issued to the card.
During the Read Wait interval, the SDMMC can detect SDIO interrupts on SDMMC_D1.

31.6.2

CMD12 send timing
CMD12 is used to stop/abort the data transfer, the card data transmission is terminated two
clock cycles after the end bit of the Stop Transmission command.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Table 294. CMD12 use cases
Data operation

Stop Transmission command CMD12 Description

SDMMC stream write

The data transfer is stopped/aborted by sending the Stop Transmission
command.

SDMMC open ended
multiple block write

The data transfer is stopped/aborted by sending the Stop Transmission
command.
If the card detects an error, the host must abort the operation by sending
the Stop Transmission command.

The Stop Transmission command is not required at the end of this type of
multiple block write. (sending the Stop Transmission command after the
SDMMC block write with
card has received the last block is regarded as an illegal command.)
predefined block count
If the card detects an error, the host must abort the operation by sending
the Stop Transmission command.
SDMMC stream read

The data transfer is stopped/aborted by sending the Stop Transmission
command.

SDMMC open ended
multiple block read

The data transfer is stopped/aborted by sending the Stop Transmission
command.
If the card detects an error, the host must abort the operation by sending
the Stop Transmission command.

The Stop Transmission command is not required at the end of this type of
multiple block read. (sending the Stop Transmission command after the
SDMMC block read with card has transmitted the last block is regarded as an illegal command.)
predefined block count Transaction can be aborted by sending the Stop Transmission command.
If the card detects an error, the host must abort the operation by sending
the Stop Transmission command.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
All data write and read commands can be aborted any time by a Stop Transmission
command CMD12. The following data abort procedure applies during an ongoing data
transfer:
1.

Load CMD12 Stop Transmission command in registers and set the CMDSTOP bit.
–

2.

3.

4.

5.

This causes the CPSM Abort signal to be generated when the command is sent to
the DPSM.

Configure the CPSM to send a command immediately (clear WAITPEND bit).
–

The card, when sending data, stops data transfer 2 cycles after the Stop
Transmission command end bit.
The card when no data is being sent, does not start sending any new data.

–

The host, when sending data, sends one last data bit followed by an end bit after
the Stop Transmission command end bit.
The host when not sending data, does not start sending any new data.

When IDMAEN = 0, the FIFO need to be reset with FIFORST.
–

When writing data to the card. On the CMDREND flag, firmware must stop writing
data to the FIFO. Subsequently the FIFO must be reset with FIFORST, this flushes
the FIFO.

–

When reading data from the card. On the CMDREND flag, firmware must read the
remaining data from the FIFO. Subsequently the FIFO must be reset with
FIFORST.

When IDMAEN = 1, hardware takes care of the FIFO.
–

When writing data to the card. On the CPSM Abort signal, hardware stops the
IDMA and subsequently the FIFO is flushed.

–

When reading data from the card. On the CPSM Abort signal, hardware instructs
the IDMA to transfer the remaining data from the FIFO to RAM.

When the FIFO is empty/reset the DABORT flag is generated.

Stream operation and CMD12
To stop the stream transfer after the last byte to be transfered, the CMD12 end bit timing
must be sent aligned with the data stream end of last byte. The following write stream data
procedure applies:
1.

Initialize the stream data in the DPSM, DTMODE = MCC stream data transfer.

2.

Send the WRITE_DATA_STREAM command from the CPSM with CMDTRANS = 1.

3.

Preload CMD12 in command registers, with the CMDSTOP bit set.

4.

Configure the CPSM to send a command only after a wait pending (WAITPEND = 1)
end of last data (according DATALENGTH).

5.

Enabling the CPSM to send the STOP_TRANSMISSION command, the stream data
end bit and command end bit are aligned.

6.

–

When DATALENGTH > 5 bytes, Command CMD12 is waited in the CPSM to be
aligned with the data transfer end bit.

–

When DATALENGHT < 5 bytes, Command CMD12 is started before and the
DPSM remains in the Wait_S state to align the data transfer end with the CMD12
end bit.

The write stream data can be aborted any time by clearing the WAITPEND bit. This
causes the Preloaded CMD12 to be sent immediately and stop the write data stream.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 211. CMD12 stream timing
SDMMC_CK

CMD12

SDMMC_CMD

SDMMC_D0

E

Stream data last byte

E
NST
MSv40942V1

To stop the read stream transfer after the last byte, the CMD12 end bit timing must occur
after the last data stream byte. The following read stream data procedure applies:
1.

Wait for all data to be received by the DPSM and read from the FIFO (DATAEND flag).
–

2.

Send CMD12 by the CPSM.
–

Note:

The DPSM does not receive more data than indicated by DATALENGTH, even if
the card is sending more data.
CMD12 stops the card sending data.

The SDMMC does not receive any more data from the card when DATACOUNT = 0, even
when the card continues sending data.

Block operation and CMD12
To stop block transfer at the end of the data, the CMD12 end bit must be sent after the last
block end bit.
When writing data to the card the CMD12 end bit must be sent after the write data block
CRC token end bit. This requires the CMD12 sending to be tied to the data block
transmission timing. To stop an Open-ended Multiple block write, the following procedure
applies:
1.

Before starting the data transfer, set DTMODE to “block data transfer ending with
STOP_TRANSMISSION command”.

2.

Wait for all data to be sent by the DPSM and the CRC token to be received, (DATAEND
flag).
–

3.

–

<!-- pagebreak -->

