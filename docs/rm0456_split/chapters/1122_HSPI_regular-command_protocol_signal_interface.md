1177

Hexadeca-SPI interface (HSPI)

RM0456

The DQS management can be enabled by setting DQSE of HSPI_CCR.
Figure 170. DTR read in octal-SPI mode with DQS (Macronix mode) example
§

NCS

DQS
IO[7:0]

EEh

11h

A[31:24] A[23:16]

A[15:8]

A[7:0]

§

§

§

CLK

Address

D1

Dummy

D0

Word
unit

D3

D2

Word
unit
MSv43489V1

30.4.5

HSPI regular-command protocol signal interface
Single-SPI mode
The legacy SPI mode allows just a single bit to be sent/received serially. In this mode, the
data is sent to the external device over the SO signal (Single-SPI Output) (whose I/Os are
shared with IO0). The data received from the external device arrives via SI (Single-SPI
Input) (whose I/Os are shared with IO1).
Compared to the SPI legacy mode, I0/SO and I1/SI are respectively equivalent to MOSI and
MISO, having the HSPI generating the clock.
The different phases can each be configured separately to use this single-bit mode by
setting to 001 the IMODE, ADMODE, ABMODE, and DMODE fields in HSPI_CCR and
HSPI_WCCR.
In each phase configured in single-SPI mode:
•

IO0 (SO) is in output mode.

•

IO1 (SI) is in input mode (high impedance).

•

IO2 is in output mode and forced to 0 (to deactivate the “write protect” function).

•

IO3 is in output mode and forced to 1 (to deactivate the “hold” function).

•

IO4 to IO15 are in output mode and forced to 0.

This is the case even for the dummy phase if DMODE[2:0] = 001.

Dual-SPI mode
In dual-SPI mode, two bits are sent/received simultaneously over the IO0/IO1 signals.
The different phases can each be configured separately to use dual-SPI mode by setting
to 010 the IMODE, ADMODE, ABMODE, and DMODE fields in HSPI_CCR and
HSPI_WCCR.
In each phase configured in dual-SPI mode:

<!-- pagebreak -->

•

IO0/IO1 are at high-impedance (input) during the data phase for the read operations,
and outputs in all other cases.

•

IO2 is in output mode and forced to 0.

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
•

IO3 is in output mode and forced to 1.

•

IO4 to IO15 are in output mode and forced to 0.

In the dummy phase when DMODE[2:0] = 010, IO0/IO1 are always high-impedance.

Quad-SPI mode
In quad-SPI mode, four bits are sent/received simultaneously over the IO0/IO1/IO2/IO3
signals.
The different phases can each be configured separately to use the quad-SPI mode by
setting to 011 the IMODE, ADMODE, ABMODE, and DMODE fields in HSPI_CCR and
HSPI_WCCR.
In each phase configured in quad-SPI mode:
•

IO0 to IO3 are all at high-impedance (inputs) during the data phase for the read
operations, and outputs in all other cases.

•

IO4 to IO15 are in output mode and forced to 0.

In the dummy phase when DMODE[2:0] = 011, IO0 to IO3 are all high-impedance.

Octal-SPI mode
In regular octal-SPI mode, the eight bits are sent/received simultaneously over the IO[0:7]
signals.
The different phases can each be configured separately to use the octal-SPI mode by
setting to 100 the IMODE, ADMODE, ABMODE, and DMODE fields in HSPI_CCR and
HSPI_WCCR.
In each phase that is configured in octal-SPI mode, IO[0:7] are all at high-impedance (input)
during the data phase for read operations, and outputs in all other cases.
In the dummy phase when DMODE[2:0] = 100, IO[0:7] are all high-impedance.

HSPI mode
In HSPI mode, the 16 bits are sent/received simultaneously over the IO[0:15] signals during
the data phase.
The following phases must be configured separately to use the HSPI mode:
1.

Set to 100 the IMODE, ADMODE, and ABMODE fields (in HSPI_CCR and
HSPI_WCCR).

2.

Set to 101 the DMODE fields (in HSPI_CCR and HSPI_WCCR).

