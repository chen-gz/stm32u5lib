•

NCS boundary

•

refresh

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
The NCS boundary feature limits a transaction to a boundary of aligned addresses. The size
of the address to be aligned with is configured in CSBOUND[4:0] of HSPI_DCR3 and it is
equal to 2CSBOUND.
As an example, if CSBOUND(4:0] = 0x4, the boundary is set to 24 = 16 bytes. As a
consequence, the NCS is released each time that the LSB address is equal to 0xF, and
each time that a new transaction is issued to address the next data.
If CSBOUND[4:0] = 0, the feature is disabled. A minimum value of three is recommended.
The NCS boundary feature cannot be used for flash memory devices in write mode since a
command is necessary to program another page of the flash memory.
The refresh feature limits the duration of the transactions to the value programmed in
REFRESH[31:0] of HSPI_DCR4. The duration is expressed in number of cycles. This allows
an external RAM to perform its internal refresh operation regularly.
The refresh value must be greater than the minimal transaction size in terms of number of
cycles including the command, address, alternate/dummy phases.
If NCS boundary and refresh are enabled at the same time, the NCS is released on the first
condition met.

Restarting after an interrupted transfer
When a read or write operation is interrupted by a timeout, the HSPI interface, as soon as
possible after getting back the port ownership, re-issues the initial command sequence
together with the address following the last address actually accessed before interruption.
The transfer initially set goes on and ends seamlessly.

30.4.8

HSPI operating modes introduction
The HSPI has the following operating modes regardless of the low-level protocol used
(either regular-command or HyperBus):

30.4.9

•

indirect mode (read or write)

•

automatic status-polling mode (only in regular-command protocol)

•

memory-mapped mode

HSPI indirect mode
In indirect mode, the commands are started by writing to the HSPI registers, and the data is
transferred by writing or reading the data register, in a similar way to other communication
peripherals.
When FMODE[1:0] = 00 in HSPI_CR, the HSPI is in indirect-write mode: bytes are sent to
the external device during the data phase. Data are provided by writing to HSPI_DR.
When FMODE[1:0] = 01, the HSPI is in indirect-read mode: bytes are received from the
external device during the data phase. Data are recovered by reading HSPI_DR.
In indirect mode, when the HSPI is configured in DTR mode over eight lanes with DQS
disabled, the given starting address and the data length must be even.

Note:

The HSPI_AR register must be updated even if the start address is the same as the start
address of the previous indirect access.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

The number of bytes to be read/written is specified in HSPI_DLR:
•

If DL[31:0] = 0xFFFF FFFF, the data length is considered undefined and the HSPI
simply continues to transfer data until it reaches the end of the external device (as
defined by DEVSIZE). If no bytes are to be transferred, DMODE[2:0] must be set to 0 in
HSPI_CCR.

•

If DL[31:0] = 0xFFFF FFFFF and DEVSIZE[4:0] = 0x1F (its maximum value indicating
at 4-Gbyte device), the transfers continue indefinitely, stopping only after an abort
request or after the HSPI is disabled. After the last memory address is read (at address
0xFFFF FFFF), reading continues with address = 0x0000 0000.

When the programmed number of bytes to be transmitted or received is reached, TCF bit is
set in HSPI_SR and an interrupt is generated if TCIE = 1 in HSPI_CR. In the case of an
undefined number of data, TCF is set when the limit of the external SPI memory is reached,
according to the device size defined in HSPI_DCR1.

Triggering the start of a transfer in regular-command protocol
Depending on the HSPI configuration, there are three different ways to trigger the start of a
transfer in indirect mode when using regular-command protocol. In general, the start of
transfer is triggered as soon as the software gives the last information that is necessary for
the command. More specifically in indirect mode, a transfer starts when one of the following
sequence of events occurs:
•

if no address is necessary (ADMODE[2:0] = 000) and if no data needs to be provided
by the software (FMODE[1:0] = 01 or DMODE[2:0] = 000), and at the moment when a
write is performed to INSTRUCTION[31:0] in HSPI_IR

•

