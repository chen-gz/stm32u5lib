RM0456 Rev 6

RM0456

69.4.3

Serial audio interface (SAI)

Main SAI modes
Each audio subblock of the SAI can be configured to be master or slave via the MODE bits
in the SAI_xCR1 register of the selected audio block.

Master mode
In master mode, the SAI delivers the timing signals to the external connected device:
•

The bit clock and the frame synchronization are output on pin SCK_x and FS_x,
respectively.

•

If needed, the SAI can also generate a master clock on the MCLK_x pin.

Both SCK_x, FS_x and MCLK_x are configured as outputs.

Slave mode
The SAI expects to receive timing signals from an external device.
•

If the SAI subblock is configured in asynchronous mode, then the SCK_x and FS_x
pins are configured as inputs.

•

If the SAI subblock is configured to operate synchronously with another SAI interface or
with the second audio subblock, the corresponding SCK_x and FS_x pins are left free
to be used as general purpose I/Os.

In slave mode, the MCLK_x pin is not used and can be assigned to another function.
It is recommended to enable the slave device before enabling the master.

Configuring and enabling SAI modes
Each audio block can use a different audio protocol. When PRTCFG[1:0] of the SAI_xCR1
register is set to 0, the free protocol mode is selected and each SAI subblock can emulate
I2S standards, LSB- or MSB-justified, PCM/DSP, TDM, or AC’97 protocols.
Each audio subblock can be independently defined as a transmitter or receiver through the
MODE bit in the SAI_xCR1 register of the corresponding audio block. As a result, the
SAI_SD_x pin is respectively configured as an output or an input.
Two master audio blocks in the same SAI can be configured with two different MCLK and
SCK clock frequencies. In this case, they have to be configured in asynchronous mode.
Each of the audio blocks in the SAI is enabled by the SAIEN bit in the SAI_xCR1 register. As
soon as this bit is active, the transmitter or the receiver is sensitive to the activity on the
clock, data, and synchronization lines in slave mode.
In master Tx mode, enabling the audio block immediately generates the bit clock for the
external slaves even if there is no data in the FIFO. However, FS signal generation is
conditioned by the presence of data in the FIFO. After the FIFO receives the first data to
transmit, this data is output to external slaves. If there is no data to transmit in the FIFO, 0
values are then sent in the audio frame with an underrun flag generation.
In slave mode, the audio frame starts when the audio block is enabled and when a start of
frame is detected.
In Slave Tx mode, no underrun event is possible on the first frame after the audio block is
enabled, because the mandatory operating sequence in this case is:

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

69.4.4

RM0456

1.

Write into the SAI_xDR (by software or by DMA).

2.

Wait until the FIFO threshold (FLH) flag is different from 0b000 (FIFO empty).

3.

Enable the audio block in slave transmitter mode.

SAI synchronization mode
There are two levels of synchronization, either at audio subblock level or at SAI level.

Internal synchronization
An audio subblock can be configured to operate synchronously with the second audio
subblock in the same SAI. In this case, the bit clock and the frame synchronization signals
are shared to reduce the number of external pins used for the communication. The audio
block configured in synchronous mode sees its own SCK_x, FS_x, and MCLK_x pins
released back as GPIOs while the audio block configured in asynchronous mode is the one
for which FS_x and SCK_x ad MCLK_x I/O pins are relevant (if the audio block is
considered as master).
Typically, the audio block in synchronous mode can be used to configure the SAI in full
duplex mode. One of the two audio blocks can be configured as a master and the other as
slave, or both as slaves with one asynchronous block (corresponding SYNCEN[1:0] bits set
to 00 in SAI_xCR1) and one synchronous block (corresponding SYNCEN[1:0] bits set to 01
in the SAI_xCR1 register).
Note:

Due to internal resynchronization stages, PCLK APB frequency must be higher than twice
the bit rate clock frequency.

External synchronization
The audio subblocks can also be configured to operate synchronously with another SAI.
This can be done as follows:

Note:

1.

The SAI, which is configured as the source from which the other SAI is synchronized,
has to define which of its audio subblocks is supposed to provide the FS and SCK
signals to other SAI. This is done by programming SYNCOUT[1:0] bits.

2.

The SAI which receives the synchronization signals, has to select which SAI provides
the synchronization by setting the proper value on SYNCIN[1:0] bits. For each of the
two SAI audio subblocks, the user must then specify if it operates synchronously with
the other SAI via the SYNCEN bit.

The SYNCIN[1:0] and SYNCOUT[1:0] bits are located in the SAI_GCR register, and the
SYNCEN bits in the SAI_xCR1 register.
If both audio subblocks in a given SAI need to be synchronized with another SAI, it is
possible to choose one of the following configurations:
•

Configure each audio block to be synchronous with another SAI block through the
SYNCEN[1:0] bits.

•

Configure one audio block to be synchronous with another SAI through the
SYNCEN[1:0] bits. The other audio block is then configured as synchronous with the
second SAI audio block through SYNCEN[1:0] bits.

The following table shows how to select the proper synchronization signal depending on the
SAI block used. For example SAI2 can select the synchronization from SAI1 by setting SAI2
SYNCIN to 0. If SAI1 wants to select the synchronization coming from SAI2, SAI1 SYNCIN
must be set to 1. Positions noted as ‘Reserved’ must not be used.

<!-- pagebreak -->

