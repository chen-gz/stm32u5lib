1104

Octo-SPI interface (OCTOSPI)

RM0456

The MAXTRAN[7:0] value must be greater than the minimal transaction size in terms of
number of cycles including the command, address, alternate, and dummy phases.
Note:

The communication regulation feature cannot be used in write mode for the flash memory
devices that require extra command to reenable the write operation after the NCS is active
again.
If NCS boundary, refresh, and communication regulation are enabled at the same time, the
NCS is released on the first condition met.

Restarting after an interrupted transfer
When a read or write operation is interrupted by a timeout or communication regulation
feature, the Octo-SPI interface, as soon as possible after getting back the port ownership,
reissues the initial command sequence together with the address following the last address
actually accessed before interruption. The transfer initially set goes on and ends
seamlessly.

28.4.8

OCTOSPI operating mode introduction
The OCTOSPI has the following operating modes regardless of the low-level protocol used
(either regular-command or HyperBus):

28.4.9

•

indirect mode (read or write)

•

automatic status-polling mode (only in regular-command protocol)

•

memory-mapped mode

OCTOSPI indirect mode
In indirect mode, the commands are started by writing to the OCTOSPI registers, and data
are transferred by writing or reading the data register, in a similar way to other
communication peripherals.
When FMODE[1:0] = 00 in OCTOSPI_CR, the OCTOSPI is in indirect-write mode: bytes are
sent to the external device during the data phase. Data are provided by writing to
OCTOSPI_DR.
When FMODE[1:0] = 01, the OCTOSPI is in indirect-read mode: bytes are received from the
external device during the data phase. Data are recovered by reading OCTOSPI_DR.
In indirect mode, when the OCTOSPI is configured in DTR mode over eight lanes with DQS
disabled, the given starting address and the data length must be even.

Note:

The OCTOSPI_AR register must be updated even if the start address is the same as the
start address of the previous indirect access.
The number of bytes to be read/written is specified in OCTOSPI_DLR:

<!-- pagebreak -->

•

If DL[31:0] = 0xFFFF FFFF, the data length is considered undefined and the OCTOSPI
simply continues to transfer data until it reaches the end of the external device (as
defined by DEVSIZE). If no bytes are to be transferred, DMODE[2:0] must be set to 0 in
OCTOSPI_CCR.

•

If DL[31:0] = 0xFFFF FFFFF and DEVSIZE[4:0] = 0x1F (its maximum value indicating
at 4-Gbyte device), the transfers continue indefinitely, stopping only after an abort
request or after the OCTOSPI is disabled. After the last memory address is read (at
address 0xFFFF FFFF), reading continues with address = 0x0000 0000.

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)
When the programmed number of bytes to be transmitted or received is reached, the TCF
bit is set in OCTOSPI_SR, and an interrupt is generated if TCIE = 1 in OCTOSPI_CR. In the
case of an undefined number of data, TCF is set when the limit of the external SPI memory
is reached, according to the device size defined in OCTOSPI_DCR1.

Triggering the start of a transfer in regular-command protocol
Depending on the OCTOSPI configuration, there are three different ways to trigger the start
of a transfer in indirect mode when using the regular-command protocol. In general, the start
of transfer is triggered as soon as the software gives the last information that is necessary
for the command. More specifically in indirect mode, a transfer starts when one of the
following sequence of events occurs:
•

if no address is necessary (ADMODE[2:0] = 000) and if no data need to be provided by
the software (FMODE[1:0] = 01 or DMODE[2:0] = 000), and at the moment when a
write is performed to INSTRUCTION[31:0] in OCTOSPI_IR

•

if an address is necessary (when ADMODE[2:0] ≠ 000) and if no data need to be
provided by the software (when FMODE[1:0] = 01 or DMODE[2:0] = 000), and at the
moment when a write is performed to ADDRESS[31:0] in OCTOSPI_AR

•

if data need to be provided by the software (when FMODE[1:0] = 00 and
DMODE[2:0] ≠ 000), and at the moment when a write is performed to DATA[31:0] in
OCTOSPI_DR

A write to OCTOSPI_ABR never triggers the communication start. If alternate bytes are
required, they must have been programmed before.
As soon as a command is started, the BUSY bit is automatically set in OCTOSPI_SR.

