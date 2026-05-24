RM0456 Rev 6

RM0456

Serial audio interface (SAI)

Bits 7:6 SLOTSZ[1:0]: Slot size
This bits is set and cleared by software.
The slot size must be higher or equal to the data size. If this condition is not respected, the behavior
of the SAI is undetermined.
Refer to Output data line management on an inactive slot for information on how to drive SD line.
These bits must be set when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.
00: The slot size is equivalent to the data size (specified in DS[3:0] in the SAI_xCR1 register).
01: 16-bit
10: 32-bit
11: Reserved
Bit 5 Reserved, must be kept at reset value.
Bits 4:0 FBOFF[4:0]: First bit offset
These bits are set and cleared by software.
The value set in this bitfield defines the position of the first data transfer bit in the slot. It represents
an offset value. In transmission mode, the bits outside the data field are forced to 0. In reception
mode, the extra received bits are discarded.
These bits must be set when the audio block is disabled.
They are ignored in AC’97 or SPDIF mode.

69.6.6

SAI interrupt mask register (SAI_AIM)
Address offset: 0x14
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LFSDET AFSDETI CNRDY FREQ WCKCFG MUTEDET OVRUDR
IE
E
IE
IE
IE
IE
IE
rw

rw

rw

rw

rw

rw

rw

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 LFSDETIE: Late frame synchronization detection interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt is generated if the LFSDET bit is set in the SAI_xSR register.
This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.
Bit 5 AFSDETIE: Anticipated frame synchronization detection interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt is generated if the AFSDET bit in the SAI_xSR register is set.
This bit is meaningless in AC’97, SPDIF mode or when the audio block operates as a master.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Bit 4 CNRDYIE: Codec not ready interrupt enable (AC’97).
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When the interrupt is enabled, the audio block detects in the slot 0 (tag0) of the AC’97 frame if the
Codec connected to this line is ready or not. If it is not ready, the CNRDY flag in the SAI_xSR
register is set and an interrupt is generated.
This bit has a meaning only if the AC’97 mode is selected through PRTCFG[1:0] bits and the audio
block is operates as a receiver.
Bit 3 FREQIE: FIFO request interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt is generated if the FREQ bit in the SAI_xSR register is set.
Since the audio block defaults to operate as a transmitter after reset, the MODE bit must be
configured before setting FREQIE to avoid a parasitic interrupt in receiver mode,
Bit 2 WCKCFGIE: Wrong clock configuration interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
This bit is taken into account only if the audio block is configured as a master (MODE[1] = 0) and
NODIV = 0.
It generates an interrupt if the WCKCFG flag in the SAI_xSR register is set.
Note: This bit is used only in Free protocol mode and is meaningless in other modes.
Bit 1 MUTEDETIE: Mute detection interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt is generated if the MUTEDET bit in the SAI_xSR register is set.
This bit has a meaning only if the audio block is configured in receiver mode.
Bit 0 OVRUDRIE: Overrun/underrun interrupt enable.
This bit is set and cleared by software.
0: Interrupt is disabled
1: Interrupt is enabled
When this bit is set, an interrupt is generated if the OVRUDR bit in the SAI_xSR register is set.

69.6.7

SAI status register (SAI_ASR)
Address offset: 0x18
Reset value: 0x0000 0008

31

30

29

28

27

26

25

24

23

22

21

20

19

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

LFSDE
AFSDET CNRDY
T
r

<!-- pagebreak -->

r

RM0456 Rev 6

r

FREQ
r

18

17

16

FLVL[2:0]
r

r

r

2

1

0

WCKCFG MUTEDET OVRUDR
r

r

r

RM0456

Serial audio interface (SAI)

