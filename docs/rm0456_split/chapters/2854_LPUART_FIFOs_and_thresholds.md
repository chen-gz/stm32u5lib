RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Table 707. External synchronization selection
Block instance

69.4.5

SYNCIN = 1

SYNCIN = 0

SAI1

SAI2 sync.

Reserved

SAI2

Reserved

SAI1 sync.

Audio data size
The audio frame can target different data sizes by configuring bit DS[2:0] in the SAI_xCR1
register. The data sizes may be 8, 10, 16, 20, 24, or 32 bits. During the transfer, either the
MSB or the LSB of the data is sent first, depending on the configuration of the LSBFIRST bit
in the SAI_xCR1 register.

69.4.6

Frame synchronization
The FS signal acts as the frame synchronization signal in the audio frame (start of frame).
The shape of this signal is completely configurable to target the different audio protocols
with their own specificities concerning this frame synchronization behavior. This
reconfigurability is done using the SAI_xFRCR register. Figure 860 illustrates this flexibility.
Figure 860. Audio frame
FS Length: up to 256 bits
FS active: up to 128 bits
FS

The falling edge can occur into this area

SCK
SD
(FSOFF = 0)

Slot 0

SD
(FSOFF = 1)

Slot 0

Slot 1

Slot 2

Slot 1

Slot 2

Slot 3

Slot 3

Slot 4

Slot 4

…...

…...

Slot 0

Slot 0

MSv30037V2

In AC’97 mode or in SPDIF mode (bit PRTCFG[1:0] = 10 or PRTCFG[1:0] = 01 in the
SAI_xCR1 register), the frame synchronization shape is forced to match the AC’97 protocol.
The SAI_xFRCR register value is ignored.
Each audio block is independent and consequently each one requires a specific
configuration.

Frame length
•

Master mode
The audio frame length can be configured to up to 256-bit clock cycles, by configuring
the FRL[7:0] field in the SAI_xFRCR register.
If the frame length is greater than the number of declared slots for the frame, the
remaining bits to transmit are extended to 0 or the SD line is released to high-Z

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

depending on the state of bit TRIS in the SAI_xCR2 register (refer to FS signal role). In
reception mode, the remaining bit is ignored.
If bit NODIV is cleared, (FRL+1) must be equal to a power of 2, from 8 to 256, to ensure
that an audio frame contains an integer number of MCLK pulses per bit clock cycle.
If bit NODIV is set, the (FRL+1) field can take any value from 8 to 256. Refer to
Section 69.4.8: SAI clock generator”
•

Slave mode
The audio frame length is mainly used to specify to the slave the number of bit clock
cycles per audio frame sent by the external master. It is used mainly to detect from the
master any anticipated or late occurrence of the frame synchronization signal during an
ongoing audio frame. In this case, an error is generated. For more details, refer to
Section 69.4.14: Error flags.
In slave mode, there are no constraints on the FRL[7:0] configuration in the
SAI_xFRCR register.

The number of bits in the frame is equal to FRL[7:0] + 1.
The minimum number of bits to transfer in an audio frame is 8.

Frame synchronization polarity
The FSPOL bit in the SAI_xFRCR register sets the active polarity of the FS pin from which a
frame is started. The start of the frame is edge sensitive.
In slave mode, the audio block waits for a valid frame to start transmitting or receiving. The
start of the frame is synchronized to this signal. It is effective only if the start of the frame is
not detected during an ongoing communication and assimilated to an anticipated start of
frame (refer to Section 69.4.14: Error flags).
In master mode, the frame synchronization is sent continuously each time an audio frame is
complete until the SAIEN bit in the SAI_xCR1 register is cleared. If no data are present in
the FIFO at the end of the previous audio frame, an underrun condition is managed as
described in Section 69.4.14: Error flags), but the audio communication flow is not
interrupted.

Frame synchronization active level length
The FSALL[6:0] bits of the SAI_xFRCR register enable the configuration of the length of the
active level of the frame synchronization signal. The length can be set from 1- to 128-bit
clock cycles.
As an example, the active length can be half of the frame length in I2S, LSB or MSB-justified
modes, or one-bit wide for PCM/DSP or TDM.