Triggering the start of a transfer in HyperBus protocol
Depending on the OCTOSPI configuration, there are different ways to trigger the start of a
command in indirect mode. In general, it is triggered as soon as the firmware gives the last
information that is necessary for the transfer to start, and more specifically, a communication
in indirect mode is triggered by one of the following register settings, when it is the last one
to be executed:
•

when a write is performed to ADDRESS[31:0] (OCTOSPI_AR) with
ADMODE[2:0] ≠ 000 in indirect read mode (FMODE[1:0] = 01).

•

when a write is performed to DATA[31:0] (OCTOSPI_DR) in indirect-write mode (when
FMODE = 00).

•

when a (dummy) write is performed to INSTRUCTION[31:0] (OCTOSPI_IR) for indirect
read mode (with ADMODE[2:0] = 000 and FMODE = 01).

As soon as a transfer is started, the BUSY bit (OCTOSPI_SR[5]) is automatically set.

FIFO and data management
Data in indirect mode passes through a 32-byte FIFO that is internal to the OCTOSPI.
FLEVEL in OCTOSPI_SR indicates how many bytes are currently being held in the FIFO.
AHB burst transactions are supported. Data of the burst are successively written
in OCTOSPI_DR, and immediately transferred in the internal FIFO.
In indirect-write mode (FMODE[1:0] = 00), the software adds data to the FIFO when it writes
in OCTOSPI_DR. A word write adds 4 bytes to the FIFO, a half-word write adds 2 bytes,
and a byte write adds only 1 byte. If the software adds too many bytes to the FIFO (more

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

than indicated in DL[31:0]), the extra bytes are flushed from the FIFO at the end of the write
operation (when TCF is set).
The byte/half-word accesses to OCTOSPI_DR must be done only to the least significant
byte/halfword of the 32-bit register.
FTHRES is used to define a FIFO threshold after which point the FIFO threshold flag, FTF,
gets set. In indirect-read mode, FTF is set when the number of valid bytes to be read from
the FIFO is above the threshold. FTF is also set if there is any data left in the FIFO after the
last byte is read from the external device, regardless of FTHRES setting. In indirect-write
mode, the FTF is set when the number of empty bytes in the FIFO is above the threshold.
If FTIE = 1, there is an interrupt when the FTF is set. If DMAEN = 1, a DMA transfer is
initiated when the FTF is set. The FTF is cleared by hardware as soon as the threshold
condition is no longer true (after enough data has been transferred by the CPU or DMA).
The last data read in RX FIFO remains valid as long as there is no request for the next line.
This means that, when the application reads several times in a row at the same location, the
data is provided from the RX FIFO and not read again from the distant memory.

28.4.10

OCTOSPI automatic status-polling mode
In automatic status-polling mode, the OCTOSPI periodically starts a command to read a
defined number of status bytes (up to four). The received bytes can be masked to isolate
some status bits and an interrupt can be generated when the selected bits have a defined
value. The automatic status-polling mode must be used only in regular-command protocol.
For HyperBus protocol, it is not exploitable since the read status register into the
HyperFlash memory must be performed in two steps (a write operation followed by a read
operation).
The access to the device begins in the same manner as in indirect-read mode. BUSY in
OCTOSPI_SR goes high at this point and stays high even between the periodic accesses.
The content of MASK[31:0] in OCTOSPI_PSMAR is used to mask the data from the
external device in automatic status-polling mode:
•

If the MASK[n] = 0, then bit n of the result is masked and not considered.

•

If MASK[n] = 1, and the content of bit[n] is the same as MATCH[n] in
OCTOSPI_PSMAR, then there is a match for bit n.

If PMM = 0 in OCTOSPI_CR, the AND-match mode is activated: SMF is set in
OCTOSPI_SR only when there is a match on all of the unmasked bits.
If PMM = 1 in OCTOSPI_CR, the OR-match mode is activated: SMF gets set if there is a
match on any of the unmasked bits.
An interrupt is called when SMF = 1 if SMIE = 1.
If APMS is set in OCTOSPI_CR, the operation stops and BUSY goes to 0 as soon as a
match is detected. Otherwise, BUSY stays at 1 and the periodic accesses continue until
there is an abort or until the OCTOSPI is disabled (EN = 0).
OCTOSPI_DR contains the latest received status bytes (FIFO deactivated). The content of
this register is not affected by the masking used in the matching logic. FTF in OCTOSPI_SR
is set as soon as a new reading of the status is complete. FTF is cleared as soon as the
data is read.
In automatic status-polling mode, variable latency is not supported. The memory must then
be configured in fixed latency.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

