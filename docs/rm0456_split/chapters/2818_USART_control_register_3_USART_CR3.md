RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
content of one received data packet. Once one data packet is uploaded and one packet is
downloaded, the application software or the DMA checks again the DXP flag and repeats
the data packet read and write operation sequence until DXP is read as zero.
The drawback of services based on DXP exclusively is that servicing the TxFIFO is delayed
on purpose due to the SPI nature since the TXP events precedes the RXP ones. To allow
continuous SCK clock flow on the master side and prevent underrun on the slave side, it is
recommended to prefill a few data ahead to the TxFIFO at the transfer start, and wait until
the transfer is complete to read the last received data.

Hardware data counter
If a hardware data counter is used (TSIZE bitfield of the SPI_CR2 register set to a nonzero
value), the application software does not need to calculate the remaining number of data to
be handled. The end of transfer automatically controls the CRC, as well as the hardware SS
management, when used. The user application does not need to ensure that the overall
number of data is a multiple of the packet size. If the last data packet is incomplete, it is
serviced in the same way as any full packet. The unused part of this last packet is not
handled by the peripheral. Only the valid data are written into the TxFIFO and/or read from
the RxFIFO, and the redundant writes and reads are discarded. When the hardware counter
reaches zero, the EOT flag is raised, and the RXP flag is not set for the last data packet. If
the last packet is not aligned with the packet size, the TXP and EOT occupancy events are
not related to the configured packet size but to the number of remaining data calculated by
hardware.
If TSIZE is kept at zero (for example due to an unpredictable number of data), only the
number of transferred data corresponding to the FIFO thresholds is supported. If some data
are not aligned with the configured packet size, they may remain pending and available in
the RxFIFO. In this case, the FTHLV level is not reached and the RXP flag is not set. Then
the number of remaining received data frames in the RxFIFO is indicated by the RXWNE
and RXPLVL bitfields of the SPI_SR register. Nevertheless, the application software can still
read the complete data packet from the RxFIFO and the redundant data are read as zero.
To prevent such an unaligned data reception, the user must configure the FIFO threshold to
a single data (FTHLV = 0). In this case, each data frame is serviced by its own RXP
occupancy event.

Data transfer handling
Data are transferred using MOSI and MISO lines, depending on the SPI communication
mode and associated configuration. The mode is configured through the COMM[1:0] bitfield
of the SPI_CFG2 register. The HDDIR bit of the SPI_CR1 register controls the data flow
direction in half-duplex mode. The communication flow is handled by the master via the SS
and SCK signals. The slave selected for the communication is fully subordinated to the
master communication activity, no matter if it handles the data flow on time or not. The slave
can only temporarily suspend the master communication by using the RDY signal. The
active levels of RDY and SS signals can be changed, or MISO and MOSI functionality
swapped (via the RDIOP, SSIOP and IOSWP bits of the SPI_CFG2 register).
The SPI master transmitter can operate in full-duplex, simplex, or half-duplex mode. In fullduplex mode, data are received synchronously. The master starts the data transfer once the
CSTART bit of the SPI_CR1 register is set, provided the SPI is enabled and the TxFIFO

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

content is not empty. The master then provides the serial clock signal continuously on the
SCK pin until:
•

The total number of required data programmed in TSIZE is transferred, or

•

The transfer is suspended.

An automatic temporary suspension of the master transmission occurs when:
•

The slave does not assert the RDY signal, if it is used, or

•

The TxFIFO becomes empty, or

•

In full-duplex mode, the RxFIFO becomes full while the automatic suspension is
enabled (MASRX set in the SPI_CR1 register).

When an automatic suspension occurs, the master stops providing the clock, and the
transfer proceeds depending on the cause of the suspension, when:
•

The slave asserts RDY.

•

The master software or the DMA writes additional data to TxFIFO.

•

In full-duplex mode, the master software or the DMA releases the RxFIFO to enable it
to accommodate new data.

The SPI master can receive data only when it operates in simplex receiver or half-duplex
receiver mode. In these modes, the master starts the data transfer when the SPI is enabled,
and the transfer is released by setting the CSTART bit. The serial clock signal is then
provided continuously on the SCK pin by the master until:
•

The total number of required data programmed in TSIZE is received, or

•

The transfer is suspended by the master.

An automatic temporary suspension of the reception occurs when:
•

The slave does not assert the RDY signal, if it is used, or

•

The RxFIFO becomes full while the automatic suspension is enabled (MASRX bit set in
the SPI_CR1 register).

When an automatic suspension occurs, the master stops providing the clock and the
transfer proceeds depending on the cause of the suspension, when:
•

The slave asserts RDY.

•

The master software or the DMA releases the RxFIFO to enable it in order to
accommodate new data.

A preferable way to terminate a transfer is to program the TSIZE bitfield to generate an EOT
event. Hardware SS signal or CRC handshake can then be controlled. To restart the internal
state machine properly, it is recommended to disable the SPI and enable it again when the
transfer is complete, even if the SPI configuration is not changed.
A transfer can be suspended at any time by setting the CSUSP bit of the SPI_CR1 register,
which clears the CSTART bit. This software suspension control ensures the completion of
any ongoing data frame. To restart the internal state machine properly, the SPI must be
disabled and enabled again before the next transfer starts.
Oppositely, a temporary automatic suspension controlled by hardware typically results in a
frozen and incomplete data frame transfer. This depends on baud rate setting, but usually,
due to internal synchronization delays, the SCK signal stops when a few bits from the next
data frame are already transferred on the bus. Once the suspension is released, the data
frame transmission completes by transferring the remaining bits. That is why this automatic
suspension is not quite reliable when the data frame size is less than eight bits. When
shorter data frames are used, to prevent any data loss and assure proper RDY and/or

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
MASRX operation, the user can interleave and so extend the transfer time by inserting
additional dummy clock cycles so that the period for a single data frame is higher or equal to
eight SCK duration. This is done by setting the MIDI[3:0] bitfield of the SPI_CFG2 register.

