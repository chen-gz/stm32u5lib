RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)

Data frame format
The SPI shift register can be set up to shift out MSB-first or LSB-first, depending on the
value of the LSBFRST bit of the SPI_CFG2 register.
The data frame size is configured through the DSIZE bitfield of the SPI_CFG1 register to a
range that depends on the SPI instance (see Section 68.3: SPI implementation). The setting
applies to both transmission and reception. The data bits in the SPI_TXDR/SPI_RXDR
registers are right-aligned with 8, 16, or 32 bits, depending on the data size (see
Figure 856). These registers can consequently be accessed by bytes, half-words, or words.
FIFO accesses smaller than a single data are forbidden. When the FIFO occupancy flag is
raised, a FIFO access greater than the FIFO threshold can lead to spurious data being read
or written data being lost. When the access to/from the SPI_TXDR/SPI_RXDR registers is a
multiple of the configured data size, data packing is applied automatically, while the lowest
significant bits/bytes are communicated first. For more details, see Section 68.4.12: SPI
data transmission and reception procedures.
Figure 856. Data alignment when data size is not equal to 8, 16 or 32 bits
'6,=(ELWV
GDWDLVULJKWDOLJQHGRQE\WH
([DPSOH: DSIZE[4:0]=00011

7

Tx

4

3

0

XXXX

7

Rx

ELWV'6,=(ELWV
GDWDLVULJKWDOLJQHGRQKDOIZRUG
([DPSOH: DSIZE[4:0]=01101

15 14 13

0

XX

4

3

0

0000

ELWV'6,=(ELWV
GDWDLVULJKWDOLJQHGRQZRUG
([DPSOH: DSIZE[4:0]=11010
31

27

26

0

26

0

XXXXX

15 13 12
00

0

31

27

00000

MSv40473V1

68.4.10

Configuring the SPI
The configuration procedure is almost the same for the master and the slave. For specific
mode setups, follow the dedicated chapters. When a standard communication must be
initialized, perform these steps:
1.

Write the proper GPIO registers: configure GPIO alternate functions at MOSI, MISO,
SCK, SS, and RDY pins if applied.

2.

Write into the SPI_CFG1 and SPI_CFG2 registers and set up the proper values of all
‘not reserved’ bits and bitfields, prior to enabling the SPI, with the following exceptions:
a)

The SSOM, MASRX, SSOE, RDIOM, MBR[2:0], BPASS, MIDI[3:0], MSSI[3:0] bits
are taken into account in master mode only, the MSSI[3:0] bits take effect when
the SSOE bit is set, the RDIOP bit takes no effect when the RDIOM bit is not set in
master mode. When the slave is configured in TI mode, the MBR[2:0] setting is
also considered.

b)

UDRCFG is taken into account in slave mode only.

c)

CRCSIZE[4:0] bitfield must be configured if CRCEN is set.

d)

CPOL, CPHA, LSBFRST, SSOM, SSOE, SSIOP, SSM, RDIOP, RDIOM, MSSI,
and MIDI are not required in TI mode.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)
e)

68.4.11

RM0456

Once the AFCNTR bit is set in the SPI_CFG2 register, all the SPI outputs start to
be propagated onto the associated GPIO pins regardless of the peripheral enable.
So, any later configuration changes of the SPI_CFG1 and SPI_CFG2 registers
can affect the level of signals on these pins.

3.

Write to the SPI_CR2 register to select the length of the transfer, if it is not known
TSIZE must be programmed to zero.

4.

Write to the SPI_CRCPOLY and into the TCRCINI, RCRCINI, and CRC33_17 bits of
the SPI_CR1 register to configure the CRC polynomial and CRC calculation if needed.

5.

Configure DMA streams dedicated for the SPI Tx and Rx in DMA registers if the DMA
streams are used (see Section 68.4.14: Communication using DMA (direct memory
addressing)).

6.

Configure SSI, HDDIR, and MASRX in the SPI_CR1 register if required.

7.

Program the IOLOCK bit in the SPI_CR1 register if the configuration protection is
required (for safety).

Enabling the SPI
It is recommended to configure and enable the SPI slave before the master sends the clock.
But there is no impact if the configuration and enabling procedure is done while traffic is
ongoing on the bus, assuming that the SS signal is managed by hardware at slave or kept
inactive by the slave software when the software management of the SS signal is applied
(see Section 68.4.7). To prevent any risk of any data underrun, all the data to be sent have
to be written to the slave transmitter data register before the master starts its clocking. The
SCK signal must be settled to the idle state level corresponding to the selected polarity,
before the SPI slave is selected by SS, else the following transfer may be desynchronized.
When the SPI slave is enabled at the hardware SS management mode, all the transfers are
ignored even in case of the SS is found at active level. They are ignored until the slave
detects a start of the SS signal (its transition from nonactive to active level) just
synchronizing the slave with the master. That is why the hardware management mode
cannot be used when the external SS pin is fixed. There is no such protection at the SS
software management. In this case, the SSI bit must be changed when there is no traffic on
the bus and the SCK signal is at idle state level between transfers exclusively in this case.
The master in full duplex (or in any transmit-only mode) starts to communicate when the SPI
is enabled, the CSTART bit is set, and the TxFIFO is not empty, or with the next write to
TxFIFO.
In any master receive-only mode, the master starts to communicate and the clock starts
running after the SPI is enabled and the CSTART bit is set.
For handling DMA, see Section 68.4.14.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

68.4.12

Serial peripheral interface (SPI)

