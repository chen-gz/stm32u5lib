RM0456 Rev 6

RM0456

29.4.3

Octo-SPI I/O manager (OCTOSPIM)

OCTOSPIM matrix
The OCTOSPI I/O manager matrix allows the user to set a fully programmable premapping
of functions:
•

Any OCTOSPIM_Pn_CLK / OCTOSPIM_Pn_NCLK pair can be mapped independently
to OCTOSPI1_CLK/OCTOSPI1_NCLK or OCTOSPI2_CLK/OCTOSPI2_NCLK

•

Any OCTOSPIM_Pn_DQS can be mapped independently to OCTOSPI1_DQS or
OCTOSPI2_DQS

•

Any OCTOSPIM_Pn_NCS can be mapped independently to OCTOSPI1_NCS or
OCTOSPI2_NCS

•

Any OCTOSPIM_Pn_IO[3:0] and OCTOSPIM_Pn_IO[7:4] can be mapped
independently to OCTOSPI1_IO[3:0], OCTOSPI1_IO[7:4], OCTOSPI2_IO1[3:0] or
OCTOSPI2_IO[7:4]

For each OCTOSPI I/O manager port, individual signal enables and mapping are configured
through the corresponding OCTOSPI I/O manager Port n configuration register
(OCTOSPIM_PnCR).
When several I/O pins have the same configuration and are enabled at the same time, the
result can be unpredictable.
In the default out-of-reset configuration, the OCTOSPI1 and OCTOSPI2 signals are
mapped, respectively, on Port 1 and on Port 2.
The OCTOSPIM configuration can be changed only when all OCTOSPIs are disabled.

29.4.4

OCTOSPIM multiplexed mode
When this mode is set, the OCTOSPIs are time-multiplexed over the same bus. They get
the ownership of the bus (in turn) through a request/acknowledge protocol with REQ/ACK
signals.
The time-multiplexing is enabled by setting the MUXEN bit of the configuration register
OCTOSPIM_CR.
The fairness counter (MAXTRAN) of each OCTOSPI can be used to manage accurately the
maximum duration for which a given OCTOSPI takes the bus: this feature ensures a
maximum bus access latency for the other OCTOSPI(s). When the bus is released by one
OCTOSPI, an arbitration phase occurs, which is round-robin: when another OCTOSPI
requests the bus, it gets it.
When the multiplexed mode is enabled, either the fairness counter or the refresh timeout
counter of both OCTOSPI interfaces must be activated.
OCTOSPIn_NCS are not part of the multiplexing.Only OCTOSPIn_IOs, OCTOSPIn_DQS
and OCTOSPIn_CLK / OCTOSPIn_NCLK are multiplexed.
When the multiplexed mode is used, only clock mode 0 is supported on the OCTOSPIs.
Due to arbitration and bus sharing, the auto polling interval time of the OCTOSPI, when
used, may increase.

Minimum switching duration
The minimum number of cycles needed to switch from an OCTOSPI to another can be
configured.

RM0456 Rev 6

<!-- pagebreak -->

1111

Octo-SPI I/O manager (OCTOSPIM)

RM0456

This internal timer guarantees a latency between the falling edge of the REQ signal of the
active OCTOSPI (the active one releases the bus), and the rising edge of the ACK signal to
the requesting OCTOSPI (the bus is granted to the requesting one).
The REQ2ACK_TIME field of the configuration register OCTOSPIM_CR defines the
duration.

Pin mapping in multiplexed mode
In multiplexed mode, the mapping of the bus is done as described below:

<!-- pagebreak -->