Caution:

If the SPE bit is cleared in master mode while the transfer is ongoing without any
suspension, the clock is stopped even if the current frame transmission is not complete, and
the content of the FIFOs is flushed and lost.

68.4.13

Disabling the SPI
To disable the SPI, it is mandatory to follow the disable procedures described in this
paragraph.
In the master mode, it is important to do this before the system enters a low-power mode
when the peripheral clock is stopped, otherwise, ongoing transfers may be corrupted.
In slave mode, the SPI communication can continue when the spi_pclk and spi_ker_ck
clocks are stopped, without interruption, until any end of communication or data service
request condition is reached. The spi_pclk can generally be stopped by setting the system
into Stop mode. Refer to the RCC section for further information.
The master in full-duplex or transmit-only mode can finish any transfers when it stops
providing data for transmission. In this case, the clock stops after the last data transfer. TXC
flag can be polled (or interrupt enabled with EOTIE = 1) in order to wait for the last data
frame to be sent.
When the master is in any receive-only mode, to stop the peripheral, the SPI communication
must first be suspended, by setting the CSUSP bit.
The data received but not read remain stored in RxFIFO when the SPI is suspended.
After such a software suspension, SPI must always be disabled to restart the internal state
machine properly.
When SPI is disabled, RxFIFO is flushed. To prevent losing unread data, the user must
ensure that RxFIFO is empty when disabling the SPI, by reading all remaining data (as
indicated by the RXP, RXWNE, and RXPLVL fields in the SPI_SR register).
The standard disable procedure is based on polling EOT and/or TXC status to check if a
transmission session is (fully) completed. This check can be done in specific cases, too,
when it is necessary to identify the end of ongoing transfers, for example:
•

When the master handles the SS signal by a GPIO not related to SPI (for example at
case of multislave star topology) and it has to provide a proper end-of-SS pulse for the
slave, or

•

When transfers from DMA or FIFO are completed while the last data frame or CRC
frame transfer is still ongoing in the peripheral bus.

When TSIZE > 0, EOT and TXC signals are equal so polling of EOT is reliable at whatever
SPI communication mode to check the end of the bus activity. When TSIZE = 0, the user
has to check TXC, SUSP, or FIFO occupancy flags according to the applied SPI mode and
the way of the data flow termination.
The correct disable procedure in master mode, except when receive-only mode is used, is:
1.

Wait until TXC = 1 and/or EOT = 1 (no more data to transmit and last data frame sent).
When CRC is used, it is sent automatically after the last data in the block is processed.

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

TXC/EOT is set when the CRC frame is complete in this case. When a transmission is
suspended the software has to wait until the CSTART bit is cleared.
2.

Read all RxFIFO data (until RXWNE = 0 and RXPLVL = 00).

3.

Disable the SPI (SPE = 0).

The correct disable procedure for master receive-only modes is:
1.

Wait on EOT or break the receive flow by suspending SPI (CSUSP = 1).

2.

Wait until SUSP = 1 (the last data frame is processed) if the receive flow is suspended.

3.

Read all RxFIFO data (until RXWNE = 0 and RXPLVL = 00).

4.

Disable the SPI (SPE = 0).

In slave mode, any ongoing data are lost when disabling the SPI.

Controlling the I/Os
As soon as the SPI is disabled, the associated and enabled AF outputs can still be driven by
the device depending on the AFCNTR bit of the SPI_CFG2 register. When active output
control is applied (AFCNTR = 1) and SPI has just been disabled (SPE = 0), the enabled
outputs associated with SPI control signals (like SS and SCK at master and RDY at slave)
can immediately toggle to inactive level (according to SSIOP and CPOL settings at master
and RDIOP at slave respectively). Instead, the data line output (MOSI at master and MISO
at slave) can immediately change its level depending on the actual TxFIFO content, with the
effect of potentially making invalid and no more guaranteed the value of the latest
transferred bit on the bus. If necessary, the user has to take care about proper data hold
time at the data line and avoid any eventual fast SPI disable just after the last data transfer
is complete.
Note:

Despite stability of the latest bit is guaranteed by design during the sampling edge of the
clock, some devices can require even extension of this data bit stability interval during the
sampling. It can be done, for example by inserting a small software delay between EOT
event occurrence and SPI disable action.

68.4.14

Communication using DMA (direct memory addressing)
To operate at its maximum speed and to facilitate the data register read/write process
required to avoid overrun, the SPI features a DMA capability, which implements a simple
request/acknowledge protocol.
A DMA access is requested when the TXDMAEN or RXDMAEN enable bits of the
SPI_CFG1 register are set. Separate requests must be issued to the Tx and Rx buffers to
fulfill the service of the defined packet.
•

In transmission, a series of DMA requests is triggered each time TXP is set. The DMA
then performs a series of writes to the SPI_TXDR register.

•

In reception, a series of DMA requests is triggered each time RXP is set. The DMA
then performs a series of reads from the SPI_RXDR register. When EOT is set at the
end of the transfer and the last data packet is incomplete, then DMA request is
activated automatically according to RXWNE and RXPLVL[1:0] setting to read the rest
of data.

If the SPI is programmed in receive-only mode, UDR is never set.
If the SPI is programmed in a transmit mode, TXP and UDR can be eventually set at slave
side, because transmit data may not be available. In this case, some data are sent on the
TX line according with the UDR management selection.

<!-- pagebreak -->