28.4.11

Octo-SPI interface (OCTOSPI)

OCTOSPI memory-mapped mode
When configured in memory-mapped mode, the external SPI device is seen as an internal
memory.

Note:

No more than 256 Mbytes can be addressed even if the external device capacity is larger.
If an access is made to an address outside of the range defined by DEVSIZE[4:0] but still
within the 256 Mbytes range, then an AHB error is given. The effect of this error depends on
the AHB master that attempted the access:
•

If it is the Cortex CPU, a hard-fault interrupt is generated.

•

If it is a DMA, a DMA transfer error is generated, and the corresponding DMA channel
is automatically disabled.

Byte, half-word, and word access types are all supported.
A support for execute in place (XIP) operation is implemented, where the OCTOSPI
continues to load the bytes to the addresses following the most recent access. If
subsequent accesses are continuous to the bytes that follow, then these operations end up
quickly since their results were prefetched.
By default, the OCTOSPI never stops its prefetch operation. It either keeps the previous
read operation active with the NCS maintained low or it relaunches a new transfer, even
if no access to the external device occurs for a long time.
Since external devices tend to consume more power when the NCS is held low, the
application may want to activate the timeout counter (TCEN = 1 in OCTOSPI_CR): the NCS
is released after a period defined by TIMEOUT[15:0] in OCTOSPI_LPTR, when x cycles
have elapsed without access since the clock is inactive.
BUSY goes high as soon as the first memory-mapped access occurs. Because of the
prefetch operations, BUSY does not fall until there is an abort, or the peripheral is disabled.
It is not recommended to program the flash memory using the memory-mapped writes.
The indirect-write mode fulfills this operation.
However, if the application requires the use of the MCE for encryption (check MCE product
availability), the memory-mapped write mode may be used to program encrypted data to
external flash memory under the following conditions:
•

Prefetch must be enabled.

•

In block cipher mode, the CPU must write a complete 128-bit data block to prevent the
MCE from initiating read-modify-write operations when only a few bytes need to be
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

Relaunch the write enable command in indirect mode, then switch back to the
memory-mapped mode configuration to continue to program additional pages if any.

It is recommended to add a synchronization barrier between the end of the controller
registers configuration and the first memory-mapped access to the external memory when
the controller is configured in memory-mapped mode.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

28.4.12

RM0456

OCTOSPI configuration introduction
The OCTOSPI configuration is done in three steps:

28.4.13

1.

OCTOSPI system configuration

2.

OCTOSPI device configuration

3.

OCTOSPI mode configuration

OCTOSPI system configuration
The OCTOSPI is configured using OCTOSPI_CR. The user must program:
•

the functional mode with FMODE[1:0]

•

the automatic status-polling mode behavior if needed with PMM and APMS

•

the FIFO level with FTHRES

•

the DMA use with DMAEN

•

the timeout counter use with TCEN

•

the dual-memory configuration, if needed, with DMM

In case of an interrupt use, the respective enable bit can also be set during this phase.
If the timeout counter is used, the timeout value is programmed in OCTOSPI_LPTR.
The DMA channel must not be enabled during the OCTOSPI configuration: it must be
enabled only when the operation is fully configured, to avoid any unexpected request
generation.
The DMA and OCTOSPI must be configured in a coherent manner regarding data length:
FTHRES value must reflect the DMA burst size.

28.4.14

OCTOSPI device configuration
The parameters related to the external device targeted are configured through
OCTOSPI_DCR1 and OCTOSPI_DCR2.The user must program:
•

the device size with DEVSIZE[4:0]

•

the chip-select minimum high time with CSHT[5:0]

•

the clock mode with FRCK and CKMODE

•

the device frequency with PRESCALER[7:0]