Bits 31:19 Reserved, must be kept at reset value.
Bits 18:16 FLVL[2:0]: FIFO level threshold.
This bit is read only. The FIFO level threshold flag is managed only by hardware and its setting
depends on SAI block configuration (transmitter or receiver mode).
000: FIFO empty (transmitter and receiver modes)
001: FIFO ≤ ¼ but not empty (transmitter mode), FIFO < ¼ but not empty (receiver mode)
010: ¼ < FIFO ≤ ½ (transmitter mode), ¼ ≤ FIFO < ½ (receiver mode)
011: ½ < FIFO ≤ ¾ (transmitter mode), ½ ≤ FIFO < ¾ (receiver mode)
100: ¾ < FIFO but not full (transmitter mode), ¾ ≤ FIFO but not full (receiver mode)
101: FIFO full (transmitter and receiver modes)
Others: Reserved
Bits 15:7 Reserved, must be kept at reset value.
Bit 6 LFSDET: Late frame synchronization detection.
This bit is read only.
0: No error.
1: Frame synchronization signal is not present at the right time.
This flag can be set only if the audio block is configured in slave mode.
It is not used in AC’97 or SPDIF mode.
It can generate an interrupt if LFSDETIE bit is set in the SAI_xIM register.
This flag is cleared when the software sets bit CLFSDET in SAI_xCLRFR register
Bit 5 AFSDET: Anticipated frame synchronization detection.
This bit is read only.
0: No error.
1: Frame synchronization signal is detected earlier than expected.
This flag can be set only if the audio block is configured in slave mode.
It is not used in AC’97 or SPDIF mode.
It can generate an interrupt if AFSDETIE bit is set in SAI_xIM register.
This flag is cleared when the software sets CAFSDET bit in SAI_xCLRFR register.
Bit 4 CNRDY: Codec not ready.
This bit is read only.
0: External AC’97 Codec is ready
1: External AC’97 Codec is not ready
This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register and
configured in receiver mode.
It can generate an interrupt if CNRDYIE bit is set in SAI_xIM register.
This flag is cleared when the software sets CCNRDY bit in SAI_xCLRFR register.
Bit 3 FREQ: FIFO request.
This bit is read only.
0: No FIFO request.
1: FIFO request to read or to write the SAI_xDR.
The request depends on the audio block configuration:
– If the block is configured in transmission mode, the FIFO request is related to a write request
operation in the SAI_xDR.
– If the block configured in reception, the FIFO request related to a read request operation from the
SAI_xDR.
This flag can generate an interrupt if FREQIE bit is set in SAI_xIM register.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Bit 2 WCKCFG: Wrong clock configuration flag.
This bit is read only.
0: Clock configuration is correct
1: Clock configuration does not respect the rule concerning the frame length specification defined in
Section 69.4.6: Frame synchronization (configuration of FRL[7:0] bit in the SAI_xFRCR register)
This bit is used only when the audio block operates in master mode (MODE[1] = 0) and NODIV = 0.
It can generate an interrupt if WCKCFGIE bit is set in SAI_xIM register.
This flag is cleared when the software sets CWCKCFG bit in SAI_xCLRFR register.
Bit 1 MUTEDET: Mute detection.
This bit is read only.
0: No MUTE detection on the SD input line
1: MUTE value detected on the SD input line (0 value) for a specified number of consecutive audio
frame
This flag is set if consecutive 0 values are received in each slot of a given audio frame and for a
consecutive number of audio frames (set in the MUTECNT bit in the SAI_xCR2 register).
It can generate an interrupt if MUTEDETIE bit is set in SAI_xIM register.
This flag is cleared when the software sets bit CMUTEDET in the SAI_xCLRFR register.
Bit 0 OVRUDR: Overrun / underrun.
This bit is read only.
0: No overrun/underrun error.
1: Overrun/underrun error detection.
The overrun and underrun conditions can occur only when the audio block is configured as a
receiver and a transmitter, respectively.
It can generate an interrupt if OVRUDRIE bit is set in SAI_xIM register.
This flag is cleared when the software sets COVRUDR bit in SAI_xCLRFR register.

69.6.8

SAI clear flag register (SAI_ACLRFR)
Address offset: 0x1C
Reset value: 0x0000 0000

31

30

26

25

24

23

22

21

20

19

18

17

16

Res.

Res. Res. Res. Res. Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

9

8

7

6

5

4

3

2

1

0

Res.

CWCKCF
G

Res.

29

13

28

12

27

11

10

Res. Res. Res. Res. Res.

Res.

Res.

Res.

CLFSDET CAFSDET
w

w

CCNRDY
w

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 CLFSDET: Clear late frame synchronization detection flag.
This bit is write only.
Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register.
This bit is not used in AC’97 or SPDIF mode
Reading this bit always returns the value 0.

<!-- pagebreak -->

RM0456 Rev 6

w

CMUTE COVRUD
DET
R
w

w

RM0456

Serial audio interface (SAI)

Bit 5 CAFSDET: Clear anticipated frame synchronization detection flag.
This bit is write only.
Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register.
It is not used in AC’97 or SPDIF mode.
Reading this bit always returns the value 0.
Bit 4 CCNRDY: Clear Codec not ready flag.
This bit is write only.
Programming this bit to 1 clears the CNRDY flag in the SAI_xSR register.
This bit is used only when the AC’97 audio protocol is selected in the SAI_xCR1 register.
Reading this bit always returns the value 0.
Bit 3 Reserved, must be kept at reset value.
Bit 2 CWCKCFG: Clear wrong clock configuration flag.
This bit is write only.
Programming this bit to 1 clears the WCKCFG flag in the SAI_xSR register.
This bit is used only when the audio block is set as master (MODE[1] = 0) and NODIV = 0 in the
SAI_xCR1 register.
Reading this bit always returns the value 0.
Bit 1 CMUTEDET: Mute detection flag.
This bit is write only.
Programming this bit to 1 clears the MUTEDET flag in the SAI_xSR register.
Reading this bit always returns the value 0.
Bit 0 COVRUDR: Clear overrun / underrun.
This bit is write only.
Programming this bit to 1 clears the OVRUDR flag in the SAI_xSR register.
Reading this bit always returns the value 0.

