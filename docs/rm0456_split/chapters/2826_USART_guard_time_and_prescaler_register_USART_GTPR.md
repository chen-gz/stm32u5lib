RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
Figure 858. Optional configurations of the slave behavior when an underrun
condition is detected
UDRCFG=0
SCK

MOSI

DI1

DI2

DI3

MISO

DO1

Dummy

[UDRDR]

TxFIFO
occupancy

1

0
UDR propagation latency

UDR

UDRCFG=1
SCK

MOSI

DI1

DI2

DI3

MISO

DO1

DI1

DI2

TxFIFO
occupancy
UDR

1

0
UDR propagation latency
MSv63460V1

Note:

The hardware propagation of an UDR event needs additional traffic on the bus. It always
takes a few extra SPI clock cycles after the event happens (both underrun captured by
hardware and cleared by software). If clearing of the UDR flag by software is applied close
to the end of the data frame transfer or when the SCK line is at idle in between the frames,
the next extra underrun pattern is sent initially by the slave before the valid data from
TxFIFO becomes transferred again. The user can prevent this by SPI disable/enable action
between sessions to restart the underrun logic and so initiate the next session by the valid
data.

Mode fault (MODF)
Mode fault occurs when the master device has its internal SS signal (SS pin in SS hardware
mode, or SSI bit in SS software mode) pulled low. This automatically affects the SPI
interface in the following ways:
•

The MODF bit is set and the SPI interrupt is triggered if the MODFIE bit is set.

•

The SPE bit is forced to zero until the MODF bit is set. This disables the SPI and blocks
all the peripheral outputs except the MODF interrupt request if enabled.

•

The MASTER bit is cleared, thus forcing the device into slave mode.

MODF is cleared by writing 1 to the MODFC bit of the SPI_IFCR register.

RM0456 Rev 6

<!-- pagebreak -->