In each phase that is configured in HSPI mode, IO[0:15] are all at high-impedance (input)
during the data phase for read operations, and outputs in all other cases.
In the dummy phase when DMODE[2:0] = 101, IO[0:15] are all high-impedance.
IO[8:15] are used only in HSPI mode. If none of the phases are configured to use this mode,
then the pins corresponding to IO[8:15] can be used for other functions, even while the
HSPI is active.

Single-data rate (SDR) mode
By default, all the phases operate in single-data rate (SDR) mode.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

In SDR mode, when the HSPI drives the IO0/SO and IO1 to IO15 signals, these signals
transition only with the falling edge of CLK.
When receiving data in SDR mode, the HSPI assumes that the external devices also send
the data using CLK falling edge. By default (when SSHIFT = 0 in HSPI_TCR), the signals
are sampled using the following (rising) edge of CLK.
Figure 171. SDR write command in octal-SPI mode example

§

NCS

02h

FDh

A[31:24]

A[23:16]

A[15:8]

A[7:0]

D0

D1

§§

IO[7:0]

§

CLK

D254

D255
MSv43490V1

Double-transfer rate (DTR) mode
Each of the instruction, address, alternate-byte, and data phases can be configured to
operate in DTR mode setting IDTR, ADDTR, ABDTR, and DDTR in HSPI_CCR.
In memory-mapped mode, the DTR mode for each phase of the write operations is specified
in HSPI_WCCR. The DTR mode for each phase of the read operations is specified in
HSPI_CCR.
In DTR mode, when the HSPI drives the IO0/SO and IO1to IO7 signals in the instruction,
address, and alternate-byte phases, a bit is sent or received on each of the falling and rising
edges of CLK.
In DTR mode, when the HSPI drives the IO0 to IO15 signals in the data phases, a bit is sent
or received on each of the falling and rising edges of CLK.
When receiving data in DTR mode, the HSPI assumes that the external devices also send
the data using both CLK rising and falling edges. When DDTR = 1 in HSPI_CCR,
the software must clear SSHIFT in HSPI_TCR. Thus, the signals are sampled one half of a
CLK cycle later (on the following, opposite edge).
Figure 172. DTR write in octal-SPI mode (Macronix mode) example

CLK
IO[7:0]

02h

FDh A[31:24] A[23:16] A[15:8] A[7:0]

D1

D0

Word Unit

у у у

у

NCS

D255

D254

Word Unit
MSv43491V1

Dual-quad configuration
When DMM = 1 in HSPI_CR, the HSPI is in dual-memory configuration: if DMODE = 011,
two external quad-SPI devices (device A and device B) are used in order to send/receive
eight bits (or 16 bits in DTR mode) every cycle, effectively doubling the throughput.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
Each device (A or B) uses the same CLK and NCS signals, but each has separate IO0 to
IO3 signals.
The dual-quad configuration can be used in conjunction with the single-SPI, dual-SPI, and
quad-SPI modes, as well as with either SDR or DTR mode.
The device size, as specified in DEVSIZE[4:0] of HSPI_DCR1, must reflect the total external
device capacity that is the double of the size of one individual component.
If address X is even, then the byte that the HSPI gives for address X is the byte at the
address X/2 of device A, and the byte that the HSPI gives for address X + 1 is the byte at
the address X/2 of device B. In other words, the bytes at even addresses are all stored in
device A and the bytes at odd addresses are all stored in device B.
When reading the status registers of the devices in dual-quad configuration, twice as many
bytes must be read compared to the same read in regular-command protocol: if each device
gives eight valid bits after the instruction for fetching the status register, then the HSPI must
be configured with a data length of 2 bytes (16 bits), and the HSPI receives one byte from
each device.
If each device gives a status of 16 bits, then the HSPI must be configured to read 4 bytes to
get all the status bits of both devices in dual-quad configuration. The least-significant byte of
the result (in the data register) is the least-significant byte of device A status register.
The next byte is the least-significant byte of device B status register. Then, the third byte of
the data register is the device A second byte. The forth byte is the device B second byte
(if devices have 16-bit status registers).
An even number of bytes must always be accessed in dual-quad configuration. For this
reason, bit 0 of DL[31:0] in HSPI_DLR is stuck at 1 when DMM = 1.
In dual-quad configuration, the behavior of device A interface signals is basically the same
as in normal mode. Device B interface signals have exactly the same waveforms as
device A ones during the instruction, address, alternate-byte, and dummy-cycle phases. In
other words, each device always receives the same instruction and the same address.
Then, during the data phase, the AIOx and the BIOx buses both transfer data in parallel, but
the data that is sent to (or received from) device A is distinct than the one from device B.