if an address is necessary (when ADMODE[2:0] ≠ 000) and if no data needs to be
provided by the software (when FMODE[1:0] = 01 or DMODE[2:0] = 000), and at the
moment when a write is performed to ADDRESS[31:0] in HSPI_AR

•

if data needs to be provided by the software (when FMODE[1:0] = 00 and
DMODE[2:0] ≠ 000), and at the moment when a write is performed to DATA[31:0] in
HSPI_DR

A write to HSPI_ABR never triggers the communication start. If alternate bytes are required,
they must have been programmed before.
As soon as a command is started, the BUSY bit is automatically set in HSPI_SR.

Triggering the start of a transfer in HyperBus protocol
Depending on the HSPI configuration, there are different ways to trigger the start of a
command in indirect mode. In general, it is triggered as soon as the firmware gives the last
information that is necessary for the transfer to start, and more specifically, a communication
in indirect mode is triggered by one of the following register settings, when it is the last one
to be executed:
•

when a write is performed to ADDRESS[31:0] (HSPI_AR) with ADMODE[2:0] ≠ 000 in
indirect read mode (FMODE[1:0] = 01)

•

when a write is performed to DATA[31:0] (HSPI_DR) in indirect-write mode
(when FMODE = 00)

•

when a (dummy) write is performed to INSTRUCTION[31:0] (HSPI_IR) for indirect read
mode (with ADMODE[2:0] = 000 and FMODE = 01)

As soon as a transfer is started, the BUSY bit (HSPI_SR[5]) is automatically set.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)

FIFO and data management
Data in indirect mode pass through a 64-byte FIFO that is internal to the HSPI. FLEVEL in
HSPI_SR indicates how many bytes are currently being held in the FIFO.
In indirect-write mode (FMODE[1:0] = 00), the software adds data to the FIFO when it writes
in the HSPI_DR. A word write adds 4 bytes to the FIFO, a half-word write adds 2 bytes, and
a byte write adds only 1 byte. If the software adds too many bytes to the FIFO (more than
indicated in DL[31:0]), the extra bytes are flushed from the FIFO at the end of the write
operation (when TCF is set).
The byte/half-word accesses to the HSPI_DR must be done only to the least significant
byte/halfword of the 32-bit register.
FTHRES is used to define a FIFO threshold after which point the FIFO threshold flag, FTF,
gets set. In indirect-read mode, FTF is set when the number of valid bytes to be read from
the FIFO is above the threshold. FTF is also set if there is any data left in the FIFO after the
last byte is read from the external device, regardless of FTHRES setting. In indirect-write
mode, the FTF is set when the number of empty bytes in the FIFO is above the threshold.
If FTIE = 1, there is an interrupt when the FTF is set. If DMAEN = 1, a DMA transfer is
initiated when the FTF is set. The FTF is cleared by hardware as soon as the threshold
condition is no longer true (after enough data has been transferred by the CPU or DMA).
In indirect-read mode, when the FIFO becomes full, the HSPI temporarily stops reading
bytes from the external device to avoid an overrun.
The last data read in RX FIFO remains valid as long as there is no request for the next line.
This means that, when the application reads several times in a row at the same location, the
data is provided from the RX FIFO and not read again from the distant memory.

30.4.10

HSPI automatic status-polling mode
In automatic status-polling mode, the HSPI periodically starts a command to read a defined
number of status bytes (up to four). The received bytes can be masked to isolate some
status bits and an interrupt can be generated when the selected bits have a defined value.
The automatic status-polling mode must be used only in regular-command protocol. For
HyperBus protocol, it is not exploitable since the read status register into the HyperFlash
memory must be performed in two steps (a write operation followed by a read operation).
The access to the device begins in the same manner as in indirect-read mode. BUSY in
HSPI_SR goes high at this point, and stays high even between the periodic accesses.
The content of MASK[31:0] in HSPI_PSMAR is used to mask the data from the external
device in automatic status-polling mode:
•

If the MASK[n] = 0, then bit n of the result is masked and not considered.

•

If MASK[n] = 1, and the content of bit[n] is the same as MATCH[n] in HSPI_PSMAR,
then there is a match for bit n.

