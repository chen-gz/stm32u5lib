RM0456 Rev 6

16

r

RM0456

Serial audio interface (SAI)

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

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

69.6.16

RM0456

SAI clear flag register (SAI_BCLRFR)
Address offset: 0x3C
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

w

CMUTE COVRUD
DET
R
w

w

Bits 31:7 Reserved, must be kept at reset value.
Bit 6 CLFSDET: Clear late frame synchronization detection flag.
This bit is write only.
Programming this bit to 1 clears the LFSDET flag in the SAI_xSR register.
This bit is not used in AC’97or SPDIF mode
Reading this bit always returns the value 0.
Bit 5 CAFSDET: Clear anticipated frame synchronization detection flag.
This bit is write only.
Programming this bit to 1 clears the AFSDET flag in the SAI_xSR register.
It is not used in AC’97or SPDIF mode.
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

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

69.6.17

SAI data register (SAI_BDR)
Address offset: 0x40
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

69.6.18

SAI PDM control register (SAI_PDMCR)
Address offset: 0x44
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

PDMEN

CKEN4 CKEN3 CKEN2 CKEN1
rw

rw

rw

rw

MICNBR[1:0]
rw

rw

rw

Bits 31:16 Reserved, must be kept at reset value.
Bits 15:12 Reserved, must be kept at reset value.
Bit 11 CKEN4: Clock enable of bitstream clock number 4
This bit is set and cleared by software.
0: SAI_CK4 clock disabled
1: SAI_CK4 clock enabled
Note: It is not recommended to configure this bit when PDMEN = 1.
SAI_CK4 might not be available for all SAI instances. Refer to Section 69.3: SAI
implementation for details.
Bit 10 CKEN3: Clock enable of bitstream clock number 3
This bit is set and cleared by software.
0: SAI_CK3 clock disabled
1: SAI_CK3 clock enabled
Note: It is not recommended to configure this bit when PDMEN = 1.
SAI_CK3 might not be available for all SAI instances. Refer to Section 69.3: SAI
implementation for details.

RM0456 Rev 6

<!-- pagebreak -->

