2952

Serial peripheral interface (SPI)

RM0456

To avoid any multiple slave conflicts in a system comprising several MCUs, the SS pin must
be pulled to its nonactive level before reenabling the SPI, by setting the SPE bit.
As a security, the hardware does not allow the SPE bit to be set while the MODF bit is set. In
a slave device, the MODF bit cannot be set except as the result of a previous multimaster
conflict.
A correct software procedure when a master overtakes the bus in multimaster systems must
be the following one:
1.

Switch into master mode while SSOE = 0 (potential conflict can appear when another
master occupies the bus. In this case, MODF is raised, which prevents any next node
switching into master mode).

2.

Put GPIO pin dedicated for another master SS control into active level.

3.

Perform a data transfer.

4.

Put GPIO pin dedicated for another master SS control into nonactive level.

5.

Switch back to slave mode.

CRC error (CRCE)
This flag is used to verify the validity of the value received when the CRCEN bit of the
SPI_CFG1 register is set. The CRCE flag of the SPI_SR register is set if the value received
in the shift register does not match the receiver SPI_RXCRC value, after the last data is
received (as defined by TSIZE). The CRCE flag triggers an interrupt if the CRCEIE bit is set.
Clearing the bit CRCE is done by a writing 1 to the CRCEC bit of the SPI_IFCR register.

TI mode frame format error (TIFRE)
A TI mode frame format error is detected when an SS pulse occurs during an ongoing
communication when the SPI is operating in slave mode and configured to conform to the TI
mode protocol. When this error occurs, the TIFRE flag is set in the SPI_SR register. The SPI
is not disabled when an error occurs, the SS pulse is ignored, and the SPI waits for the next
SS pulse before starting a new transfer. The data may be corrupted since the error detection
may result in the loss of a few data frames.
The TIFRE flag is cleared by writing 1 to the TIFREC bit of the SPI_IFCR register. If the
TIFREIE bit is set, an interrupt is generated on the SS error detection. As data consistency
is no longer guaranteed, communication must be reinitiated by software between master
and slave.

<!-- pagebreak -->