DEVSIZE[4:0] defines the size of external memory using the following formula:
Number of bytes in the device = 2DEVSIZE+1
where DEVSIZE+1 is the number of address bits required to address the external device.
The external device capacity can go up to 4 Gbytes (addressed using 32 bits) in indirect
mode, but the addressable space in memory-mapped mode is limited to 256 Mbytes.
If DMM = 1, DEVSIZE[4:0] must reflect the total capacity of the two devices together
considering the above formula (DEVSIZE[4:0] value is so equal to one of the two memory
capacities).
When the OCTOSPI executes two commands, one immediately after the other, it raises the
chip-select signal (NCS) high between the two commands for only one CLK cycle by default.
If the external device requires more time between commands, the chip-select high time
CSHT[5:0] can be used to specify the minimum number of CLK cycles for which the NCS
must remain high.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)
CKMODE indicates the level that the CLK takes between commands (when NCS = 1).
In HyperBus protocol, the device timing (tACC and tRWR) and the latency mode must be
configured in OCTOSPI_HLCR.

Memory types
External memory providers may present some architecture and slight data formatting
differences between them. The bitfield MTYP[2:0] into the OCTOSPI_CR register allows
targeting the right controller configuration depending on the associated memory type
selected in the application. This is the responsibility of the software developer to align the
controller configuration to fit with the targeted memory type.
The memory types are grouped in a such way:
•

D0/D1 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 000. For instance, Micron is using such data ordering. In this
configuration, the DQS is sent with a polarity inverted respect to the clock polarity.
Figure 157. D0/D1 data ordering in octal-SPI DTR mode (Micron) - Read access
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

•

D1/D0 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 001. For instance, Macronix is using this reverse data
ordering in its Octaflash portfolio (this configuration is not adapted to its OctaRAM™
memories). DQS is keeping the same polarity as the clock when reading data from the
memory. Refer to Figure 148: DTR read in octal-SPI mode with DQS (Macronix mode)
example.

•

