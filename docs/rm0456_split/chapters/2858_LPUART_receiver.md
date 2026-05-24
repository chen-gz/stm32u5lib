RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Figure 863. Slot size configuration with FBOFF = 0 in SAI_xSLOTR

Audio block is transmitter

Audio block is receiver

Slot size = data size

Slot size = data size

data size

slotx

data size

data size
slotx

data size

slotx

00..00

data size

slotx

16-bit
slotx

X..X

data size
16-bit

00..00

data size

slotx

data size

32-bit

XX..XX

32-bit
X: don’t care
MSv30033V1

It is possible to choose the position of the first data bit to transfer within the slots. This offset
is configured by FBOFF[4:0] bits in the SAI_xSLOTR register. 0 values are injected in
transmitter mode from the beginning of the slot until this offset position is reached. In
reception, the bit in the offset phase is ignored. This feature targets the LSB justified
protocol (if the offset is equal to the slot size minus the data size).
Figure 864. First bit offset

slotx

Audio block is transmitter

Audio block is receiver

Slot size = data size

Slot size = data size
slotx

data size

data size
data size

data size

FBOFF

FBOFF
data size

00
slotx

XX

00
slotx

16-bit

data size

00..00

XX

16-bit
FBOFF = SLOT SZ -DS

FBOFF = SLOT SZ -DS
slotx

data size

slotx

XX .. XX

data size

32-bit
X: don’t care

32-bit

MS30034V1

It is mandatory to respect the following conditions to avoid bad SAI behavior:
•

FBOFF ≤(SLOTSZ - DS)

•

DS ≤SLOTSZ

•

NBSLOT x SLOTSZ ≤FRL (frame length)

The number of slots must be even when bit FSDEF in the SAI_xFRCR register is set.
In AC’97 and SPDIF protocol (bit PRTCFG[1:0] = 10 or PRTCFG[1:0] = 01), the slot size is
automatically set as defined in Section 69.4.11: AC’97 link controller.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

69.4.8

RM0456

SAI clock generator
Each audio block has its own clock generator. The clock generator builds the master clock
(MCLK_x) and bit clock (SCK_x) signals from the sai_x_ker_ck. The sai_x_ker_ck clock is
delivered by the clock controller of the product (RCC).

Generation of the master clock (MCLK_x)
The clock generator provides the master clock (MCLK_x) when the audio block is defined as
Master or Slave. The master clock is generated as soon as the MCKEN bit is set to 1 even if
the SAIEN bit for the corresponding block is set to 0. This feature can be useful if the
MCLK_x clock is used as system clock for an external audio device, since it enables the
generation of the MCLK_x before activating the audio stream.
To generate a master clock on MCLK_x output before transferring the audio samples, the
user application has to follow the sequence below:
1.

Check that SAIEN = 0.

2.

Program the MCKDIV[5:0] divider to the required value.

3.

Set the MCKEN bit to 1.

4.

Later, the application can configure other parts of the SAI, and sets the SAIEN bit to 1
to start the transfer of audio samples.

To avoid disturbances on the clock generated on MCLK_x output, the following operations
are not recommended:
•

Changing MCKDIV when MCKEN = 1

•

Setting MCKEN to 0 if the SAIEN = 1

The SAI makes sure that there are no spurs on MCLK_x output when the MCLK_x is
switched ON and OFF via the MCKEN bit (with SAIEN = 0).
Table 708 shows MCLK_x activation conditions.
Table 708. MCLK_x activation conditions
MCLKEN
0
1
0
1
X

Note:

<!-- pagebreak -->

NODIV

SAIEN for block x

X

0

MCLK_x
Disabled
Enabled
Disabled

1

1

0

Enabled
Enabled

MCLK_x can also be generated in AC’97 mode, when MCLKEN is set to 1.

RM0456 Rev 6

RM0456

Serial audio interface (SAI)

