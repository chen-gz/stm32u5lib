RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
When the SPI is used at a simplex mode, the user must enable the adequate DMA channel
only while keeping the complementary unused channel disabled.
If the SPI is programmed in transmit-only mode, RXP and OVR are never set.
If the SPI is programmed in full-duplex mode, RXP and OVR are eventually set, because
received data are not read.
In transmission mode, when the DMA or the user has written all the data to be transmitted
(the TXTF flag is set in the SPI_SR register), the EOT (or TXC at case TSIZE = 0) flag can
be monitored to ensure that the SPI communication is complete. This is required to avoid
corrupting the last transmission before disabling the SPI or before disabling the spi_pclk in
master mode. The software must first wait until EOT = 1 and/or TXC = 1.
When starting communication using DMA, to prevent DMA channel management raising
error events, these steps must be followed in order:
1.

Enable the DMA Rx buffer in the RXDMAEN bit of the SPI_CFG1 register, if DMA Rx is
used.

2.

Enable DMA requests for Tx and Rx in DMA registers, if the DMA is used.

3.

Enable the DMA Tx buffer in the TXDMAEN bit of the SPI_CFG1 register, if DMA Tx is
used.

4.

Enable the SPI by setting the SPE bit.

To close communication, it is mandatory to follow these steps in order:
1.

Disable DMA requests for Tx and Rx in the DMA registers, if the DMA issued.

2.

Disable the SPI by following the SPI disable procedure.

3.

Disable DMA Tx and Rx buffers by clearing the TXDMAEN and RXDMAEN bits of the
SPI_CFG1 register, if DMA Tx and/or DMA Rx are used.

Data packing with DMA
If the transfers are managed by DMA (TXDMAEN and RXDMAEN set in the SPI_CFG1
register) the packing mode is enabled/disabled automatically depending on the PSIZE value
configured for SPI TX and the SPI RX DMA channel.
The packing mode is enabled if the DMA channel PSIZE value is a multiple of the data size.
Then the DMA automatically manages the sequences of write and read operations to/from
the SPI data registers, based on FIFO occupancy flags, and depending on the FIFO
threshold and data size configurations.
The DMA completes the transfer automatically according to the TSIZE field setting,
whatever the data packing mode used, and even if the number of data to transfer is not a
multiple of the DMA data size (16 bits or 32 bits) while the frame size is smaller.
Alternatively, the last data frames can be written by software, in the single/unpacked mode.
Configuring any DMA data access to less than the configured data size is forbidden. One
complete data frame must be always accessed at minimum.

68.4.15

Autonomous mode
The SPI is capable of handling and initializing transfers autonomously. It requires no specific
system execution interaction until the ongoing transfer ends.
Such autonomous transfers can be handled not only in Run or Sleep mode but even in Stop
mode when the SPI logic is able to provide temporary clock requests addressed to the reset

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

and clock controller (RCC) to ensure clocking of those SPI domains just necessary for
handling the data flow between the memory and the peripheral interface at dependency in
the SPI mode.
In Stop mode, the APB clock is requested by the peripheral each time the SPI registers
need to be updated based on specific traffic events (mainly TXP and RXP). The required
clock is provided by RCC if SPI autonomous mode is enabled at the RCC configuration and
the SPI is clocked by an internal oscillator available in Stop mode.
Interrupts or DMA requests are then generated, depending on the SPI configuration. If no
interrupt is enabled, the device remains in Stop mode. If DMA requests are enabled, the
data are directly transferred to/from the SRAM thanks to the DMA while the device remains
in Stop mode. If an enabled interrupt occurs, the device wakes up from Stop mode.
Note:

The peripheral clock request stays pending until the flag with enabled interrupt is set. This is
why it is important to service these pending requests and clear their flag as soon as possible
at system sensitive to the low-power consumption especially, and the application must
acknowledge all pending interrupts events before switching the SPI to low-power mode.

Slave mode
When the SPI is configured as a standard slave and the device is at Stop mode, the SPI
kernel clock and the SPI APB clock are not provided permanently. All the data flow between
the SPI interface and associated FIFOs is handled by an external SCK clock provided by
the outer master device within the serial interface clock domain. APB clock temporal
requests are then based upon specific traffic events at dependency on the SPI
configuration. As the slave never initializes a transfer, there is no need to synchronize any
transfer start in this mode.
Note:

The peripheral clock request stays pending until the flag with enabled interrupt stays set.
This is why it is important to service these pending requests and clear their flag as soon as
possible to achieve the lowest power consumption, and the application must acknowledge
all pending interrupt events before switching the SPI to low-power mode.

Master mode
The SPI operating in master mode provides the SCK signal for outer slaves until the transfer
is complete. The SCK signal is derived from the SPI clock generator running within the
kernel clock domain fed from the RCC upon kernel clock request provided by the SPI when
the device is in Stop mode. Temporal requests for the APB clock are then based upon
specific traffic events, depending on the SPI configuration. The SPI master always initializes
a transfer.
To minimize consumption in Stop mode, it is suggested to combine communication starts
triggered by hardware (the TRIGEN bit is set in the SPI_AUTOCR register) and transfers of
predefined data size (TSIZE > 0 in the SPI_CR2 register). This ensures that any APB clock
request is suppressed between EOT handling and the next trigger event.
A transfer starts once the CSTART bit is set. In the case of master transmitter, the TxFIFO
must be filled by data, too. The CSTART bit can be written either by software or by hardware
when a synchronous trigger is detected at Run, Sleep, or Stop mode. The trigger source is
selected by the TRIGSEL bits and enabled by the TRIGEN bit of the SPI_AUTOCR register.
When the enabled trigger is detected, the transfer starts and continues by handling data
until the EOT event or the transfer suspension. When the TRIGEN bit is changed, the user
must prevent any trigger event occurrence. If the user cannot prevent that, the TRIGEN bit

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
must be written while the SPI is disabled otherwise the peripheral behavior is not
guaranteed.

68.5

SPI specific modes and control

68.5.1

TI mode
With a specific SP[2:0] bitfield setting of the SPI_CFG2 register, the SPI can be configured
compliant with the TI protocol. The SCK and SS signals polarity, phase and flow, as well as
the bit order are fixed, so the setting of CPOL, CPHA, LSBFRST, SSOM, SSOE, SSIOP,
SSM, RDIOP, RDIOM, MSSI, and MIDI is not required when the SPI is in TI mode
configuration. The SS signal synchronizes the protocol by pulses over the LSB data bit as it
is shown in Figure 857.
Figure 857. TI mode transfer
SS

SCK

MOSI
MISO

X

X

MSB

LSB

MSB

LSB

MSB

LSB

MSB

LSB

DSIZE[4:0] + 1

DSIZE[4:0] + 1
TRELEASE

MSv40475V1

In slave mode, the clock generator is used to define the time when the slave output at MISO
pin becomes high-Z when the current transfer finishes. The master baud rate setting
(MBR[2:0] of SPI_CFG1) is applied and any baud rate can be used to determine this
moment with optimal flexibility. The delay for the MISO signal to become high-Z (TRELEASE)
depends on internal resynchronization, too, which takes the next additional 2-4 periods of
the clock signal feeding the generator. It is given by the following formula:

Tbaud
Tbaud
----------------- + 2 x Tspi_ker_ck ≤ Trelease ≤ ----------------- + 4 x Tspi_ker_ck
2
2
If the slave detects a misplaced SS pulse during a data transfer the TIFRE flag is set.

RM0456 Rev 6

<!-- pagebreak -->

