2952

Serial peripheral interface (SPI)

RM0456

Figure 850. Master and three independent slaves connected in star topology
(1)

SS
Rx shift register
Tx shift register
SPI clock
generator
Master

MISO

MISO

MOSI

MOSI

SCK

SCK

IO1

SS

Tx shift register
Rx shift register

Slave 1

IO2
IO3

MISO
MOSI

Tx shift register
Rx shift register

SCK
SS
Slave 2

MISO
MOSI

Tx shift register
Rx shift register

SCK
SS
Slave 3
MSv50508V1

1. The master single SS pin hardware output functionality cannot support this topology (to be replaced by a
set of GPIOs under software control). The SS pin is free for other application uses (such as GPIO or other
alternate functions). Refer to Section 68.4.7 for details.
2. If the application cannot ensure that no more than a single SS active signal is provided by the master at a
given time, it is better to configure the MISO pins in an open-drain configuration with an external pull-up on
the MISO line to prevent conflicts between the interconnected outputs of the slaves. Else, a push-pull
configuration can be applied without an extra resistor (see I/O alternate function input/output (GPIO)
section).
3. The RDY signals can be read by the master from the slaves optionally.

68.4.6

Multimaster communication
Unless the SPI bus is not designed primarily for a multimaster capability, the user can use a
built-in feature that detects a potential conflict between two nodes trying to master the bus at
the same time. For this detection, the SS pin is configured in hardware input mode. The
connection of more than two SPI nodes working in this mode is impossible, as only one
node can apply its output on a common data line at a given time.
When the nodes are not active, both stay in slave mode by default. Once a node wants to
overtake control on the bus, it switches itself into master mode and applies active level on

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
the slave select input of the other node via the dedicated GPIO pin. After the session is
complete, the active slave select signal is released and the node mastering the bus
temporarily returns back to passive slave mode waiting for the next session to start.
If both nodes raise their mastering request at the same time, a bus conflict event appears
(see mode fault MODF event). The user can apply some simple arbitration process (for
example postpone the next attempt by different predefined timeouts applied to both nodes).
Figure 851. Multimaster application

Rx (Tx) shift register
Tx (Rx) shift register
SPI clock
generator
Master
(Slave)

MISO

MISO

MOSI

MOSI

SCK

SCK

GPIO

SS(1)

SS(1)

GPIO

Rx (Tx) shift register
Tx (Rx) shift register
SPI clock
generator

Master
(Slave)

MSv50502V1

1. The SS pin is configured at hardware input mode at both nodes. Its active level enables the MISO line
output control as the passive node is configured as a slave.
2. The RDY signal is not used in this communication.

68.4.7

Slave select (SS) pin management
In slave mode, the SS works as a standard ‘chip select’ input and lets the slave
communicate with the master. In master mode, the SS can be used either as an output or an
input. As an input it can prevent a multimaster bus collision, and as an output it can drive a
slave select signal of a single slave. The SS signal can be managed internally (software
management of the SS input) or externally when both the SS input and output are
associated with the SS pin (hardware SS management). The user can configure which level
of this input/output external signal (present on the SS pin) is considered as active by the
SSIOP bit setting. SS level is considered as active if it is equal to SSIOP.
The hardware or software slave select management can be set using the SSM bit in the
SPI_CFG2 register:
•

Software SS management (SSM = 1): in this configuration, slave select information is
driven internally by the SSI bit value in the SPI_CR1 register. The external SS pin is
free for other application uses (such as GPIO or other alternate functions).

•

Hardware SS management (SSM = 0): in this case, there are two possible
configurations. The configuration used depends on the SS output configuration (SSOE
bit in the SPI_CFG2 register).
–

