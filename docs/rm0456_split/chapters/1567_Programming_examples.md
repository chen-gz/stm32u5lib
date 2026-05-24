RM0456 Rev 6

RM0456

Multi-function digital filter (MDF)
Configurations #7, #8, #10 give signal-to-noise ratio around 120 dB, with an ideal
microphone model, using a sinus signal of 997 Hz.

#6

3.072

#7
#8

4.096

#9
#10

6.144

#11
#12

7.680

32 0x2B (- 14.5 dB)

1

2

5

1

2

1

0

0

5

16 0x01 (+ 3.5 dB)

0

0

6

5

8

2

2

5

12 0x06 (+ 18.1 dB)

1

2

5

24 0x2C (- 12 dB)

1

2

5

32 0x27 (- 26.6 dB)

3

2

5

16 0x02 (+ 6.0 dB)

2

2

5

24 0x2C (- 12 dB)

1

2

5

16 0x01 (+ 3.5 dB)

1

2

5

20 0x2E (- 6.0 dB)

SCALE

16 0x01 (+ 3.5 dB)

1.024

2.048

0x0B (+ 33.6 dB)
0

0

0

0

0

x

0

0

0

=>

3.072

64

-

32

-

64

32

64

64

1.024

16

32

32

0.512

8

48
96

64
64

FPCM (kHz)

5

FMDF_CCKx (MHz)

2

x

mdf_proc_ck
(MHz)

1

1

HPFBYP

64 0x2D (- 8.5 dB)

FRS (kHz)

#5

4

-

Total dec. ratio

2.048

2

RSFLTD

#4

1

RSFLTBYP

#3

MCICD + 1

1.024

CIC order (1)

#2

CCKDIV + 1

#1

PROCDIV + 1

Configuration

mdf_ker_ck (MHz)

Table 385. Examples of MDF settings for microphone capture

8
0.512

16
8

0.768

16

1.536

4.096

128

2.048

64

3.072

96

6.144

64

192

3.072

48

7.680

80

192

3.840

48

64

2.048

16

1.024

16

1.536

1. CICMOD = 100 for CIC order equal to 4. CICMOD = 101 for CIC order equal to 5.

39.7.2

Programming examples
This example describes how to capture sound from four microphones, assuming that each
microphone pair shares the same data line. The MDF_SD0 and MDF_SD2 data lines are
used. The microphone clock is provided by the MDF via MDF_CCK0 pin.
Table 386. Programming sequence
Operations

Comments

Adjust the proper kernel clock frequency via
the RCC block

Assuming that the RCC is programmed to provide a kernel clock
(mdf_ker_ck) of 12.288 MHz

Select the proper MDF kernel clock source via
Refer to the RCC of the product.
the RCC block
Enable the MDF clocks via the RCC block

Refer to the RCC of the product.

Reset the MDF via the RCC block

Refer to the RCC of the product.

AFMUX programming

Program the AFMUX to select MDF_SD0, MDF_SD2 and
MDF_CCK0 function.

RM0456 Rev 6

<!-- pagebreak -->

1599

Multi-function digital filter (MDF)

RM0456

Table 386. Programming sequence (continued)
Operations

Comments

Enable MDF processing clock:
MDF_CKGCR = 0x0103 0023

PROCDIV = 1 (division by 2): mdf_proc_ck frequency is
6.144 MHz.
CCKDIV = 3 (division by 4): MDF_CCK0 clock frequency is
1.536 MHz.
The MDF_CCK0 pad is set in output and generates a clock so that
the microphones can exit from low-power mode.

Serial interfaces configuration:
MDF_SITF0CR = 0x0000 1F01
MDF_SITF2CR = 0x0000 1F01

SCKSRC = 0 to select MDF_CCK0 as serial clock.
SIFTMOD = 0 to select LF_MASTER mode.
Clock absence feature is not working in this mode.
The serial interfaces are enabled.

Bitstream matrix configuration:
MDF_BSMX0CR = 0x0000 0000
MDF_BSMX1CR = 0x0000 0001
MDF_BSMX2CR = 0x0000 0004
MDF_BSMX3CR = 0x0000 0005

DFLT0 filter takes the bitstream of SITF0, sampled on rising edge.
DFLT1 filter takes the bitstream of SITF0, sampled on falling edge.
DFLT2 filter takes the bitstream of SITF2, sampled on rising edge.
DFLT3 filter takes the bitstream of SITF2, sampled on falling edge.

Filters configuration (CIC):
MDF_DFLT0CICR = 0x02C0 1750
MDF_DFLT1CICR = 0x02C0 1750
MDF_DFLT2CICR = 0x02C0 1750
MDF_DFLT3CICR = 0x02C0 1750

SCALE = 0x2C (- 12 dB) to avoid any saturation
MCICD = 0x17 (decimation by 24)
CICMOD = 5 to select a Sinc5
DATSCR = 0 to select data coming from BSMX

Filters configuration (RSFLT and HPF):
MDF_DFLT0RSFR = 0x0000 0100
MDF_DFLT1RSFR = 0x0000 0100
MDF_DFLT2RSFR = 0x0000 0100
MDF_DFLT3RSFR = 0x0000 0100

HPFC = 1: cut-off frequency of 16 kHz * 0.00125 = 20 Hz
HPFBYP = 0: HPF not bypassed
RSFLTD = 0: RSFLT decimates by 4
RSFLTBYP = 0: RSFLT is not bypassed

Filters configuration (INT):
MDF_DFLT0INTR = 0x0000 0000
MDF_DFLT1INTR = 0x0000 0000
MDF_DFLT2INTR = 0x0000 0000
MDF_DFLT3INTR = 0x0000 0000

INTVAL = 0: INT filter not used
Other parameter is not significant.

Micro delay adjust:
MDF_DLY0CR = 0x0000 0005
MDF_DLY0CR = 0x0000 0012
MDF_DLY0CR = 0x0000 0023
MDF_DLY0CR = 0x0000 0000

Initial micro-delay for each microphone, values just given as
example

Offset error correction:
MDF_OEC0CR = 0x0000 0000
MDF_OEC1CR = 0x0000 0000
MDF_OEC2CR = 0x0000 0000
MDF_OEC3CR = 0x0000 0000

No correction. DC offset is removed by HPF.

<!-- pagebreak -->