Dual-octal configuration
When DMM = 1 in HSPI_CR, the HSPI is in dual-memory configuration: when
DMODE = 100, two external octal-SPI devices (device A and device B) are used in order to
receive 32 bits in DTR mode every cycle, effectively doubling the throughput as well as the
capacity.
Each device (A or B) uses the same CLK and NCS signals, but each has separate IO0 to
IO7 signals.
The dual-octal configuration can be used in DTR mode exclusively in conjunction with the
single-SPI, dual-SPI, quad-SPI and octal-SPI modes.
The device size, as specified in DEVSIZE[4:0] of HSPI_DCR1, must reflect the total external
device capacity that is the double of the size of one individual component.
If address X is even, then the byte that the HSPI gives for address X is the byte at the
address X/2 of device A, and the byte that the HSPI gives for address X + 1 is the byte at the
address X/2 of device B. In other words, the bytes at even addresses are all stored in device
A and the bytes at odd addresses are all stored in device B.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

When reading the status registers of the devices in dual-octal DTR mode, twice as many
bytes must be read compared to the same read in regular DTR mode: if each device gives
twice eight valid bits after the instruction for fetching the status register, then the HSPI must
be configured with a data length of 4 bytes. The LSB is the LSB of device A, and the third
byte is the LSB of device B.
If each device gives a status of 16 bits, then the HSPI must be configured to read 4 bytes to
get all the status bits of both devices in dual-octal DTR mode. In such case, the order of
retrieved status bits is as follows:
•

first byte: LSB of device A

•

second byte: second byte of device A

•

third byte: LSB of device B

•

fourth byte: second byte of device B

In indirect mode using DTR mode, a number of bytes multiple of four must always be
accessed in HSPI mode. For this reason, bit 0 and bit 1 of the DL[31:0] bitfield in HSPI_DLR
are stuck at 0 when DMODE[2:0] = 101.
In dual-octal configuration, the behavior of device A interface signals is basically the same
as in normal mode. Device B interface signals have exactly the same waveforms as
device A ones during the instruction, address, alternate-byte, and dummy-cycles phases. In
other words, each device always receives the same instruction and the same address.
Then, during the data phase, the AIOx and the BIOx buses both transfer data in parallel, but
the data that is sent to (or received from) device A is distinct than the one from device B.
Note:

The variable latency is not supported in dual-octal configuration.

HSPI mode
When DMODE[2:0] = 0b101 in HSPI_CCR, the HSPI is in single 16-bit-memory
configuration: when DMODE[2:0] = 0b101 with DMM value ignored in that case. A single
external HSPI device is used in order to send/receive 16 bits (or 32 bits in DTR mode) every
cycle, effectively doubling the throughput.
The device provides/receives two separate DQS signals: DQS0 for the eight LSBs and
DQS1 for the eight MSBs.
The HSPI mode can be used in conjunction with the single-SPI, dual-SPI, quad-SPI and
octal-SPI modes, as well as with either the SDR or the DTR mode.
The device size, as specified in DEVSIZE[4:0] of HSPI_DCR1, must reflect the total external
device capacity.
In SDR mode, a number of bytes multiple of two must always be accessed in HSPI mode.
For this reason, bit 0 of DL[31:0] in HSPI_DLR is stuck at 0 when DMODE = 101.
In DTR mode, a number of bytes multiple of four must always be accessed in HSPI mode.
For this reason, bit 0 and 1 of DL[31:0] in HSPI_DLR are stuck at 0 when DMODE = 101.

<!-- pagebreak -->