If PMM = 0 in HSPI_CR, the AND-match mode is activated: SMF is set in HSPI_SR only
when there is a match on all of the unmasked bits.
If PMM = 1 in HSPI_CR, the OR-match mode is activated: SMF gets set if there is a match
on any of the unmasked bits.
An interrupt is called when SMF = 1 if SMIE = 1.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

If APMS is set in HSPI_CR, the operation stops and BUSY goes to 0 as soon as a match is
detected. Otherwise, BUSY stays at 1 and the periodic accesses continue until there is an
abort or until the HSPI is disabled (EN = 0).
HSPI_DR contains the latest received status bytes (FIFO deactivated). The content of this
register is not affected by the masking used in the matching logic. FTF in HSPI_SR is set as
soon as a new reading of the status is complete. FTF is cleared as soon as the data is read.
In automatic status-polling mode, variable latency is not supported. As a consequence, the
memory must be configured in fixed latency.

30.4.11

HSPI memory-mapped mode
When configured in memory-mapped mode, the external SPI device is seen as an internal
memory.

Note:

No more than 256 Mbytes can be addressed even if the external device capacity is larger.
If an access is made to an address outside of the range defined by DEVSIZE[4:0] but still
within the 256-Mbyte range, then an AHB error is given. The effect of this error depends on
the AHB master that attempted the access:
•

If it is the Cortex CPU, a hard-fault interrupt is generated.

•

If it is a DMA, a DMA transfer error is generated, and the corresponding DMA channel
is automatically disabled.

Byte, half-word, and word access types are all supported.
A support for execute in place (XIP) operation is implemented, where the HSPI continues to
load the bytes to the addresses following the most recent access. If subsequent accesses
are continuous to the bytes that follow, then these operations end up quickly since their
results were prefetched.
By default, the HSPI never stops its prefetch operation, it either keeps the previous read
operation active with the NCS maintained low or it relaunches a new transfer, even if no
access to the external device occurs for a long time.
Since external devices tend to consume more power when the NCS is held low, the
application may want to activate the timeout counter (TCEN = 1 in HSPI_CR): the NCS is
released after a period defined by TIMEOUT[15:0] in HSPI_LPTR, when x cycles have
elapsed without an access since the clock is inactive.
BUSY goes high as soon as the first memory-mapped access occurs. Because of the
prefetch operations, BUSY does not fall until there is an abort, or the peripheral is disabled.
It is not recommended to program the flash memory using the memory-mapped writes. The
indirect-write mode fulfills this operation.
However, if the application requires the use of the MCE for encryption (check MCE product
availability), the memory-mapped write mode may be used to program encrypted data to
external flash memory under the following conditions:

<!-- pagebreak -->

•

Prefetch must be enabled.

•

In block cipher mode, the CPU must write a complete 128-bit data block to prevent the
MCE from initiating read-modify-write operations when only a few bytes need to be

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
programmed. This precaution avoids incorrect programming operations. There are no
specific constraints to respect if the MCE is used in stream cipher mode.
•

Apply the abort sequence to exit memory-mapped mode when the data linked to the
page has been written in the external memory buffers. The abort sequence triggers the
start of the page programming.

•

Switch to the automatic status-polling mode to monitor the completion of the page
programing phase.

•

Relaunch the write enable command in indirect mode, then switch back to the memorymapped mode configuration to continue to program additional pages if any.

It is recommended to add a synchronization barrier between the end of the controller
registers configuration and the first memory-mapped access to the external memory when
the controller is configured in memory-mapped mode.

30.4.12

HSPI configuration introduction
The HSPI configuration is done in three steps:

30.4.13

1.

HSPI system configuration

2.

HSPI device configuration

3.

HSPI mode configuration

HSPI system configuration
The HSPI is configured using HSPI_CR. The user must program:
•

the functional mode with FMODE[1:0]

•

the automatic status-polling mode behavior if needed with PMM and APMS

•

the FIFO level with FTHRES

•

DMA use with DMAEN

•

the timeout counter use with TCEN

•

the dual-memory configuration, if needed, with DMM