D1/D0 data ordering in octal-SPI data mode (DMODE[2:0] = 100) in DTR mode by
configuring MTYP[2:0] = 011 with specific address phase built with row and column to
fit with Macronix OctaRAM™ memories requirement (refer to Table 256: OctaRAM
command address bit assignment (based on 64-Mbyte OctaRAM). This is the controller
which translates internally the targeted address provided by the software in row/column
address formatting to sent to the memory. DQS is keeping the same polarity as the
clock one when reading data from the memory.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

Figure 158. OctaRAM read operation with reverse data ordering D1/D0
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

Table 256. OctaRAM command address bit assignment
(based on 64-Mbyte(1) OctaRAM)
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

1. Example of 64-Mbyte OctaRAM address assignment:
Row Address [RA12:RA0]: 8K. Column address [CA9:CA0]: 1K. 64-Mbyte density = 8K x 1K x 8 bits
2. Column address A0 must be always 0.

•

•

<!-- pagebreak -->

HyperBus memories need to be selected when targeted by the application. The
configuration to set depends on the access type:
–

HyperBus memory mode: The protocol follows the HyperBus specification.
MTYP[2:0] = 100 is the configuration to use to access the memory space.

–

HyperBus register mode (addressing register space): the memory-mapped
accesses in this mode must be noncacheable, or the indirect read/write modes
must be used. The configuration to be set for this particular register space access
is MTYP[2:0] = 101.

Standard mode. It is the mode to use whenever the targeted memory is not
corresponding to any others configurations described in this section. MTYP[2:0] = 010
for this standard mode.

RM0456 Rev 6

RM0456

28.4.15

Octo-SPI interface (OCTOSPI)

OCTOSPI regular-command mode configuration
Indirect mode configuration
When FMODE[1:0] = 00, the indirect-write mode is selected and data can be sent to the
external device. When FMODE[1:0] = 01, the indirect-read mode is selected, and data can
be read from the external device.
When the OCTOSPI is used in indirect mode, the frames are constructed in the following
way:
1.

Specify a number of data bytes to read or write in OCTOSPI_DLR.

2.

Specify the frame timing in OCTOSPI_TCR.

3.

Specify the frame format in OCTOSPI_CCR.

4.

Specify the instruction in OCTOSPI_IR.

5.

Specify the optional alternate byte to be sent right after the address phase in
OCTOSPI_ABR.

6.

Specify the targeted address in OCTOSPI_AR.

7.

Enable the DMA channel if needed.

8.

Read/write the data from/to the FIFO through OCTOSPI_DR (if no DMA usage).

If neither the address register (OCTOSPI_AR) nor the data register (OCTOSPI_DR) need to
be updated for a particular command, then the command sequence starts as soon as
OCTOSPI_IR is written. This is the case when both ADMODE[2:0] and DMODE[2:0]
equal 000, or if just ADMODE[2:0] = 000 when in indirect-read mode (FMODE[1:0] = 01).
When an address is required (ADMODE[2:0] ≠ 000) and the data register does not need to
be written (FMODE[1:0] = 01 or DMODE[2:0] = 000), the command sequence starts as
soon as the address is updated with a write to OCTOSPI_AR.
In case of data transmission (FMODE[1:0] = 00 and DMODE[2:0] ≠ 000),
the communication start is triggered by a write in the FIFO through OCTOSPI_DR.

Automatic status-polling mode configuration
The automatic status-polling mode is enabled by setting FMODE[1:0] = 10. In this mode, the
programmed frame is sent and data are retrieved periodically.
The maximum amount of data read in each frame is 4 bytes. If more data is requested in
OCTOSPI_DLR, it is ignored, and only 4 bytes are read. The periodicity is specified in
OCTOSPI_PIR.
Once the status data has been retrieved, the following can be processed:
•

Set SMF (an interrupt is generated if enabled).

•

Stop automatically the periodic retrieving of the status bytes.

The received value can be masked with the value stored in OCTOSPI_PSMKR, and can be
ORed or ANDed with the value stored in OCTOSPI_PSMAR.
In case of a match, SMF is set and an interrupt is generated if enabled. The OCTOSPI can
be automatically stopped if AMPS is set. In any case, the latest retrieved value is available
in OCTOSPI_DR.
When the OCTOSPI is used in automatic status-polling mode, the frames are constructed in
the following way:
1.

Specify the input mask in OCTOSPI_PSMKR.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

2.

Specify the comparison value in OCTOSPI_PSMAR.

3.

Specify the read period in OCTOSPI_PIR.

4.

Specify a number of data bytes to read in OCTOSPI_DLR.

5.

Specify the frame timing in OCTOSPI_TCR.

6.

Specify the frame format in OCTOSPI_CCR.

7.

Specify the instruction in OCTOSPI_IR.

8.

Specify the optional alternate byte to be sent right after the address phase in
OCTOSPI_ABR.

9.

Specify the optional targeted address in OCTOSPI_AR.

If the address register (OCTOSPI_AR) does not need to be updated for a particular
command, then the command sequence starts as soon as OCTOSPI_CCR is written. This
is the case when ADMODE[2:0] = 000.
When an address is required (ADMODE[2:0] ≠ 000), the command sequence starts as soon
as the address is updated with a write to OCTOSPI_AR.

Memory-mapped mode configuration
In memory-mapped mode, the external device is seen as an internal memory but with some
latency during accesses. Read and write operations are allowed to the external device in
this mode.
It is not recommended to program the flash memory using memory-mapped writes, as the
internal flags for erase or programming status have to be polled. The indirect-write mode
fulfills this operation, possibly in conjunction with the automatic status-polling mode.
The memory-mapped mode is entered by setting FMODE[1:0] = 11 in OCTOSPI_CR.
The programmed instruction and frame are sent when an AHB master accesses the
memory-mapped space.
The FIFO is used as a prefetch buffer to anticipate any linear reads. Any access to
OCTOSPI_DR in this mode returns zero.
The data length register (OCTOSPI_DLR) has no meaning in memory-mapped mode.
When the OCTOSPI is used in memory-mapped mode, the frames are constructed
in the following way:
1.

Specify the frame timing in OCTOSPI_TCR for read operation.

2.

Specify the frame format in OCTOSPI_CCR for read operation.

3.

Specify the instruction in OCTOSPI_IR.

4.

Specify the optional alternate byte to be sent right after the address phase in
OCTOSPI_ABR for read operation.

5.

Specify the frame timing in OCTOSPI_WTCR for write operation.

6.

Specify the frame format in OCTOSPI_WCCR for write operation.

7.

Specify the instruction in OCTOSPI_WIR.

8.

Specify the optional alternate byte to be sent right after the address phase in
OCTOSPI_WABR for write operation.

All configuration operations must be completed (ensured by checking BUSY = 0) before the
first access to the memory area: any register write operation when BUSY = 1 has no effect
and is not signaled with an error response. On the first access, the OCTOSPI becomes

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Octo-SPI interface (OCTOSPI)
busy, and no further configuration is allowed. Then, the only way to get BUSY low is to clear
the ENABLE bit or to abort by setting the ABORT bit.

OCTOSPI delayed data sampling when no DQS is used
By default, when no DQS is used, the OCTOSPI samples the data driven by the external
device one half of a CLK cycle after the external device drives the signal.
In case of any external signal delays, it may be useful to sample the data later. Using
SSHIFT in OCTOSPI_TCR, the sampling of the data can be shifted by half of a CLK cycle.
The firmware must clear SSHIFT when the data phase is configured in DTR mode
(DDTR = 1).

OCTOSPI delayed data sampling when DQS is used
When external DQS is used as a sampling clock, it can be shifted in time to compensate the
data propagation delay. This shift is performed by an external delay block located outside
the OCTOSPI. The control of this feature depends on the device implementation (see the
product reference manual for more details).
In configurations where delay does not need to be compensated, the external delay block
can be bypassed by setting DLYBYP in OCTOSPI_DCR1.

Sending the instruction only once (SIOO)
A flash memory can provide a mode where an instruction must be sent only with the first
command sequence, while subsequent commands start directly with the address. The user
can take advantage of this type of features using SIOO in OCTOSPI_CCR.
SIOO is valid for memory-mapped mode only. If this bit is set, the instruction is sent only for
the first command following a write to OCTOSPI_CCR.
Subsequent command sequences skip the instruction phase, until there is a write to
OCTOSPI_CCR. SIOO has no effect when IMODE[1:0] = 00 (no instruction).
The SIOO mode is not supported when any of the communication regulations, NCS
boundary, or refresh features are used.

28.4.16

OCTOSPI HyperBus protocol configuration
Indirect mode configuration (HyperBus)
When FMODE[1:0] = 00, the indirect-write mode is selected and data can be sent to the
external device. When FMODE[1:0] = 01, the indirect-read mode is selected where data can
be read from the external device. ADMODE must be configured with a value different from
000 (for instance ADMODE = 100).
When the OCTOSPI is used in indirect mode, the frames are constructed in the following
way:
1.

Specify a number of data bytes to read or write in OCTOSPI_DLR.

2.

Specify the targeted address in OCTOSPI_AR.

3.

Enable the DMA channel if needed.

4.

Read/write the data from/to the FIFO through OCTOSPI_DR (if no DMA usage).

In indirect-read mode, the command sequence starts as soon as the address is updated
with a write to OCTOSPI_AR.

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

In indirect-write mode, the communication start is triggered by a write in the FIFO through
OCTOSPI_DR.

Memory-mapped mode configuration (HyperBus)
In memory-mapped mode, the external device is seen as an internal memory but with some
latency during the accesses. Read and write operations are allowed to the external device in
this mode.
It is not recommended to program the flash memory using the memory-mapped writes:
the indirect-write mode fulfills this operation.
The memory-mapped mode is entered by setting FMODE[1:0] = 11. The programmed
instruction and frame is sent when an AHB master accesses the memory-mapped space.
The FIFO is used as a prefetch buffer to anticipate any linear reads. Any access to
OCTOSPI_DR in this mode returns zero.
The data length register (OCTOSPI_DLR) has no meaning in memory-mapped mode.
All the configuration operation must be completed before the first access to the memory
area. On the first access, the OCTOSPI becomes busy, and no configuration is allowed.
Then, the only way to get BUSY low is to clear the ENABLE bit, or to abort by setting the
ABORT bit.

28.4.17

OCTOSPI error management
The following errors set the TEF flag in OCTOSPI_SR and generates an interrupt if enabled
(TEIE = 1 in OCTOSPI_CR):
•

in indirect or automatic status-polling mode, when a wrong address has been
programmed in OCTOSPI_AR (according to the device size defined by DEVSIZE[4:0]).

•

in indirect mode, if the address plus the data length exceed the device size: TEF is set
as soon as the access is triggered.

In memory-mapped mode, the OCTOSPI generates an AHB slave error in the following
situations,:

<!-- pagebreak -->

•

The memory-mapped mode is disabled and an AHB read or write request occurs.

•

A read or write address exceeds the size of the external memory.

•

An abort is received while a read or write burst is ongoing.

•

The OCTOSPI is disabled while a read or write burst is requested.

•

A write wrap burst is received.

•

A write request is received while DQSE = 0 in OCTOSPI_WCCR in octal DTR mode, in
dual-memory configuration, in HyperBus mode.

•

A read or write request is received while DMODE[2:0] = 000 (no data phase), except
when MTYP[2:0] is HyperBus.

•

Illegal access size when wrap read burst. This means that the AHB bus transfer size
(HSIZE) is different from 4 bytes (only for memory-mapped mode).

•

Bit DMM is set in OCTOSPI_CR (dual-memory configuration) and octal mode is set.

RM0456 Rev 6

RM0456

28.4.18

Octo-SPI interface (OCTOSPI)

OCTOSPI BUSY and ABORT
Once the OCTOSPI starts an operation with the external device, BUSY is automatically set
in OCTOSPI_SR.
In indirect mode, BUSY is reset once the OCTOSPI has completed the requested command
sequence and the FIFO is empty.
In automatic status-polling mode, BUSY goes low only after the last periodic access is
complete, due to a match when APMS = 1 or due to an abort.
After the first access in memory-mapped mode, BUSY goes low only on an abort.
Any operation can be aborted by setting ABORT in OCTOSPI_CR. Once the abort is
completed, BUSY and ABORT are automatically reset, and the FIFO is flushed.
Before setting ABORT, the software must ensure that all the current transactions are
finished using the synchronization barriers. When DMA is enabled to handle the data read
or write operations in OCTOSPI_DR, it is recommended to disable the DMA channel before
aborting the OCTOSPI.

Note:

Some devices may misbehave if a write operation to a status register is aborted.

28.4.19

OCTOSPI reconfiguration or deactivation
Before any OCTOSPI reconfiguration, the software must ensure that all the transactions are
completed:

28.4.20

•

After a memory-mapped write, the software must perform a dummy read followed
by a synchronization barrier, then an abort.

•

After a memory-mapped read, the software must perform a synchronization barrier
than an abort.

NCS behavior
By default, NCS is high, deselecting the external device. NCS falls before an operation
begins and rises as soon as it finishes.
When CKMODE = 0 (clock mode 0: CLK stays low when no operation is in progress), NCS
falls one CLK cycle before an operation first rising CLK edge, and NCS rises one CLK cycle
after the operation final rising CLK edge (see the figure below).
Figure 159. NCS when CKMODE = 0 (T = CLK period)
T

T

NCS

SCLK

MSv44100V1

RM0456 Rev 6

<!-- pagebreak -->

1104

Octo-SPI interface (OCTOSPI)

RM0456

When CKMODE = 1 (clock mode 3: CLK goes high when no operation is in progress) and
when in SDR mode, NCS falls one CLK cycle before an operation first rising CLK edge, and
NCS rises one CLK cycle after the operation final rising CLK edge (see the figure below).
Figure 160. NCS when CKMODE = 1 in SDR mode (T = CLK period)
T

T

NCS
SCLK

MSv44101V1

When the CKMODE = 1 (clock mode 3) and DDTR = 1 (data DTR mode), NCS falls one
CLK cycle before an operation first rising CLK edge, and NCS rises one CLK cycle after the
operation final active rising CLK edge (see the figure below). Because the DTR operations
must finish with a falling edge, CLK is low when NCS rises, and CLK rises back up one half
of a CLK cycle afterwards.
Figure 161. NCS when CKMODE = 1 in DTR mode (T = CLK period)
T/2
T

T

NCS

SCLK

MSv44102V1

When the FIFO stays full during a read operation, or if the FIFO stays empty during a write
operation, the operation stalls and CLK stays low until the software services the FIFO. If an
abort occurs when an operation is stalled, NCS rises just after the abort is requested and
then CLK rises one half of a CLK cycle later (see the figure below).
Figure 162. NCS when CKMODE = 1 with an abort (T = CLK period)
T/2
Clock stalled

T
NCS
SCLK
Abort

MSv44103V1

<!-- pagebreak -->

