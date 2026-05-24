1599

Multi-function digital filter (MDF)

RM0456

Bits 10:4 INTVAL[6:0]: Integration value selection
This bitfield is set and cleared by software. It is used to select the integration value.
0: The integration value is 1, meaning bypass mode (default after reset).
1: The integration value is 2.
2: The integration value is 3.
...
127: The integration value is 128.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 3:2 Reserved, must be kept at reset value.
Bits 1:0 INTDIV[1:0]: Integrator output division
This bitfield is set and cleared by software. It is used to rescale the signal at the integrator
output in order keep the data width lower than 24 bits.
00: The integrator data outputs are divided by 128 (default value).
01: The integrator data outputs are divided by 32.
10: The integrator data outputs are divided by 4.
11: The integrator data outputs are not divided.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

39.8.9

MDF out-of limit detector control register x (MDF_OLDxCR)
Address offset: 0x098 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used to configure the OLDx.The number of registers is equal to the amount
of filters. Refer to Section 39.3 for details.

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

OLDAC
TIVE

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

r
15

14

13

12

11

10

9

8

Res.

Res.

ACICN[1:0]

Res.

Res.

Res.

Res.

rw

rw

7

6

21

20

rw

18

17

ACICD[4:0]
rw

rw

5

4

BKOLD[3:0]
rw

19

rw

rw

rw

16
Res.

rw

3

2

Res.

Res.

rw
1

0

THINB OLDEN
rw

rw

Bit 31 OLDACTIVE: OLDx active flag
This bit is set and cleared by hardware. It is used to check if the OLDx is effectively enabled
(active) or not. The protected fields and registers of this function can only be updated when
the OLDACTIVE is set to 0 (refer to Section 39.4.15 for details).
The delay between a transition on OLDEN and a transition on OLDACTIVE is two periods of
AHB clock and two periods of mdf_proc_ck.
0: OLDx not active and can be configured if needed
1: OLDx active and protected fields cannot be configured
Bits 30:22 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

Bits 21:17 ACICD[4:0]: OLDx CIC decimation ratio selection
This bitfield is set and cleared by software. It is used to select the decimation ratio of the
ACIC. It is only taken into account by the MDF when CICMOD[2:0] = 0xx. The decimation
ratio is given by (ACICD + 1).
0: Decimation ratio is 1.
1: Decimation ratio is 2.
2: Decimation ratio is 3.
3: Decimation ratio is 4.
...
31: Decimation ratio is 32.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 16:14 Reserved, must be kept at reset value.
Bits 13:12 ACICN[1:0]: OLDx CIC order selection
This bitfield is set and cleared by software. It is used to select the ACIC type and order. It is
only taken into account by the MDF when CICMOD[2:0] = 0xx.
00: FastSinc filter type
01: Sinc1 filter type
10: Sinc2 filter type
11: Sinc3 filter type
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 11:8 Reserved, must be kept at reset value.
Bits 7:4 BKOLD[3:0]: Break signal assignment for out-of limit detector
This bitfield is set and cleared by software.
BKOLD[i] = 0: Break signal (mdf_break[i]) not assigned to threshold event
BKOLD[i] = 1: Break signal (mdf_break[i]) assigned to threshold event
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 3:2 Reserved, must be kept at reset value.
Bit 1 THINB: Threshold In band
This bit is set and cleared by software.
0: The OLDx generates an event if the signal is lower than OLDTHL or higher than OLDTHH
(default value).
1: The OLDx generates an event if the signal is lower than OLDTHH and higher than
OLDTHL.
Note: This bit can be write-protected (refer to Section 39.4.15 for details).
Bit 0 OLDEN: OLDx enable
This bit is set and cleared by software.
0: OLDx disabled (default value)
1: OLDx enabled, including the ACIC filter working in continuous mode

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

39.8.10

RM0456

MDF OLDx low threshold register x (MDF_OLDxTHLR)
Address offset: 0x09C + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used for the adjustment of the out-off-limit low threshold. The number of
registers is equal to the amount of filters. Refer to Section 39.3 for details.

31

30

29

28

27

26

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

OLDTHL[25:16]
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