In case of an interrupt use, the respective enable bit can also be set during this phase.
If the timeout counter is used, the timeout value is programmed in HSPI_LPTR.
The DMA channel must not be enabled during the HSPI configuration: it must be enabled
only when the operation is fully configured, to avoid any unexpected request generation.
The DMA and HSPI must be configured in a coherent manner regarding data length:
FTHRES value must reflect the DMA burst size.

30.4.14

HSPI device configuration
The parameters related to the external device targeted are configured through HSPI_DCR1
and HSPI_DCR2.The user must program:
•

the device size with DEVSIZE[4:0]

•

the chip-select minimum high time with CSHT[5:0]

•

the device frequency with PRESCALER[7:0]

DEVSIZE[4:0] defines the size of external memory using the following formula:
Number of bytes in the device = 2[DEVSIZE+1]

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

where DEVSIZE+1 is the number of address bits required to address the external device.
The external device capacity can go up to 4 Gbytes (addressed using 32 bits) in indirect
mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes.
If DMM = 1, DEVSIZE[4:0] must reflect the total capacity of the two devices together
considering the above formula (DEVSIZE[4:0] value is so equal to one of the two memory
capacities).
When the HSPI executes two commands, one immediately after the other, it raises the NCS
high between the two commands, at least one CLK cycle by default.
If the external device requires more time between commands, CSHT[5:0] can be used to
specify the minimum number of CLK cycles (up to 64) for which the NCS must remain high.
CKMODE indicates the level that the CLK takes between commands (when NCS = 1).
In HyperBus protocol, the device timing (tACC and tRWR) and the Latency mode must be
configured in HSPI_HLCR.

Memory types
External memory providers may present some architecture and slight data formatting
differences between them. The bitfield MTYP[2:0] into the HSPI_CR register allows
targeting the right controller configuration depending on the associated memory type
selected in the application. This is the responsibility of the software developer to align the
controller configuration to fit with the targeted memory type.
The memory types are grouped in a such way:
•

D0/D1 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 000. For instance, Micron is using such data ordering. In this
configuration, the DQS is sent with a polarity inverted respect to the clock polarity.
Figure 180. D0/D1 data ordering in octal-SPI DTR mode (Micron) - Read access
§

NCS

DQS
IO[7:0]

EEh

11h

A[31:24] A[23:16]

A[15:8]

A[7:0]

Address

§

§

§

CLK

Dummy

D0

D1

Word
unit

D2

D3

Word
unit
MSv71558V1

<!-- pagebreak -->

•

D1/D0 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 001. For instance, Macronix is using this reverse data
ordering in its Octaflash portfolio (this configuration is not adapted to its OctaRAM™
memories). DQS is keeping the same polarity as the clock when reading data from the
memory. Refer to Figure 170: DTR read in octal-SPI mode with DQS (Macronix mode)
example.

•