Frame synchronization offset
Depending on the audio protocol targeted in the application, the frame synchronization
signal can be asserted when transmitting the last bit or the first bit of the audio frame (this is
the case in I2S standard protocol and in MSB-justified protocol, respectively). The FSOFF
bit in the SAI_xFRCR register enables the possibility to choose between two configurations.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

FS signal role
The FS signal can have a different meaning depending on the FS function. FSDEF bit in the
SAI_xFRCR register selects which meaning it has:
•

0: start of frame, like, for instance, the PCM/DSP, TDM, AC’97, audio protocols,

•

1: start of frame and channel side identification within the audio frame like for the I2S or
the MSB- or LSB-justified protocols.

When the FS signal is considered as a start of frame and channel side identification within
the frame, the number of declared slots must be considered to be half the number for the left
channel and half the number for the right channel. If the number of bit clock cycles on half
audio frame is greater than the number of slots dedicated to a channel side, and TRIS = 0, 0
is sent for transmission for the remaining bit clock cycles in the SAI_xCR2 register.
Otherwise, if TRIS = 1, the SD line is released to high-Z. In reception mode, the remaining
bit clock cycles are not considered until the channel side changes.
Figure 861. FS role is start of frame + channel side identification (FSDEF = TRIS = 1)
Number of slots not aligned with the audio frame
Audio frame
Half of frame
FS

sck

slot

Slot 0 ON

Slot 3 ON Slot 4 OFF Slot 5 ON

Slot 1 OFF Slot 2 ON

Number of slots aligned with the audio frame
Audio frame
Half of frame

FS

sck

slot

Slot 0

Slot 1

Slot 2

Slot 3

Slot 4

Slot 5
MS30038V2

1. The frame length must be even.

If the FSDEF bit in SAI_xFRCR is kept clear, so FS signal is equivalent to a start of frame,
and if the number of slots defined in NBSLOT[3:0] in SAI_xSLOTR multiplied by the number
of bits by slot configured in SLOTSZ[1:0] in SAI_xSLOTR is less than the frame size (bit
FRL[7:0] in the SAI_xFRCR register), then:

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

•

If TRIS = 0 in the SAI_xCR2 register, the remaining bit after the last slot is forced to 0
until the end of frame in case of transmitter,

•

If TRIS = 1, the line is released to high-Z during the transfer of these remaining bits. In
reception mode, these bits are discarded.
Figure 862. FS role is start of frame (FSDEF = 0)
Audio frame

sck

slot

Slot 0

Slot 1

Slot 2

...

Slot n

Data = 0 after slot n if TRIS = 0
SD output released (HI-Z) after slot n if TRIS = 1
MS30039V1

The FS signal is not used when the audio block in transmitter mode is configured to get the
SPDIF output on the SD line. The corresponding FS I/O is released and left free for other
purposes.

69.4.7

Slot configuration
The slot is the basic element in the audio frame. The number of slots in the audio frame is
equal to NBSLOT[3:0] + 1.
The maximum number of slots per audio frame is fixed at 16.
For AC’97 protocol or SPDIF (when bit PRTCFG[1:0] = 10 or PRTCFG[1:0] = 01), the
number of slots is automatically set to target the protocol specification, and the value of
NBSLOT[3:0] is ignored.
Each slot can be defined as a valid slot, or not, by setting SLOTEN[15:0] bits of the
SAI_xSLOTR register.
When an invalid slot is transferred, the SD data line is either forced to 0 or released to highZ depending on the TRIS bit configuration (refer to Output data line management on an
inactive slot) in transmitter mode. In receiver mode, the received value from the end of this
slot is ignored. Consequently, there is no FIFO access and so no request to read or write the
FIFO linked to this inactive slot status.
The slot size is also configurable as shown in Figure 863. The size of the slots is selected by
configuring the SLOTSZ[1:0] bits in the SAI_xSLOTR register. The size is applied identically
for each slot in an audio frame.

<!-- pagebreak -->

