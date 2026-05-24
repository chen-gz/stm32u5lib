RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Figure 879. Overrun detection error
Example: FIFO overrun on Slot 1
Audio frame

Audio frame

sck

data

Slot 0 ON

Slot 1 ON

Slot 1 ON

Slot 0 ON

Slot 1 ON

... ON

Slot n ON

Data stored again in FIFO

FIFO full
Received data discarded

OVRUDR
COVRUDR = 1

MSv192348V2

Underrun
An underrun may occur when the audio block in the SAI is a transmitter and the FIFO is
empty when data need to be transmitted. If an underrun is detected, the slot number for
which the event occurs is stored and the MUTE value (00) is sent until the FIFO is ready to
transmit the data corresponding to the slot for which the underrun was detected (refer to
Figure 880). This avoids desynchronization between the memory pointer and the slot in the
audio frame.
The underrun event sets the OVRUDR flag in the SAI_xSR register and an interrupt is
generated if the OVRUDRIE bit is set in the SAI_xIM register. To clear this flag, set the
COVRUDR bit in the SAI_xCLRFR register.
The underrun event can occur when the audio subblock is configured as master or slave.
Figure 880. FIFO underrun event
Example: FIFO underrun on Slot 1
Audio frame

Audio frame

sck
Slot size = data size
data
SD (output)

Slot 0 ON

MUTE

MUTE

MUTE

Slot 1 ON

... ON

Slot 0 ON

FIFO empty

OVRUND
OVRUND=1

MSv192347V2

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Anticipated frame synchronization detection (AFSDET)
The AFSDET flag is used only in slave mode. It is never asserted in master mode. It
indicates that a frame synchronization (FS) has been detected earlier than expected since
the frame length, the frame polarity, and the frame offset are defined and known.
Anticipated frame detection sets the AFSDET flag in the SAI_xSR register.
This detection has no effect on the current audio frame, which is not sensitive to the
anticipated FS. This means that “parasitic” events on signal FS are flagged without any
perturbation of the current audio frame.
An interrupt is generated if the AFSDETIE bit is set in the SAI_xIM register. To clear the
AFSDET flag, the CAFSDET bit must be set in the SAI_xCLRFR register.
To resynchronize with the master after an anticipated frame detection error, four steps are
required:

Note:

1.

Disable the SAI block by resetting the SAIEN bit in the SAI_xCR1 register. To make
sure that the SAI is disabled, read back the SAIEN bit and check it is set to 0.

2.

Flush the FIFO via the FFLUS bit in the SAI_xCR2 register.

3.

Enable the SAI peripheral again (SAIEN bit set to 1).

4.

The SAI block waits for the assertion on FS to restart the synchronization with master.

The AFSDET flag is not asserted in AC’97 mode since the SAI audio block acts as a link
controller and generates the FS signal even when declared as slave.It has no meaning in
SPDIF mode since the FS signal is not used.

Late frame synchronization detection
The LFSDET flag in the SAI_xSR register can be set only when the SAI audio block
operates as a slave. The frame length, the frame polarity, and the frame-offset configuration
are known in register SAI_xFRCR.
If the external master does not send the FS signal at the expected time, thus generating the
signal too late, the LFSDET flag is set and an interrupt is generated if the LFSDETIE bit is
set in the SAI_xIM register.
The LFSDET flag is cleared when the CLFSDET bit is set in the SAI_xCLRFR register.
The late frame synchronization detection flag is set when the corresponding error is
detected. The SAI needs to be resynchronized with the master (see sequence described in
Anticipated frame synchronization detection (AFSDET)).
In a noisy environment, glitches on the SCK clock may be wrongly detected by the audio
block state machine and shift the SAI data at a wrong frame position. This event can be
detected by the SAI and reported as a late frame synchronization detection error.
There is no corruption if the external master is not managing the audio data frame transfer in
continuous mode, which must not be the case in most applications. In this case, the
LFSDET flag is set.
Note:

<!-- pagebreak -->

