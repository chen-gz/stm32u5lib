RM0456 Rev 6

RM0456

Serial audio interface (SAI)
executed to recover from an underrun error detected via the underrun interrupt or the
underrun status bit:
1.

Disable the DMA stream (via the DMA peripheral) if the DMA is used.

2.

Disable the SAI and check that the peripheral is physically disabled by polling the
SAIEN bit in the SAI_xCR1 register.

3.

Clear the COVRUNDR flag in the SAI_xCLRFR register.

4.

Flush the FIFO by setting the FFLUSH bit in SAI_xCR2.
The software needs to point to the address of the future data corresponding to the start
of a new block (data for preamble B). If the DMA is used, the DMA source base
address pointer must be updated accordingly.

5.

Enable the DMA stream (DMA peripheral) again if the DMA is used to manage data
transfers according to the new source base address.

6.

Enable the SAI again by configuring the SAIEN bit in the SAI_xCR1 register.

Clock generator programming in SPDIF generator mode
For the SPDIF generator, the SAI provides a bit clock twice as fast as the symbol-rate. The
table below shows examples of symbol rates with respect to the audio sampling rate.
Table 714. Audio sampling frequency versus symbol rates
Audio sampling frequencies (FS)

Symbol-rate

44.1 kHz

2.8224 MHz

48 kHz

3.072 MHz

96 kHz

6.144 MHz

192 kHz

12.288 MHz

More generally, the relationship between the audio sampling frequency (FS) and the bit
clock rate (FSCK_X) is given by the formula:
F SCK_x
F S = ----------------------128
The bit clock rate is obtained as follows:
F sai_x_ker_ck
F SCK_x = ---------------------------------MCKDIV
Note:

The above formulas are valid only if NODIV is set to 1 in the SAI_ACR1 register.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

69.4.13

RM0456

Specific features
The SAI interface embeds specific features that can be useful depending on the audio
protocol selected. These functions are accessible through specific bits of the SAI_xCR2
register.

Mute mode
The mute mode can be used when the audio subblock is a transmitter or a receiver.
Audio subblock in transmission mode
In transmitter mode, the mute mode can be selected at any time. The mute mode is active
for entire audio frames. The MUTE bit in the SAI_xCR2 register enables the mute mode
when it is configured during an ongoing frame.
The mute mode bit is strobed only at the end of the frame. If it is set at this time, the mute
mode is active at the beginning of the new audio frame and for a complete frame, until the
next end of frame. The bit is then strobed to determine if the next frame is still a mute frame.
If the number of slots set through the NBSLOT[3:0] bits in the SAI_xSLOTR register is lower
than or equal to 2, it is possible to specify if the value sent in mute mode is 0 or if it is the last
value of each slot. The selection is done via the MUTEVAL bit in the SAI_xCR2 register.
If the number of slots set in the NBSLOT[3:0] bits in the SAI_xSLOTR register is greater
than 2, the MUTEVAL bit in the SAI_xCR2 register is meaningless as 0 values are sent on
each bit on each slot.
The FIFO pointers are still incremented in mute mode. This means that data present in the
FIFO and for which the mute mode is requested are discarded.
Audio subblock in reception mode
In reception mode, it is possible to detect a mute mode sent from the external transmitter
when all the declared and valid slots of the audio frame receive 0 for a given consecutive
number of audio frames (MUTECNT[5:0] bits in the SAI_xCR2 register).
When the number of MUTE frames is detected, the MUTEDET flag in the SAI_xSR register
is set and an interrupt can be generated if the MUTEDETIE bit is set in SAI_xCR2.
The mute frame counter is cleared when the audio subblock is disabled or when a valid slot
receives at least one data in an audio frame. The interrupt is generated just once, when the
counter reaches the value specified in the MUTECNT[5:0] bits. The interrupt event is then
reinitialized when the counter is cleared.
Note:

The mute mode is not available for SPDIF audio blocks.

Mono/stereo mode
In transmitter mode, the mono mode can be addressed without any data preprocessing in
memory, assuming the number of slots is equal to 2 (NBSLOT[3:0] = 0001 in SAI_xSLOTR).
In this case, the access time to and from the FIFO is reduced by 2 since the data for slot 0 is
duplicated into data slot 1.
To enable the mono mode:

<!-- pagebreak -->

1.

Set the MONO bit to 1 in the SAI_xCR1 register.

2.

Set NBSLOT to 1 and SLOTEN to 3 in SAI_xSLOTR.

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
In reception mode, the MONO bit can be set and is meaningful only if the number of slots is
equal to 2, like in transmitter mode. When it is set, only slot 0 data are stored in the FIFO.
The data belonging to slot 1 are discarded since, in this case, it is supposed to be the same
as the previous slot. If the data flow in reception mode is a real stereo audio flow with a
distinct and different left and right data, the MONO bit is meaningless. The conversion from
the output stereo file to the equivalent mono file is done by software.

Companding mode
Telecommunication applications can require processing the data to be transmitted or
received using a data companding algorithm.
Depending on the COMP[1:0] bits in the SAI_xCR2 register (used only when free protocol
mode is selected), the application software can choose to process or not the data before
sending it on the SD serial output line (compression) or to expand the data after the
reception on the SD serial input line (expansion), as illustrated in Figure 876. The two
companding modes supported are the µ-Law and the A-Law logs, which are a part of the
CCITT G.711 recommendation.
The companding standard used in the United States and Japan is the µ-Law. It supports 14
bits of dynamic range (COMP[1:0] = 10 in the SAI_xCR2 register).
The European companding standard is A-Law and supports 13 bits of dynamic range
(COMP[1:0] = 11 in the SAI_xCR2 register).
Both µ-Law and A-Law companding standard can be computed based on 1’s complement or
2’s complement representation, depending on the CPL bit setting in the SAI_xCR2 register.
In µ-Law and A-Law standards, data are coded as 8 bits with MSB alignment. Companded
data are always 8 bits wide. For this reason, the DS[2:0] bits in the SAI_xCR1 register are
forced to 010 when the SAI audio block is enabled (the SAIEN bit = 1 in the SAI_xCR1
register) and when one of these two companding modes is selected through the COMP[1:0]
bits.
If no companding processing is required, the COMP[1:0] bits must be kept clear.
Figure 876. Data companding hardware in an audio block in the SAI
Receiver mode (bit MODE[0] = 1 in SAI_xCR1)
COMP[1]

FIFO

1

SD

expand

32-bit shift register

0

Transmitter mode (bit MODE[0] = 0 in SAI_xCR1)

FIFO

0

SD
32-bit shift register

1

compress

COMP[1]
MSv19244V1

1. Not applicable when AC’97 or SPDIF are selected.

RM0456 Rev 6

<!-- pagebreak -->