SPI data transmission and reception procedures
The SPI can transfer data at a very high communication speed. Even though the data is
FIFO buffered, handling a continuous data flow by servicing data frames individually leads
to an enormous CPU or DMA load, in particular when the data size is small. This potentially
leads to a significant limitation of the overall system performance, as well as communication
errors such as data overrun or underrun that may be raised when the system latencies in
servicing requests become comparable to single data frame transfer time.
The SPI offers advanced hardware control features to prevent these issues from occurring:
•

Decreasing the number of events requiring service by collecting the data frames into
larger data packets
The number of data frames per packet (FIFO threshold) can be configured. The
complete data packet generates a FIFO occupancy event, which is then handled within
a single, compact, service.

•

Note:

Decreasing the number of CPU execution cycles required for the service, by applying
the appropriate data access:
–

The widest access to the SPI data registers must be applied. This allows
cumulative data to be handled by a single register read or write operation (data
packing).

–

The number of read/write accesses necessary to complete the data packet must
be aligned with the data frame size and the FIFO threshold.

•

Concurrent read and write services to handle full-duplex data flow based on
common FIFO occupancy events.

•

Embedded hardware data counters to define the exact number of data involved in
transfers.

Using these features is optional. Nothing prevents the application from handling the data
flow frame by frame.
The following sections give more details and describe specific cases for using these
advanced hardware control features to handle data transfers.

Data packet control
The data frame size (number of bits in the frame) is defined through the DSIZE bitfield of the
SPI_CFG1 register. The number of data frames per packet can be configured by selecting
the FIFO threshold through the FTHLV bitfield of the SPI_CFG1 register. If the threshold
value is set to zero, each completed data frame raises the FIFO occupancy event. The data
packet occupancy must not exceed half of the FIFO size, which depends on the SPI
instance of the product. The FIFO capacity (multiple of 8 bits) is consumed according to the
following rules, depending on the data frame size:
•

Data frames from 4 to 8 bits occupy one byte of the FIFO.

•

Data frames from 9 to 16 bits occupy two bytes of the FIFO.

•

Data frames from 17 to 24 bits occupy three bytes of the FIFO.

•

Data frames from 25 to 32 bits occupy four bytes of the FIFO.

For example, if the FIFO capacity is 16 bytes, it accommodates up to five 24-bit frames. In
this case, the data packet configuration must not exceed two data frames.
The SPI features two separate FIFOs to handle the reception and transmission data flow,
the RxFIFO, and the TxFIFO. Their content can be handled by monitoring the occupancy

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

flags RXP, TXP, and DXP of the SPI_SR register according to the SPI mode (duplex or
simplex):
•

If RXP is set, at least one complete data packet can be read from the RxFIFO.

•

If TXP is set, at least one complete data packet can be written to the TxFIFO.

•

If DXP is set, at least one complete data packet can be read from the RxFIFO and
written to the TxFIFO at full-duplex mode.

These flags can be polled by software, or they can trigger an interrupt and/or a DMA request
(if enabled through the RXPIE, TXPIE, and DXPIE bits of the SPI_EIR register or the
RXDMAEN and TXDMAEN bits of the SPI_CFG1 register).
Once an occupancy flag is set, the software or the DMA must read and/or write a complete
data packet before checking the flag again to verify if the next packet can be handled. This
cycle can be repeated until the corresponding flag is read at zero.
Both RxFIFO and TxFIFO contents are flushed and cannot be accessed when the SPI is
disabled (SPE cleared in the SPI_CR1 register).

Data packing versus data register access control
The content of the RxFIFO and TxFIFO can be accessed by reading/writing from/to the SPI
data registers SPI_RXDR and SPI_TXDR, respectively. A read access from the SPI_RXDR
register returns the oldest values stored in the RxFIFO that have not been read yet. A write
access to the SPI_TXDR register stores the data written at the end of the send queue in the
TxFIFO.
These data registers can be accessed by 8-, 16-, or 32-bit read and write CPU instructions,
forced by the register address casting applied by the software. Data packing is performed
automatically by hardware if the data access applied by the software is a multiple of the data
size. It allows handling more than one data in parallel in a single data register access. Then
the SPI operates using the lowest significant byte or half-word first. FIFO data accesses of
less than the configured data size are forbidden. To avoid spurious data being read or
written data being lost, a complete data packet (configured through the FIFO threshold)
must be serviced when the corresponding FIFO occupancy flag is set. If the data pattern is
not byte-, half-word, or word-aligned, only the valid data bits are stored right-aligned to the
FIFO and transferred on the bus. Unused bits are discarded on the transmitter side and
padded with zeros on the receiver side.
For example, if the data frame size fits into one byte, the data packing is used automatically
when a 16-bit or 32-bit read/write access is performed by software from/to the
SPI_RXDR/SPI_TXDR register. In this case, two respectively four data are handled by a
single 16-bit respectively 32-bit access. Additionally, if such data frames are grouped into
packets of four via the FIFO threshold setting, the packet to be serviced is signalized by
raising the corresponding FIFO occupancy flag (TXP, RXP, or DXP). If the instance FIFO
threshold is sufficient, a packet containing eight 8-bit data frames can be handled by two
consecutive 32-bit accesses upon the same single threshold event. The most efficient data
handling is achieved when the data packet size (defined by DSIZE and FTHLV bitfields) is
aligned with the read/write access from/to the data registers.

Concurrent read and write services
In full-duplex mode, both TxFIFO and RxFIFO packet occupancies can be monitored
through a common FIFO flag (DXP). When the DXP flag is set, the application performs the
specified number of writes to SPI_TXDR to upload the content of one entire data packet for
transmission, followed by the same number of reads from SPI_RXDR to download the

<!-- pagebreak -->