D1/D0 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 011 with specific address phase built with row and column to
fit with Macronix OctaRAM™ memories requirement (refer to Table 268: OctaRAM

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
command address bit assignment (based on 64 Mb OctaRAM). This is the controller
which translates internally the targeted address provided by the software in row/column
address formatting to sent to the memory. DQS is keeping the same polarity as the
clock when reading data from the memory.
Figure 181. OctaRAM read operation with reverse data ordering D1/D0
§

NCS

§

CLK

§

DQS

D1

§

IO[7:0]
Command

Raw address

D3

D0

Word Unit

Column address

D2

Word Unit

Command & Address

MSv71560V1

Table 268. OctaRAM command address bit assignment
(based on 64 Mb(1) OctaRAM)
Clock

1st clock

2nd clock

3rd clock

Function

Command

Row address

Column address

SIO[7]

Reserved

RA7

CA9

Reserved

SIO[6]

Reserved

RA6

CA8

Reserved

SIO[5]

Reserved

RA5

CA7

Reserved

RA12

RA4

CA6

Reserved

RA11

RA3

CA5

CA3

SIO[2]

RA10

RA2

CA4

CA2

SIO[1]

RA9

RA1

Reserved

CA1

SIO[0]

RA8

RA0

Reserved

CA0(2)

SIO[4]
SIO[3]

Command

1. Example of 64 Mb OctaRAM address assignment:
Row Address [RA12:RA0]: 8K. Column address [CA9:CA0]: 1K. 64 Mb density = 8K x 1K x 8 bits
2. Column address A0 must be always 0.

•

HyperBus memories need to be selected when targeted by the application. The
configuration to set depends on the access type:
–

HyperBus memory mode: The protocol follows the HyperBus specification.
MTYP[2:0] = 100 is the configuration to use to access the memory space.

–

HyperBus register mode (addressing register space): the memory-mapped
accesses in this mode must be noncacheable, or the indirect read/write modes

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

must be used. The configuration to be set for this particular register space access
is MTYP[2:0] = 101.

30.4.15

•

The software must configure MTYP[2:0] = 110 when the memory targeted comes from
APmemory and DMODE[2:0] = 101 to fit with the memory provider requirements
concerning the address formating.

•

Standard mode. It is the mode to use whenever the targeted memory is not
corresponding to any others configurations described in this section. MTYP[2:0] = 010
for this standard mode.

HSPI regular-command mode configuration
Indirect mode configuration
When FMODE[1:0] = 00, the indirect-write mode is selected and data can be sent to the
external device. When FMODE[1:0] = 01, the indirect-read mode is selected and data can
be read from the external device.
When the HSPI is used in indirect mode, the frames are constructed in the following way:
1.

Specify a number of data bytes to read or write in HSPI_DLR.

2.

Specify the frame timing in HSPI_TCR.

3.

Specify the frame format in HSPI_CCR.

4.

Specify the instruction in HSPI_IR.

5.

Specify the optional alternate byte to be sent right after the address phase in
HSPI_ABR.

6.

Specify the targeted address in HSPI_AR.

7.

Enable the DMA channel if needed.

8.

Read/write the data from/to the FIFO through HSPI_DR (if no DMA usage).

If neither the address register (HSPI_AR) nor the data register (HSPI_DR) need to be
updated for a particular command, then the command sequence starts as soon as HSPI_IR
is written. This is the case when both ADMODE[2:0] and DMODE[2:0] equal 000, or if just
ADMODE[2:0] = 000 when in indirect-read mode (FMODE[1:0] = 01).
When an address is required (ADMODE[2:0] ≠ 000) and the data register does not need to
be written (FMODE[1:0] = 01 or DMODE[2:0] = 000), the command sequence starts as
soon as the address is updated with a write to HSPI_AR.
In case of data transmission (FMODE[1:0] = 00 and DMODE[2:0] ≠ 000), the
communication start is triggered by a write in the FIFO through HSPI_DR.

Automatic status-polling mode configuration
The automatic status-polling mode is enabled by setting FMODE[1:0] = 10. In this mode, the
programmed frame is sent and the data is retrieved periodically.
The maximum amount of data read in each frame is 4 bytes. If more data is requested in
HSPI_DLR, it is ignored, and only 4 bytes are read.The periodicity is specified in HSPI_PIR.
Once the status data has been retrieved, the following can be processed:

<!-- pagebreak -->

•

Set SMF (an interrupt is generated if enabled).

•

Stop automatically the periodic retrieving of the status bytes.

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
The received value can be masked with the value stored in HSPI_PSMKR, and can be
ORed or ANDed with the value stored in HSPI_PSMAR.
In case of a match, SMF is set and an interrupt is generated if enabled. The HSPI can be
automatically stopped if AMPS is set. In any case, the latest retrieved value is available in
HSPI_DR.
When the HSPI is used in automatic status-polling mode, the frames are constructed in the
following way:
1.

Specify the input mask in HSPI_PSMKR.

2.

Specify the comparison value in HSPI_PSMAR.

3.

Specify the read period in HSPI_PIR.

4.

Specify a number of data bytes to read in HSPI_DLR.

5.

Specify the frame timing in HSPI_TCR.

6.

Specify the frame format in HSPI_CCR.

7.

Specify the instruction in HSPI_IR.

8.

Specify the optional alternate byte to be sent right after the address phase in
HSPI_ABR.

9.

Specify the optional targeted address in HSPI_AR.

If the address register (HSPI_AR) does not need to be updated for a particular command,
then the command sequence starts as soon as HSPI_CCR is written. This is the case when
ADMODE[2:0] = 000.
When an address is required (ADMODE[2:0] ≠ 000), the command sequence starts as soon
as the address is updated with a write to HSPI_AR.

Memory-mapped mode configuration
In memory-mapped mode, the external device is seen as an internal memory but with some
latency during accesses. Read and write operations are allowed to the external device in
this mode.
It is not recommended to program the flash memory using memory-mapped writes, as the
internal flags for erase or programming status have to be polled. The indirect-write mode
fulfills this operation, possibly in conjunction with the automatic status-polling mode.
Memory-mapped mode is entered by setting FMODE[1:0] = 11 in HSPI_CR.
The programmed instruction and frame are sent when an AHB master accesses the
memory mapped space.
The FIFO is used as a prefetch buffer to anticipate any linear reads. Any access to
HSPI_DR in this mode returns zero.
The data length register (HSPI_DLR) has no meaning in memory-mapped mode.
When the HSPI is used in memory-mapped mode, the frames are constructed in the
following way:
1.

Specify the frame timing in HSPI_TCR for read operation.

2.

Specify the frame format in HSPI_CCR for read operation.

3.

Specify the instruction in HSPI_IR.

4.

Specify the optional alternate byte to be sent right after the address phase in
HSPI_ABR for read operation.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

5.

Specify the frame timing in HSPI_WTCR for write operation.

6.

Specify the frame format in HSPI_WCCR for write operation.

7.

Specify the instruction in HSPI_WIR.

8.

Specify the optional alternate byte to be sent right after the address phase in
HSPI_WABR for write operation.

All the configuration operations must be completed (ensured by checking BUSY = 0) before
the first access to the memory area: any register write operation when BUSY = 1 have no
effect and is not signaled with an error response. On the first access, the HSPI becomes
busy, and no further configuration is allowed. Then, the only way to get BUSY low is to clear
the ENABLE bit or to abort by setting the ABORT bit.

HSPI delayed data sampling when no DQS is used
By default, when no DQS is used, the HSPI samples the data driven by the external device
one half of a CLK cycle after the external device drives the signal.
In case of any external signal delays, it may be useful to sample the data later. Using
SSHIFT in HSPI_TCR, the sampling of the data can be shifted by half of a CLK cycle.
The firmware must clear SSHIFT when the data phase is configured in DTR mode
(DDTR = 1).

HSPI delayed data sampling when DQS is used
When external DQS is used as a sampling clock, it is shifted precisely by one quarter of the
SPI clock cycle, for all frequencies above freq_min, to compensate the data propagation
delay in the “high-speed interface” when the product embeds one.

30.4.16

HSPI HyperBus protocol configuration
Indirect mode configuration (HyperBus)
When FMODE[1:0] = 00, the indirect-write mode is selected and data can be sent to the
external device. When FMODE[1:0] = 01, the indirect-read mode is selected where data can
be read from the external device. ADMODE must be configured with a value different from
000 (for instance ADMODE = 100).
When the HSPI is used in indirect mode, the frames are constructed in the following way:
1.

Specify a number of data bytes to read or write in HSPI_DLR.

2.

Specify the targeted address in HSPI_AR.

3.

Enable the DMA channel if needed.

4.

Read/write the data from/to the FIFO through HSPI_DR (if no DMA usage).

In indirect-read mode, the command sequence starts as soon as the address is updated
with a write to HSPI_AR.
In indirect-write mode, the communication start is triggered by a write in the FIFO through
HSPI_DR.

Memory-mapped mode configuration (HyperBus)
In memory-mapped mode, the external device is seen as an internal memory but with some
latency during the accesses. Read and write operations are allowed to the external device in
this mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
It is not recommended to program the flash memory using the memory-mapped writes:
the indirect-write mode fulfills this operation.
The memory-mapped mode is entered by setting FMODE[1:0] = 11. The programmed
instruction and frame is sent when an AHB master is accessing the memory mapped space.
The FIFO is used as a prefetch buffer to anticipate any linear reads. Any access to
HSPI_DR in this mode returns zero.
The data length register (HSPI_DLR) has no meaning in memory-mapped mode.
All the configuration operation must be completed before the first access to the memory
area. On the first access, the HSPI becomes busy, and no configuration is allowed.
Then, the only way to get BUSY low is to clear the ENABLE bit or to abort by setting
the ABORT bit.

30.4.17

HSPI error management
The following errors set the TEF flag in HSPI_SR and generates an interrupt if enabled
(TEIE = 1 in HSPI_CR):
•

in indirect or automatic status-polling mode, when a wrong address has been
programmed in HSPI_AR (according to the device size defined by DEVSIZE[4:0]).

•

in indirect mode, if the address plus the data length exceed the device size. TEF is set
as soon as the access is triggered.

In memory-mapped mode, the HSPI generates an AHB slave error in the following
situations:

30.4.18

•

The memory-mapped mode is disabled and an AHB read or write request occurs.

•

Read or write address exceeds the size of the external memory.

•

Abort is received while a read or write burst is ongoing when the abort condition is
present at the moment of transfer phases preceding the data phase (In incremental or
wrap burst type modes). To avoid the error, refer to Section 30.4.19: HSPI BUSY and
ABORT.

•

The HSPI is disabled while a read or write burst is requested.

•

A write wrap burst is received.

•

A write request is received while DQSE = 0 in HSPI_WCCR in octal DTR mode, in
dual-memory configuration, in hyperbus mode or 16-bit mode.

•

A read or write request is received while DMODE[2:0] = 000 (no data phase), except
when MTYP[2:0] is HyperBus.

•

Illegal access size when wrap read burst. This means AHB Bus transfer size (HSIZE) is
different from 4 bytes (only for memory-mapped mode).

•

Bit DMM is set in HSPI_CR (dual-memory configuration) and 16-bit mode is set.

HSPI high-speed interface and calibration
To reach higher frequencies, a dedicated high-speed interface is inserted between the HSPI
(or the I/O manager in case the product embeds one), and the I/O pads.
The following is valid for all data bus sizes 1, 2, 4, 8 or 16 bits.
The high-speed interface block embeds resynchronization registers that are clocked by a
delayed clock created from a DLL (delay locked loop) also located in the high-speed
interface. The high-speed interface features are controlled by registers located in the HSPI.

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

The purpose of resynchronization is primary to shift data or data strobe by one quarter of
controller bus clock period, with a correct timing accuracy. DLL must be calibrated versus
this clock period.
The calibration process is automatically enabled when one of the three conditions below is
met:
•

The HSPI exits reset state.

•

A value is written in PRESCALER[7:0] of HSPI_DCR2.

•

A value is written in HSPI_CCR.

The calibration process starts when the two following conditions are both met:
•

The calibration has been enabled by one of the three conditions above.

•

An action that sets BUSY = 1 is performed. For example the first transfer to memory
after calibration is enabled. When the calibration is completed, BUSY returns to 0.

In case a periodic recalibration is needed (for example to take in account possible variations
in temperature or power supply on a long duration), this recalibration must be triggered by
writing periodically in PRESCALER[7:0] of HSPI_DCR2, while BUSY = 0.
Once the calibration is completed, the value of the SPI bus clock period, expressed in
number of unitary delay, is available to user in COARSE[5:0] and FINE[6:0] of
HSPI_CALFCR.
After auto-calibration, HSPI_CALSOR and HSPI_CALSIR are automatically loaded with the
same value that corresponds to the delay for a quarter cycle.
The automatic calibration is not executed if at least one of both registers
HSPI_CALSOR/HSPI_CALSIR is written between the last write operation on HSPI_DCR2
or HSPI_CCR and the next transfer start. In such case, the calibration values must be set by
the software code into HSPI_CALSOR and HSPI_CALSIR registers. It may improve the
flash programming time without relaunching the automatic calibration processing between
two programming pages.
When the memory is not supporting DQS (DQSE = 0), the automatic calibration is not used
in reception. The DLL Master is used instead for delaying the feedback clock
(HSPI_CALMR). This delay needs to be adjusted by the application itself, using a software
sequence that determines which delay is optimal to guarantee the correct read operations.
When the clock is divided in DTR transmission mode, the quarter cycle delays on DQS/data
are not inserted by the DLLs themselves, but by internal flops design scheme. In SDR
transmission mode, the DLLs are not used and this, whatever the clock prescaler value.
In case of DTR mode and prescaler bypassed (PRESCALER[7:0] = 0), the kernel clock
provided to interface must have a 50 % duty-cycle.
When using the high-speed-interface, the system clock (AHB clock) must be at least as fast
as the SPI clock.

30.4.19

HSPI BUSY and ABORT
Once the HSPI starts an operation with the external device, BUSY is automatically set in
HSPI_SR.
In indirect mode, BUSY is reset once the HSPI has completed the requested command
sequence and the FIFO is empty.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Hexadeca-SPI interface (HSPI)
In automatic status-polling mode, BUSY goes low only after the last periodic access is
complete, due to a match when APMS = 1 or due to an abort.
After the first access in memory-mapped mode, BUSY goes low only on an abort.
Any operation can be aborted by setting ABORT in HSPI_CR. Once the abort is completed,
BUSY and ABORT are automatically reset, and the FIFO is flushed.
Before setting ABORT, the software must ensure that all the current transactions are
finished using the synchronization barriers. When DMA is enabled to handle the data read
or write operations in HSPI_DR, it is recommended to disable the DMA channel before
aborting the HSPI.

Note:

Some devices may misbehave if a write operation to a status register is aborted.

30.4.20

HSPI reconfiguration or deactivation
Before any HSPI reconfiguration, the software must ensure that all the transactions are
completed:

30.4.21

•

After a memory-mapped write, the software must perform a dummy read followed by a
synchronization barrier, then an abort.

•

After a memory-mapped read, the software must perform a synchronization barrier
then an abort.

NCS behavior
By default, NCS is high, deselecting the external device. NCS falls before an operation
begins and rises as soon as it finishes.
When CKMODE = 0 (clock mode 0: CLK stays low when no operation is in progress), NCS
falls one CLK cycle before an operation first rising CLK edge, and NCS rises one CLK cycle
after the operation final rising CLK edge (see the figure below).
Figure 182. NCS when CKMODE = 0 (T = CLK period)
T

T

NCS

CLK

MSv44100V2

RM0456 Rev 6

<!-- pagebreak -->

1177

Hexadeca-SPI interface (HSPI)

RM0456

When CKMODE = 1 (clock mode 3: CLK goes high when no operation is in progress) and
when in SDR mode, NCS falls one CLK cycle before an operation first rising CLK edge, and
NCS rises one CLK cycle after the operation final rising CLK edge (see the figure below).
Figure 183. NCS when CKMODE = 1 in SDR mode (T = CLK period)
T

T

NCS
CLK

MSv44101V2

When the CKMODE = 1 (clock mode 3) and DDTR = 1 (data DTR mode), NCS falls one
CLK cycle before an operation first rising CLK edge, and NCS rises one CLK cycle after the
operation final active rising CLK edge (see the figure below). Because the DTR operations
must finish with a falling edge, the CLK is low when NCS rises, and CLK rises back up one
half of a CLK cycle afterwards.
Figure 184. NCS when CKMODE = 1 in DTR mode (T = CLK period)
T/2
T

T

NCS

CLK

MSv44102V2

When the FIFO stays full during a read operation, or if the FIFO stays empty during a write
operation, the operation stalls and CLK stays low until the software services the FIFO. If an
abort occurs when an operation is stalled, NCS rises just after the abort is requested and
then CLK rises one half of a CLK cycle later (see the figure below).
Figure 185. NCS when CKMODE =1 with an abort (T = CLK period)
T/2
Clock stalled

T
NCS
CLK
Abort

MSv44103V2

<!-- pagebreak -->