Generation of the bit clock (SCK_x)
The clock generator provides the bit clock (SCK_x) when the audio block is defined as
Master. The frame synchronization (FS_x) is also derived from the signals provided by the
clock generator.
In Slave mode, the value of NODIV and OSR fields are ignored, and the SCK_x clock is not
generated.
The bit clock strobing edge of SCK_x can be configured through the CKSTR fields, which is
functional both in master and slave mode.
Figure 865 illustrates the architecture of the audio block clock generator.
Figure 865. Audio block clock generator overview

SAI clock generator x
MCKDIV[5:0]
sai_x_ker_
ck

NODIV
SAIEN for block x
MCKEN

Clock divider

FRL[7:0]

OSR
÷2

1
0

÷

256
FRL+1

MCLK_x
NODIV

[0]

0
SCK_x

1
Audio Block x

÷ (FRL+1)

FS_x

FRL[7:0]
[0]: FRL+1 must be a power of 2 when NOMCK = 0
MSv43706V3

The NODIV bit must be used to force the ratio between the master clock (MCLK_x) and the
frame synchronization (FS_x) frequency to 256 or 512.
•

If NODIV is set to 0, the frequency ratio between the frame synchronization and the
master clock is fixed to 512 or 256, according to OSR value, but the frame length must
be a power of 2. More details are given below.

•

If NODIV is set to 1, the application can adjust the frequency of the bit clock (SCK_x)
via MCKDIV. In addition, there is no restriction on the frame length value as long as the
frame length is bigger or equal to 8 (that is, FRL[7:0] > 6). The frame synchronization
frequency depends on MCKDIV and frame length (FRL[7:0]). In that case, the
frequency of the MCLK_x is equal to the SCK_x.

The NODIV, MCKEN, SAIEN, OVR, CKSTR, and MCKDIV[5:0] bits belong to the SAI_xCR1
register, while FRL[7:0] belongs to SAI_xFRCR.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Clock generator programming when NODIV = 0
In that case, the MCLK_x frequency is:
•

FMCLK_x = 256 x FFS_x if OSR = 0

•

FMCLK_x = 512 x FFS_x if OSR = 1

When MCKDIV is different from 0, the MCLK_x frequency is given by the formula below:
F

F sai_x_ker_ck
MCLK_x = ---------------------------------MCKDIV

The frame synchronization frequency is given by:
F

F sai_x_ker_ck
FS_x = ----------------------------------------------------------------------------MCKDIV × ( OSR + 1 ) × 256

The bit clock frequency (SCK_x) is given by the following formula:
F sai_x_ker_ck × ( FRL + 1 )
F SCK_x = ----------------------------------------------------------------------------MCKDIV × ( OSR + 1 ) × 256
Note:

When NODIV is equal to 0, (FRL+1) must be a power of two. In addition, (FRL+1) must
range between 8 and 256. (FRL +1) represents the number of bit clock in the audio frame.
When the MCKDIV division ratio is odd, the MCLK duty cycle is not 50%. The bit clock
signal (SCK_x) can also have a duty cycle different from 50% if MCKDIV is odd, if OSR is
equal to 0, and if (FRL+1) = 2 8.
It is recommended, to program MCKDIV to an even value or to large values (higher than
10).
Note that MCKDIV = 0 gives the same result as MCKDIV = 1.
Clock generator programming when NODIV = 1
When MCKDIV is different from 0, the frequency of the bit clock (SCK_x) is given in the
formula below:
F sai_x_ker_ck
F SCK_x = F MCLK_x = ---------------------------------MCKDIV
The frequency of the frame synchronization (FS_x) in given by the following formula:
F sai_x_ker_ck
F FS_x = ---------------------------------------------------------( FRL + 1 ) × MCKDIV

Note:

When NODIV is set to 1, (FRL+1) can take any values from 8 to 256.
MCKDIV = 0 gives the same result as MCKDIV = 1.

<!-- pagebreak -->

