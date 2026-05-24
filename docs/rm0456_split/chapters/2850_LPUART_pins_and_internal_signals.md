RM0456 Rev 6

RM0456

Serial audio interface (SAI)

69.4

SAI functional description

69.4.1

SAI block diagram
Figure 859 shows the SAI block diagram while Table 705 and Table 706 list SAI internal and
external signals.
Figure 859. SAI functional block diagram
32-bit APB bus

Audio block A
FIFO ctrl

FIFO
sai_a_
ker_ck

Clock generator
Audio block A

FSM

Configuration
and status
registers

32-bit shift register

sai_pclk
Audio block B
FIFO ctrl

FIFO
sai_b_
ker_ck

Clock generator
Audio block B

Synchro
ctrl out

PDM_IF
Configuration
and status
registers

sai_sync_out_sck
sai_sync_out_fs
FS_A
SCK_A
SD_A
MCLK_A
FS_B
SCK_B
SD_B
MCLK_B
D[4:1](1)
CK[4:1](1)

FSM
32-bit shift register

SAI_BCR1

sai_sync_in_sck

sai_sync_in_fs

APB Interface
sai_b_gbl_it sai_b_dma
32-bit APB bus

From other SAI Blocks

SAI_ACR1

SAI_GCR

IO Line Management

APB Interface

Synchro
in

SAI

To other SAI Blocks

sai_a_gbl_it sai_a_dma

MSv62453V1

1. These signals might not be available for all SAI instances. Refer to Section 69.3: SAI implementation for details.

The SAI is mainly composed of two audio subblocks with their own clock generator. Each
audio block integrates a 32-bit shift register controlled by their own functional state machine.
Data are stored or read from the dedicated FIFO. FIFO may be accessed by the CPU, or by
DMA to leave the CPU free during the communication. Each audio block is independent.
They can be synchronous with each other.
An I/O line controller manages a set of 4 dedicated pins (SD, SCK, FS, MCLK) for a given
audio block in the SAI. Some of these pins can be shared if the two subblocks are declared
as synchronous to leave some free to be used as general purpose I/Os. The MCLK pin can
be output, or not, depending on the application, the decoder requirement and whether the
audio block is configured as the master.
If one SAI is configured to operate synchronously with another one, even more I/Os can be
freed (except for pins SD_x).

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

The functional state machine can be configured to address a wide range of audio protocols.
Some registers are present to set up the desired protocols (audio frame waveform
generator).
The audio subblock can be a transmitter or receiver, in master or slave mode. The master
mode means the SCK_x bit clock and the frame synchronization signal are generated from
the SAI, whereas in slave mode, they come from another external or internal master. There
is a particular case for which the FS signal direction is not directly linked to the master or
slave mode definition. In AC’97 protocol, it is an SAI output even if the SAI (link controller) is
set up to consume the SCK clock (and so to be in Slave mode).
Note:

For ease of reading of this section, the notation SAI_x refers to SAI_A or SAI_B, where ‘x’
represents the SAI A or B subblock.

69.4.2

SAI pins and internal signals
Table 705. SAI internal input/output signals
Internal signal name Signal type
sai_a_gbl_it/
sai_b_gbl_it
sai_a_dma,
sai_b_dma

Output

Description
Audio block A and B global interrupts

Input/output Audio block A and B DMA acknowledges and requests

sai_sync_out_sck,
sai_sync_out_fs

Output

Internal clock and frame synchronization output signals
exchanged with other SAI blocks

sai_sync_in_sck,
sai_sync_in_fs

Input

Internal clock and frame synchronization input signals
exchanged with other SAI blocks

sai_a_ker_ck/
sai_b_ker_ck

Input

Audio block A/B kernel clock

sai_pclk

Input

APB clock

Table 706. SAI input/output pins
Pin name
SAI_SCK_A/B
SAI_MCLK_A/B

Pin type

Comments

Input/output Audio block A/B bit clock
Output

Audio block A/B master clock

SAI_SD_A/B

Input/output Data line for block A/B

SAI_FS_A/B

Input/output Frame synchronization line for audio block A/B

SAI_CK[4:1]

Output

PDM bitstream clock(1)

SAI_D[4:1]

Input

PDM bitstream data(1)

1. These signals might not be available in all SAI instances. Refer to Section 69.3: SAI implementation for
details.

<!-- pagebreak -->