SS output enable (SSOE = 1): this configuration is only used when the MCU is
set as master. The SS pin is managed by the hardware. The functionality is tied to
CSTART and EOT control. As a consequence, the master must apply the proper
TSIZE > 0 setting to control the SS output correctly. Even if SPI AF is not applied
at the SS pin (it can be used as a standard GPIO then), keep anyway SSOE = 1 to
ensure the default SS input level and prevent any mode fault evaluation at the

RM0456 Rev 6

<!-- pagebreak -->

2952

Serial peripheral interface (SPI)

RM0456

input of the master SS internal logic applicable at a multimaster topology
exclusively.

Note:

a)

When SSOM = 0 and SP = 000, the SS signal is driven to the active level as soon
as the master transfer starts (CSTART = 1) and it is kept active until its EOT flag is
set or the transmission is suspended.

b)

When SP = 001, a pulse is generated as defined by the TI mode.

c)

When SSOM = 1, SP = 000 and MIDI > 1 the SS is pulsed inactive between data
frames, and kept inactive for a number of SPI clock periods defined by the MIDI
value decremented by one (from 1 to 14).

d)

SS input is forced to nonactive state internally at master to prevent any mode fault.

–

SS output disable (SSM = 0, SSOE = 0):

a)

If the microcontroller is acting as the master on the bus, this configuration allows
multimaster capability. If the SS pin is pulled into an active level in this mode, the
SPI enters master mode fault state and the SPI device is automatically
reconfigured in slave mode (MASTER = 0).

b)

In slave mode, the SS pin works as a standard ‘chip select’ input and the slave is
selected while the SS line is at its active level.

The purpose of automatic switching into slave mode when a mode fault occurs is to avoid
the possible conflicts on data and clock lines. As the SPE is automatically reset in this
condition, both Rx and Tx FIFOs are flushed and current data is lost.
When the SPI slave is enabled in the hardware SS management mode, all the transfers are
ignored even in case of the SS is found at active level. They are ignored until the slave
detects a start of the SS signal (transition from nonactive to active level) just synchronizing
the slave with the master. This is because the hardware management mode cannot be used
when the external SS pin is fixed. There is no such protection in the SS software
management. Then the SSI bit must be changed when there is no traffic on the bus and the
SCK signal is in idle state level between transfers exclusively in this case.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial peripheral interface (SPI)
Figure 852. Scheme of SS control logic
SSIOP control
SSI control
SSM control
Master
mode

Slave
mode

SS(SSI) <> SSIOP

OK

Non active

SS(SSI) = SSIOP

Conflict

Active

SS_IN

1
SS
pin

SS_IN

0
GPIO
logic

SSOM, MIDI
MSSI control
SS
output
control

SS_OUT
Master mode & HW SS
management (SSM=0)
only

SS
output
logic

SSOE control
SS external
logic

SS internal logic
MSv40470V2

When the hardware output SS control is applied (SSM = 0, SSOE = 1), by configuration of
the MIDI[3:0] and MSSI[3:0] bitfields, the user can control the timing of the SS signal
between data frames and can insert an extra delay at the beginning of every transfer (to
separate the SS and clock starts). This can be useful when the slave needs to slow down
the flow to obtain sufficient room for correct data handling (see Figure 853).
Figure 853. Data flow timing control (SSOE = 1, SSOM = 0, SSM = 0)
SS
SCK
MOSI
MISO

MSB

MSSI[3:0]

LSB

tSCK

DSIZE[4:0] + 1

MSB

MIDI[3:0]
DSIZE[4:0] + 1
MSv40472V1

1. MSSI[3:0] = 0011, MIDI[3:0] = 0011 (SCK flow is continuous when MIDI[3:0] = 0).
2. CPHA = 0, CPOL = 0, SSIOP = 0, LSBFRST = 0.

Additionally, setting the SSOM bit invokes a specific mode, which interleaves pulses
between data frames if there is a sufficient space to provide them (MIDI[3:0] must be set
greater than one SPI period). Some configuration examples are shown in Figure 854.

RM0456 Rev 6

<!-- pagebreak -->

