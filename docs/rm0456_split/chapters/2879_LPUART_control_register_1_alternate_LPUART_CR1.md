3020

Serial audio interface (SAI)

RM0456

Expansion and compression mode are automatically selected through SAI_xCR2:
•

If the SAI audio block is configured to be a transmitter, and if the COMP[1] bit is set in
the SAI_xCR2 register, the compression mode is applied.

•

If the SAI audio block is declared as a receiver, the expansion algorithm is applied.

Output data line management on an inactive slot
In transmitter mode, it is possible to choose the behavior of the SD line output when an
inactive slot is sent on the data line (via the TRIS bit).
•

Either the SAI forces 0 on the SD output line when an inactive slot is transmitted, or

•

The line is released in high-Z state at the end of the last bit of data transferred, to
release the line for other transmitters connected to this node.

It is important to note that the two transmitters cannot attempt to drive the same SD output
pin simultaneously, which may result in a short circuit. To ensure a gap between
transmissions, if the data is lower than 32-bit, the data can be extended to 32-bit by setting
the bit SLOTSZ[1:0] = 10 in the SAI_xSLOTR register. The SD output pin is then tri-stated at
the end of the LSB of the active slot (during the padding to 0 phase to extend the data to 32bit) if the following slot is declared inactive.
In addition, if the number of slots multiplied by the slot size is lower than the frame length,
the SD output line is tri-stated when the padding to 0 is done to complete the audio frame.
Figure 877 illustrates these behaviors.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Figure 877. Tristate strategy on SD output line on an inactive slot
Bit TRIS = 1 in the SAI_xCR1 and frame length = number of slots
Audio frame
sck
Slot size = data size
Slot 0 ON

slot
SD (output)

Slot 1 OFF

Slot 2 OFF

Data 0

Slot 3 ON
Data 1

.. ON

.. ON

..

..

Slot n ON
Data m

Slot size > data size
slot

Slot 0 ON

SD (output)

Data 0

slot

Slot 0 ON

Slot 1 OFF

Slot 2 OFF

Slot 3 ON
Data 1

Slot 1 OFF

Slot 2 OFF

Slot 3 ON
Data 0

SD (output)

.. ON

.. ON

..

..

Slot n ON

.. ON

.. ON

Slot n ON

..

..

Data m

Data m

Bit TRIS = 1 in the SAI_xCR1 and frame length > number of slots
Audio frame
sck
Slot size = data size
slot
SD (output)

Slot 0 ON

Slot 1 OFF

Slot 2 OFF

Data 0

.. ON
..

Slot n ON
Data m

Slot size > data size
slot

Slot 0 ON

SD (output)

Data 0

slot

Slot 0 ON

Slot 1 OFF

Slot 1 OFF

Slot 2 OFF

Slot 2 OFF

.. ON

Data m

.. ON

.. ON

..

SD (output)

Slot n ON

..

Data m
MSv192345V1

When the selected audio protocol uses the FS signal as a start of frame and a channel side
identification (bit FSDEF = 1 in the SAI_xFRCR register), the tristate mode is managed
according to Figure 878 (where the bit TRIS in the SAI_xCR1 register = 1, and FSDEF=1,
and half frame length is higher than number of slots/2, and NBSLOT=6).

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Figure 878. Tristate on output data line in a protocol like I2S

sck

Slot size = data size
slot
SD (output)

Slot 0 ON

Slot 1 OFF

Data 0

Slot 2 ON

Slot 3 ON

Data 1

Data 2

Slot 4 OFF

Slot 5 ON
Data 3

Slot size > data size
slot

Slot 0 ON

SD (output)

Data 0

slot

Slot 0 ON

SD (output)

Data 0

Slot 1 OFF

Slot 1 OFF

Slot 2 ON

Slot 3 ON

Data 1

Data 2

Slot 2 ON

Slot 3 ON

Data 1

Slot 4 OFF

Slot 5 ON
Data 3

Slot 4 OFF

Slot 5 ON
Data m
MSv192346V1

If the TRIS bit in the SAI_xCR2 register is cleared, all the high impedance states on the SD
output line in Figure 877 and Figure 878 are replaced by a drive with a value of 0.

69.4.14

Error flags
The SAI implements the following error flags:
•

FIFO overrun/underrun.

•

Anticipated frame synchronization detection.

•

Late frame synchronization detection.

•

Codec not ready (AC’97 exclusively).

•

Wrong clock configuration in master mode.

FIFO overrun/underrun (OVRUDR)
The FIFO overrun/underrun bit is called OVRUDR in the SAI_xSR register.
The overrun or underrun errors share the same bit since an audio block can be either
receiver or transmitter and each audio block in a given SAI has its own SAI_xSR register.
Overrun
When the audio block is configured as receiver, an overrun condition may appear if data are
received in an audio frame when the FIFO is full and not able to store the received data. In
this case, the received data are lost, the OVRUDR flag in the SAI_xSR register is set, and
an interrupt is generated if the OVRUDRIE bit is set in the SAI_xIM register. The slot
number, from which the overrun occurs, is stored internally. No more data are stored into the
FIFO until it becomes free to store new data. When the FIFO has at least one data free, the
SAI audio block receiver stores new data (from a new audio frame) from the slot number
that was stored internally when the overrun condition was detected. This avoids data slot
dealignment in the destination memory (refer to Figure 879).
The OVRUDR flag is cleared when the COVRUDR bit is set in the SAI_xCLRFR register.

<!-- pagebreak -->