OLDTHL[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 OLDTHL[25:0]: OLD low threshold value
This bitfield is set and cleared by software. OLDTHL represents a 26-bit signed value. The
real threshold compared to the signal provided by the filter is OLDTHL.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

39.8.11

MDF OLDx high threshold register x (MDF_OLDxTHHR)
Address offset: 0x0A0 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used for the adjustment of the OLDx high threshold. The number of registers
is equal to the amount of filters. Refer to Section 39.3 for details.

31

30

29

28

27

26

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

OLDTHH[25:16]
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

OLDTHH[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:26 Reserved, must be kept at reset value.
Bits 25:0 OLDTHH[25:0]: OLDx high threshold value
This bitfield is set and cleared by software. OLDTHH represents a 26-bit signed value. The
real threshold compared to the signal provided by the filter is OLDTHH.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

39.8.12

MDF delay control register x (MDF_DLYxCR)
Address offset: 0x0A4 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used for the adjustment stream delays. The number of registers is equal to
the amount of filters. Refer to Section 39.3 for details.

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

SKPBF

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

r
15

14

13

12

11

10

9

8

7

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

SKPDLY[6:0]
rw

rw

rw

rw

Bit 31 SKPBF: Skip busy flag
This bit is set and cleared by hardware. It is used to control if the delay sequence is
completed.
0: MDF ready to accept a new value into SKPDLY[6:0]
1: Last valid SKPDLY[6:0] still under processing
Bits 30:7 Reserved, must be kept at reset value.
Bits 6:0 SKPDLY[6:0]: Delay to apply to a bitstream
This bitfield is set and cleared by software. It defines the number of input samples that are
skipped. Skipping is applied immediately after writing to this bitfield, if SKPBF = 0 and the
corresponding DFLTEN = 1. If SKPBF = 1, the value written into the register is ignored by the
delay state machine.
0: No input sample skipped
1: 1 input sample skipped
...
127: 127 input samples skipped

39.8.13

MDF short circuit detector control register x (MDF_SCDxCR)
Address offset: 0x0A8 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used for the adjustment stream delays. The number of registers is equal to
the amount of filters. Refer to Section 39.3 for details.

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

SCDAC
TIVE

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

Res.

Res.

Res.

Res.

r
15

SCDT[3:0]
rw

rw

rw

rw

BKSCD[3:0]
rw

RM0456 Rev 6

rw

rw

rw

19

18

17

16

SCDT[7:4]
rw

rw

rw

3

2

1

rw
0

Res.

Res.

Res.

SCDEN
rw

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Bit 31 SCDACTIVE: SCDx active flag
This bit is set and cleared by hardware. It is used to check if the SCDx is effectively enabled
(active) or not. The protected fields of this function can only be updated when the
SCDACTIVE is set to 0 (refer to Section 39.4.15 for details).
The delay between a transition on SCDEN and a transition on SCDACTIVE is two periods of
AHB clock and two periods of mdf_proc_ck.
0: SCDx not active and can be configured if needed
1: SCDx active and protected fields cannot be configured
Bits 30:20 Reserved, must be kept at reset value.
Bits 19:12 SCDT[7:0]: SCDx threshold
This bitfield is set and cleared by software. These bits are written by software to define the
threshold counter for SCDx. If this value is reached, a short-circuit detector event occurs on a
given input stream.
0: 2 consecutive 1's or 0's generate an event.
1: 2 consecutive 1's or 0's generate an event.
2: 3 consecutive 1's or 0's generate an event.
...
255: 256 consecutive 1's or 0's generate an event.
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 11:8 Reserved, must be kept at reset value.
Bits 7:4 BKSCD[3:0]: Break signal assignment for short circuit detector
This bitfield is set and cleared by software.
BKSCD[i] = 0: Break signal (mdf_break[i]) not assigned to this SCD event
BKSCD[i] = 1: Break signal (mdf_break[i]) assigned to this SCD event
Note: This bitfield can be write-protected (refer to Section 39.4.15 for details).
Bits 3:1 Reserved, must be kept at reset value.
Bit 0 SCDEN: SCDx enable
This bit is set and cleared by software.
0: SCDx disabled
1: SCDx enabled,

39.8.14

MDF DFLT0 interrupt enable register 0 (MDF_DFLT0IER)
Address offset: 0x0AC
Reset value: 0x0000 0000
This register is used for allowing or not the events to generate an interrupt.

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

SATIE

SCDIE

SSOVR
IE

Res.

Res.

OLDIE

Res.

SSDRI
E

DOVRI
E

FTHIE

rw

rw

rw

rw

rw

rw

RFOVR CKABI
IE
E
rw

rw

Bits 31:12 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

rw

RM0456

Multi-function digital filter (MDF)

Bit 11 RFOVRIE: Reshape filter overrun interrupt enable
This bit is set and cleared by software.
0: Reshape filter overrun interrupt disabled
1: Reshape filter overrun interrupt enabled
Bit 10 CKABIE: Clock absence detection interrupt enable
This bit is set and cleared by software.
0: Clock absence interrupt disabled
1: Clock absence interrupt enabled
Bit 9 SATIE: Saturation detection interrupt enable
This bit is set and cleared by software.
0: Saturation interrupt disabled
1: Saturation interrupt enabled
Bit 8 SCDIE: SCD0 interrupt enable
This bit is set and cleared by software.
0: SCD0 interrupt disabled
1: SCD0 interrupt enabled
Bit 7 SSOVRIE: Snapshot overrun interrupt enable
This bit is set and cleared by software.
0: Snapshot overrun interrupt disabled
1: Snapshot overrun interrupt enabled
Bits 6:5 Reserved, must be kept at reset value.
Bit 4 OLDIE: OLD0 interrupt enable
This bit is set and cleared by software.
0: OLD0 event interrupt disabled
1: OLD0 event interrupt enabled
Bit 3 Reserved, must be kept at reset value.
Bit 2 SSDRIE: Snapshot data ready interrupt enable
This bit is set and cleared by software.
0: Snapshot data ready interrupt disabled
1: Snapshot data ready interrupt enabled
Bit 1 DOVRIE: Data overflow interrupt enable
This bit is set and cleared by software.
0: Data overflow interrupt disabled
1: Data overflow interrupt enabled
Bit 0 FTHIE: RXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: RXFIFO threshold interrupt disabled
1: RXFIFO threshold interrupt enabled

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

39.8.15

RM0456

MDF DFLTx interrupt enable register x (MDF_DFLTxIER)
Address offset: 0x12C + 0x80 * (x - 1), (x = 1 to 5)
Reset value: 0x0000 0000
This register is used for allowing or not, the events to generate an interrupt. The number of
registers is equal to the amount of filters. Refer to Section 39.3 for details.

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

SSDRI
E

DOVRI
E

FTHIE

rw

rw

rw

Res.

Res.

Res.

Res.

RFOVR CKABI
IE
E
rw

rw

SATIE
rw

SSOVR
SCDIE
IE
rw

Res.

rw

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 RFOVRIE: Reshape filter overrun interrupt enable
This bit is set and cleared by software.
0: Reshape filter overrun interrupt disabled
1: Reshape filter overrun interrupt enabled
Bit 10 CKABIE: Clock absence detection interrupt enable
This bit is set and cleared by software.
0: Clock absence interrupt disabled
1: Clock absence interrupt enabled
Bit 9 SATIE: Saturation detection interrupt enable
This bit is set and cleared by software.
0: Saturation interrupt disabled
1: Saturation interrupt enabled
Bit 8 SCDIE: SCDx interrupt enable
This bit is set and cleared by software.
0: SCDx interrupt disabled
1: SCDx interrupt enabled
Bit 7 SSOVRIE: Snapshot overrun interrupt enable
This bit is set and cleared by software.
0: Snapshot overrun interrupt disabled
1: Snapshot overrun interrupt enabled
Bits 6:5 Reserved, must be kept at reset value.
Bit 4 OLDIE: OLDx interrupt enable
This bit is set and cleared by software.
0: OLDx event interrupt disabled
1: OLDx event interrupt enabled
Bit 3 Reserved, must be kept at reset value.
Bit 2 SSDRIE: Snapshot data ready interrupt enable
This bit is set and cleared by software.
0: Snapshot data ready interrupt disabled
1: Snapshot data ready interrupt enabled

<!-- pagebreak -->

RM0456 Rev 6

Res.

OLDIE
rw

RM0456

Multi-function digital filter (MDF)

Bit 1 DOVRIE: Data overflow interrupt enable
This bit is set and cleared by software.
0: Data overflow interrupt disabled
1: Data overflow interrupt enabled
Bit 0 FTHIE: RXFIFO threshold interrupt enable
This bit is set and cleared by software.
0: RXFIFO threshold interrupt disabled
1: RXFIFO threshold interrupt enabled

39.8.16

MDF DFLT0 interrupt status register 0 (MDF_DFLT0ISR)
Address offset: 0x0B0
Reset value: 0x0000 0000
This register contains the status flags for each digital filter path.

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

RFOVR
CKABF
F

SATF

SCDF

SSOVR
F

THHF

THLF

OLDF

rc_w1

rc_w1

rc_w1

rc_w1

r

r

rc_w1

rc_w1

RXNEF SSDRF DOVRF
r

rc_w1

rc_w1

FTHF
r

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 RFOVRF: Reshape filter overrun detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no reshape filter overrun is detected. Write 0 has no effect.
1: Read 1 means that reshape filter overrun is detected. Write 1 clears this flag.
Bit 10 CKABF: Clock absence detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no clock absence is detected. Write 0 has no effect.
1: Read 1 means that a clock absence is detected. Write 1 clears this flag.
Bit 9 SATF: Saturation detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no saturation is detected. Write 0 has no effect.
1: Read 1 means that a saturation is detected. Write 1 clears this flag.
Bit 8 SCDF: Short-circuit detector flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no SCD0 event is detected. Write 0 has no effect.
1: Read 1 means that a SCD0 event is detected. Write 1 clears this flag.
Bit 7 SSOVRF: Snapshot overrun flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no snapshot overrun event is detected. Write 0 has no effect.
1: Read 1 means that a snapshot overrun event is detected. Write 1 clears this flag.

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Bit 6 THHF: High-threshold status flag
This bit is set by hardware and cleared by software by writing this bit to 1. It indicates the
status of the high-threshold comparator when the last OLD0 event occurred. This bit gives
additional information on the conditions triggering the last OLD0 event. It can be cleared by
writing OLDF flag to 1.
0: The signal was lower than OLDTHH when the last OLD0 event occurred.
1: The signal was higher than OLDTHH when the last OLD0 event occurred.
Bit 5 THLF: Low-threshold status flag
This bit is set by hardware and cleared by software by writing this bit to 1. It indicates the
status of the low-threshold comparator when the last OLD0 event occurred. This bit gives
additional information on the conditions triggering the last OLD0 event. It can be cleared by
writing OLDF flag to 1.
0: The signal was higher than OLDTHL when the last OLD0 event occurred.
1: The signal was lower than OLDTHL when the last OLD0 event occurred.
Bit 4 OLDF: OLD0 flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no OLD0 event is detected. Write 0 has no effect.
1: Read 1 means that an OLD0 event is detected Write 1 clears THHF, THLF and OLDF.
Bit 3 RXNEF: RXFIFO not-empty flag
this bit is set and cleared by hardware according to the RXFIFO level.
0: RXFIFO empty
1: RXFIFO not empty
Bit 2 SSDRF: Snapshot data ready flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no data is available. Write 0 has no effect.
1: Read 1 means that a new data is available. Write 1 clears this flag.
Bit 1 DOVRF: Data overflow flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no overflow is detected. Write 0 has no effect.
1: Read 1 means that an overflow is detected; Write 1 clears this flag.
Bit 0 FTHF: RXFIFO threshold flag
This bit is set by hardware and cleared by hardware when the RXFIFO level is lower than the
threshold.
0: RXFIFO threshold not reached
1: RXFIFO threshold reached

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

39.8.17

MDF DFLTx interrupt status register x (MDF_DFLTxISR)
Address offset: 0x130 + 0x80 * (x - 1), (x = 1 to 5)
Reset value: 0x0000 0000
This register contains the status flags for each digital filter path. The number of registers is
equal to the amount of filters. Refer to Section 39.3 for details.

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

THHF

THLF

OLDF

r

r

rc_w1

Res.

Res.

Res.

Res.

RFOVR
CKABF
F

SATF

SCDF

SSOVR
F

rc_w1

rc_w1

rc_w1

rc_w1

rc_w1

RXNEF SSDRF DOVRF
r

rc_w1

rc_w1

FTHF
r

Bits 31:12 Reserved, must be kept at reset value.
Bit 11 RFOVRF: Reshape filter overrun detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no reshape filter overrun is detected. Write 0 has no effect.
1: Read 1 means that reshape filter overrun is detected. Write 1 clears this flag.
Bit 10 CKABF: Clock absence detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no clock absence is detected. Write 0 has no effect.
1: Read 1 means that a clock absence is detected. Write 1 clears this flag.
Bit 9 SATF: Saturation detection flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no saturation is detected. Write 0 has no effect.
1: Read 1 means that a saturation is detected. Write 1 clears this flag.
Bit 8 SCDF: Short-circuit detector flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no SCD event is detected. Write 0 has no effect.
1: Read 1 means that a SCD event is detected. Write 1 clears this flag.
Bit 7 SSOVRF: Snapshot overrun flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no snapshot overrun event is detected. Write 0 has no effect.
1: Read 1 means that a snapshot overrun event is detected. Write 1 clears this flag.
Bit 6 THHF: High-threshold status flag
This bit is set by hardware and cleared by software by writing this bit to 1. It indicates the
status of the high-threshold comparator when the last OLDx event occurred. This bit gives
additional information on the conditions triggering the last OLDx event. It can be cleared by
writing OLDF flag to 1.
0: The signal was lower than OLDTHH when the last OLDx event occurred.
1: The signal was higher than OLDTHH when the last OLDx event occurred.

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Bit 5 THLF: Low-threshold status flag
This bit is set by hardware and cleared by software by writing this bit to 1. It indicates the
status of the low-threshold comparator when the last OLDx event occurred. This bit gives
additional information on the conditions triggering the last OLDx event. It can be cleared by
writing OLDF flag to 1.
0: The signal was higher than OLDTHL when the last OLDx event occurred.
1: The signal was lower than OLDTHL when the last OLDx event occurred.
Bit 4 OLDF: OLDx flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no OLDx event is detected. Write 0 has no effect.
1: Read 1 means that an OLDx event is detected Write 1 clears THHF, THLF and OLDF.
Bit 3 RXNEF: RXFIFO not-empty flag
this bit is set and cleared by hardware according to the RXFIFO level.
0: RXFIFO empty
1: RXFIFO not empty
Bit 2 SSDRF: Snapshot data ready flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no data is available. Write 0 has no effect.
1: Read 1 means that a new data is available. Write 1 clears this flag.
Bit 1 DOVRF: Data overflow flag
This bit is set by hardware and cleared by software by writing this bit to 1.
0: Read 0 means that no overflow is detected. Write 0 has no effect.
1: Read 1 means that an overflow is detected; Write 1 clears this flag.
Bit 0 FTHF: RXFIFO threshold flag
This bit is set by hardware and cleared by hardware when the RXFIFO level is lower than
the threshold.
0: RXFIFO threshold not reached
1: RXFIFO threshold reached

39.8.18

MDF offset error compensation control register x (MDF_OECxCR)
Address offset: 0x0B4 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register contains the offset compensation value. The number of registers is equal to the
amount of filters. Refer to Section 39.3 for details.

31

30

29

28

27

26

Res.

Res.

Res.

Res.

Res.

Res.

25

24

23

22

21

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

rw

rw

rw

rw

rw

rw

rw

rw

rw

20

19

18

17

16

rw

rw

rw

rw

rw

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

OFFSET[25:16]

OFFSET[15:0]
rw

rw

Bits 31:26 Reserved, must be kept at reset value.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)

Bits 25:0 OFFSET[25:0]: Offset error compensation
This bitfield is set and cleared by software. If the application attempts to write a new offset
value while the previous one is not yet applied, this new offset value is ignored. Reading back
this bitfield informs the application on the current offset value.
This bitfield represents the value to be subtracted to the signal before going to the SCALE.

39.8.19

MDF snapshot data register x (MDF_SNPSxDR)
Address offset: 0x0EC + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used to read the data processed by each digital filter in snapshot mode.
The number of registers is equal to the amount of filters. Refer to Section 39.3 for details.

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

SDR[15:0]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

r

r

r

r

r

r

r

EXTSDR[6:0]

MCICDC[8:0]

r

r

Bits 31:16 SDR[15:0]: Contains the 16 MSB of the last valid data processed by the digital filter.
Bits 15:9 EXTSDR[6:0]: Extended data size
If SNPSFMT = 0, this bitfield contains the bits 7 to 1 of the last valid data processed by the
digital filter.
If SNPSFMT = 1, this bitfield contains the INT accumulator counter value when the last
trigger event occurs (INT_CNT).
Bits 8:0 MCICDC[8:0]: Contains the MCIC decimation counter value when the last trigger event occurs
(MCIC_CNT)

39.8.20

MDF digital filter data register x (MDF_DFLTxDR)
Address offset: 0x0F0 + 0x80 * x, (x = 0 to 5)
Reset value: 0x0000 0000
This register is used to read the data processed by each digital filter. The number of
registers is equal to the amount of filters. Refer to Section 39.3 for details.

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

DR[23:8]
r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

r

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

r

r

r

r

r

r

r

DR[7:0]
r

Bits 31:8 DR[23:0]: Data processed by digital filter
Bits 7:0 Reserved, must be kept at reset value.

RM0456 Rev 6

<!-- pagebreak -->