69.6.9

SAI data register (SAI_ADR)
Address offset: 0x20
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

DATA[31:16]
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

rw

rw

rw

rw

rw

rw

rw

DATA[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:0 DATA[31:0]: Data
A write to this register loads the FIFO provided the FIFO is not full.
A read from this register empties the FIFO if the FIFO is not empty.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

69.6.10

RM0456

SAI configuration register 1 (SAI_BCR1)
Address offset: 0x24
Reset value: 0x0000 0040

31

30

29

28

27

26

MCK
EN

OSR

Res.

Res.

Res.

Res.

15

14

13

12

Res.

Res.

OUTD
MONO
RIV
rw

rw

25

24

23

22

21

20

MCKDIV[5:0]

19

18

NODIV

Res.

rw

rw

rw

rw

rw

rw

rw

rw

rw

11

10

9

8

7

6

5

4

3

SYNCEN[1:0]
rw

rw

CKSTR LSBFIRST
rw

rw

DS[2:0]
rw

rw

Res.
rw

2

17

16

DMAEN SAIEN
rw

rw

1

0

PRTCFG[1:0]

MODE[1:0]

rw

rw

rw

rw

Bits 31:28 Reserved, must be kept at reset value.
Bit 27 MCKEN: Master clock generation enable
0: The master clock is not generated
1: The master clock is generated independently of SAIEN bit
Bit 26 OSR: Oversampling ratio for master clock
This bit is meaningful only when NODIV bit is set to 0.
0: Master clock frequency = FFS x 256
1: Master clock frequency = FFS x 512
Bits 25:20 MCKDIV[5:0]: Master clock divider
These bits are set and cleared by software.
000000: Divides by 1 the kernel clock input (sai_x_ker_ck).
Otherwise, The master clock frequency is calculated according to the formula given in
Section 69.4.8: SAI clock generator.
These bits have no meaning when the audio block is slave.
They have to be configured when the audio block is disabled.
Bit 19 NODIV: No divider
This bit is set and cleared by software.
0: the ratio between the Master clock generator and frame synchronization is fixed to 256 or 512
1: the ratio between the Master clock generator and frame synchronization depends on FRL[7:0]
Bit 18 Reserved, must be kept at reset value.
Bit 17 DMAEN: DMA enable
This bit is set and cleared by software.
0: DMA disabled
1: DMA enabled
Note: Since the audio block defaults to operate as a transmitter after reset, the MODE[1:0] bits must
be configured before setting DMAEN to avoid a DMA request in receiver mode.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

Bit 16 SAIEN: Audio block enable
This bit is set by software.
To switch off the audio block, the application software must program this bit to 0 and poll the bit till it
reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it
is set to 0, otherwise the enable command is not taken into account.
This bit enables to control the state of the SAI audio block. If it is disabled when an audio frame
transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this
audio frame transfer.
0: SAI audio block disabled
1: SAI audio block enabled.
Note: When the SAI block (A or B) is configured in master mode, the clock must be present on the
SAI block input before setting SAIEN bit.
Bits 15:14 Reserved, must be kept at reset value.
Bit 13 OUTDRIV: Output drive
This bit is set and cleared by software.
0: Audio block output driven when SAIEN is set
1: Audio block output driven immediately after the setting of this bit.
Note: This bit has to be set before enabling the audio block and after the audio block configuration.
Bit 12 MONO: Mono mode
This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2.
When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates
as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are
stored. Refer to Section : Mono/stereo mode for more details.
0: Stereo mode
1: Mono mode.
Bits 11:10 SYNCEN[1:0]: Synchronization enable
These bits are set and cleared by software. They must be configured when the audio subblock is
disabled.
00: audio subblock in asynchronous mode.
01: audio subblock is synchronous with the other internal audio subblock. In this case, the audio
subblock must be configured in slave mode
10: audio subblock is synchronous with an external SAI embedded peripheral. In this case the audio
subblock must be configured in Slave mode.
11: Reserved
Note: The audio subblock must be configured as asynchronous when SPDIF mode is enabled.
Bit 9 CKSTR: Clock strobing edge
This bit is set and cleared by software. It must be configured when the audio block is disabled. This
bit has no meaning in SPDIF audio protocol.
0: Signals generated by the SAI change on SCK rising edge, while signals received by the SAI are
sampled on the SCK falling edge.
1: Signals generated by the SAI change on SCK falling edge, while signals received by the SAI are
sampled on the SCK rising edge.
Bit 8 LSBFIRST: Least significant bit first
This bit is set and cleared by software. It must be configured when the audio block is disabled. This
bit has no meaning in AC’97 audio protocol since AC’97 data are always transferred with the MSB
first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred
with LSB first.
0: Data are transferred with MSB first
1: Data are transferred with LSB first

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Bits 7:5 DS[2:0]: Data size
These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are
selected (bit PRTCFG[1:0]), because the frame and the data size are fixed in such case. When the
companding mode is selected through COMP[1:0] bits, DS[1:0] are ignored since the data size is
fixed to 8 bits by the algorithm.
These bits must be configured when the audio block is disabled.
000: Reserved
001: Reserved
010: 8 bits
011: 10 bits
100: 16 bits
101: 20 bits
110: 24 bits
111: 32 bits
Bit 4 Reserved, must be kept at reset value.
Bits 3:2 PRTCFG[1:0]: Protocol configuration
These bits are set and cleared by software. These bits have to be configured when the audio block is
disabled.
00: Free protocol. Free protocol enables to use the powerful config uration of the audio block to
address a specific audio protocol (such as I2S, LSB/MSB justified, TDM, PCM/DSP...) by setting
most of the configuration register bits as well as frame configuration register.
01: SPDIF protocol
10: AC’97 protocol
11: Reserved
Bits 1:0 MODE[1:0]: SAIx audio block mode
These bits are set and cleared by software. They must be configured when SAIx audio block is
disabled.
00: Master transmitter
01: Master receiver
10: Slave transmitter
11: Slave receiver
Note: When the audio block is configured in SPDIF mode, the master transmitter mode is forced
(MODE[1:0] = 00). In Master transmitter mode, the audio block starts generating the FS and the
clocks immediately.

69.6.11

SAI configuration register 2 (SAI_BCR2)
Address offset: 0x28
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

COMP[1:0]

CPL

rw

rw

rw

MUTECNT[5:0]
rw

rw

rw

rw

rw

MUTE
VAL

MUTE

TRIS

F
FLUSH

rw

rw

rw

w

rw

Bits 31:16 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

FTH[2:0]
rw

rw

rw

RM0456

Serial audio interface (SAI)

Bits 15:14 COMP[1:0]: Companding mode.
These bits are set and cleared by software. The µ-Law and the A-Law log are a part of the CCITT
G.711 recommendation, the type of complement that is used depends on CPL bit.
The data expansion or data compression are determined by the state of bit MODE[0].
The data compression is applied if the audio block is configured as a transmitter.
The data expansion is automatically applied when the audio block is configured as a receiver.
Refer to Section : Companding mode for more details.
00: No companding algorithm
01: Reserved.
10: µ-Law algorithm
11: A-Law algorithm
Note: Companding mode is applicable only when Free protocol mode is selected.
Bit 13 CPL: Complement bit.
This bit is set and cleared by software.
It defines the type of complement to be used for companding mode
0: 1’s complement representation.
1: 2’s complement representation.
Note: This bit has effect only when the companding mode is µ-Law algorithm or A-Law algorithm.
Bits 12:7 MUTECNT[5:0]: Mute counter.
These bits are set and cleared by software. They are used only in reception mode.
The value set in these bits is compared to the number of consecutive mute frames detected in
reception. When the number of mute frames is equal to this value, the flag MUTEDET is set and an
interrupt is generated if bit MUTEDETIE is set.
Refer to Section : Mute mode for more details.
Bit 6 MUTEVAL: Mute value.
This bit is set and cleared by software. It is meaningful only when the audio block operates as a
transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set.
If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0,
whatever the value of MUTEVAL.
if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each
slot is the one sent during the previous frame.
Refer to Section : Mute mode for more details.
0: Bit value 0 is sent during the mute mode.
1: Last values are sent during the mute mode.
Note: This bit is meaningless and must not be used for SPDIF audio blocks.
Bit 5 MUTE: Mute.
This bit is set and cleared by software. It is meaningful only when the audio block operates as a
transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to
2, or equal to 0 if it is greater than 2.
Refer to Section : Mute mode for more details.
0: No mute mode.
1: Mute mode enabled.
Note: This bit is meaningless and must not be used for SPDIF audio blocks.

RM0456 Rev 6

<!-- pagebreak -->

