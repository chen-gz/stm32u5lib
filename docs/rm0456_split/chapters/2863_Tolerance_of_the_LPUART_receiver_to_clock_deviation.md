3020

Serial audio interface (SAI)

RM0456

• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO half full
(FTH[2:0] set to 0b010), an interrupt is generated (FREQ bit set by hardware to 1 in the
SAI_xSR register) if less than half of the FIFO contains data (FLVL[2:0] bits in SAI_xSR
are less than 0b011). This interrupt (FREQ bit in the SAI_xSR register) is cleared by
hardware when at least half of the FIFO contains data (FLVL[2:0] bits in SAI_xSR are
higher than or equal to 0b011).
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO three
quarter (FTH[2:0] set to 0b011), an interrupt is generated (FREQ bit is set by hardware to
1 in the SAI_xSR register) if less than three quarters of the FIFO contains data (FLVL[2:0]
bits in SAI_xSR are less than 0b100). This interrupt (FREQ bit in the SAI_xSR register) is
cleared by hardware when at least three quarters of the FIFO contains data (FLVL[2:0]
bits in SAI_xSR are higher than or equal to 0b100).
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO full
(FTH[2:0] set to 0b100), an interrupt is generated (FREQ bit is set by hardware to 1 in the
SAI_xSR register) if the FIFO is not full (FLVL[2:0] bits in SAI_xSR are less than 0b101).
This interrupt (FREQ bit in the SAI_xSR register) is cleared by hardware when the FIFO
is full (FLVL[2:0] bits in SAI_xSR are equal to 0b101).

Interrupt generation in reception mode
The interrupt generation depends on the FIFO configuration in reception mode:
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO empty
(FTH[2:0] set to 0b000), an interrupt is generated (FREQ bit is set by hardware to 1 in the
SAI_xSR register) if at least one data is available in the SAI_xDR register (FLVL[2:0] bits
in SAI_xSR are higher than or equal to 0b001). This interrupt (FREQ bit in the SAI_xSR
register) is cleared by hardware when the FIFO becomes empty (FLVL[2:0] bits in the
SAI_xSR are equal to 0b000), that is, no data are stored in FIFO.
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO quarter
full (FTH[2:0] set to 0b001), an interrupt is generated (FREQ bit is set by hardware to 1 in
the SAI_xSR register) if at least one quarter of the FIFO data locations are available
(FLVL[2:0] bits in SAI_xSR are higher than or equal to 0b010). This interrupt (FREQ bit in
the SAI_xSR register) is cleared by hardware when less than a quarter of the FIFO data
locations become available (FLVL[2:0] bits in SAI_xSR are less than 0b010).
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO half full
(FTH[2:0] set to 0b010), an interrupt is generated (FREQ bit is set by hardware to 1 in the
SAI_xSR register) if at least half of the FIFO data locations are available (FLVL[2:0] bits
in SAI_xSR are higher than or equal to 0b011). This interrupt (FREQ bit in the SAI_xSR
register) is cleared by hardware when less than half of the FIFO data locations become
available (FLVL[2:0] bits in SAI_xSR are less than 0b011).
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO three
quarter full (FTH[2:0] set to 0b011), an interrupt is generated (FREQ bit is set by hardware
to 1 in the SAI_xSR register) if at least three quarters of the FIFO data locations are
available (FLVL[2:0] bits in SAI_xSR are higher than or equal to 0b100). This interrupt
(FREQ bit in the SAI_xSR register) is cleared by hardware when the FIFO has less than
three quarters of the FIFO data locations available (FLVL[2:0] bits in SAI_xSR is less than
0b100).
• When the FIFO threshold bits in the SAI_xCR2 register are configured as FIFO full
(FTH[2:0] set to 0b100), an interrupt is generated (FREQ bit is set by hardware to 1 in the
SAI_xSR register) if the FIFO is full (FLVL[2:0] bits in SAI_xSR are equal to 0b101). This

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
interrupt (FREQ bit in the SAI_xSR register) is cleared by hardware when the FIFO is not
full (FLVL[2:0] bits in SAI_xSR are less than 0b101).
Like interrupt generation, the SAI can use the DMA if the DMAEN bit in the SAI_xCR1
register is set. The FREQ bit assertion mechanism is the same as the interrupt generation
mechanism described above for FREQIE.
Each FIFO is an 8-word FIFO. Each read or write operation from/to the FIFO targets one
word FIFO location whatever the access size. Each FIFO word contains one audio slot.
FIFO pointers are incremented by one word after each access to the SAI_xDR register.
Data must be right-aligned when written in the SAI_xDR register.
Data received are right-aligned in the SAI_xDR register.
The FIFO pointers can be reinitialized when the SAI is disabled by configuring the FFLUSH
bit in the SAI_xCR2 register. If FFLUSH is set when the SAI is enabled, the data present in
the FIFO are lost automatically.

PDM interface
The PDM (pulse density modulation) interface is provided in order to support digital
microphones. Up to 4 digital microphone pairs can be connected in parallel. Depending on
product implementation, less microphones can be supported (refer to Section 69.3: SAI
implementation).
Figure 866 shows a typical connection of a digital microphone pair via a PDM interface.
Both microphones share the same bitstream clock and data line. Thanks to a configuration
pin (LR), a microphone can provide valid data on SAI_CK[m] rising edge while the other
provides valid data on SAI_CK[m] falling edge (m being the number of clock lines).
Figure 866. PDM typical connection and timing
Vcc
LR

SAI
SAI_A

TDM link
M Audio link

PDM_IF

69.4.10

SAI_Dn
MpL
SAI_CKm
GND
LR

MpR

SAI_Dn

MpL MpR MpL MpR MpL MpR MpL MpR MpL

SAI_CKm
MSv35467V7

1. n refers to the number of data lines and p to the number of microphone pairs.

The PDM function is intended to be used in conjunction with SAI_A subblock configured in
TDM master mode. It cannot be used with SAI_B subblock. The PDM interface uses the
timing signals provided by the serial audio interface of SAI_A and adapts them to generate a
bitstream clock (SAI_CK[m]).

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

The processing performed into the PDM interface is the following:
1.

The PDM interface builds the bitstream clock from the bit clock received from the serial
audio interface of SAI_A.

2.

The bitstream data received from the microphones (SAI_D[n]) are de-interleaved and
go through a 7-bit delay line to fine-tune the delay of each microphone with the
accuracy of the bitstream clock.

3.

The shift registers translate each serial bitstream into bytes.

4.

The last operation consists in shifting-out the resulting bytes to SAI_A through the data
line of the serial audio interface.

Figure 867 below shows the block diagram of the PDM interface, with a detailed view of a
de-interleaver.
Note:

The PDM interface does not embed the decimation filter required to build-up the PCM audio
samples from the bitstream. It is up to the application software to perform this operation.
Figure 867. Detailed PDM interface block diagram

LatchReg

To saia_sd_in

1 2 3 4 5 6 7 8

61626364

PDM_IF

8
8
8
8
8
8
8
8

De-Interleaver1

SAI_D1(2)

De-Interleaver2

SAI_D2(2)

De-Interleaver3

SAI_D3(2)

De-Interleaver4

SAI_D4(2)
SAI_CK1(2)

From saia_fs_out

SAI_CK2(2)

Control
Logic

pdm_ck

SAI_CK3(2)

From saia_sck_out

SAI_CK4(2)
MICNBR
PDMEN
SAI register interface

DLYM[4:1]L,
DLYM[4:1]R

CKEN[4:1]

DLYMpL
8 De-Interleaver n delay line
8

shift reg

SAI_D[n]

delay line

shift reg
DLYMpR

MSv61329V2

1. n refers to the number of data lines and p to the number of microphone pairs.
2. These signals might not be available in all SAI instances. Refer to Section 69.3: SAI implementation for
details.

The PDM interface can be enabled through the PDMEN bit in the SAI_PDMCR register.
However, the PDM interface must be enabled prior to enabling the SAI_A block.
To reduce the memory footprint, the user can select the number of microphones the
application needs. This can be done through the MICNBR[1:0] bits. It is possible to choose

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
between 2, 4, 6, or 8 microphones. For example, if the application is using 3 microphones,
the user has to select 4.

Enabling the PDM interface
To enable the PDM interface, follow the sequence below:

Note:

1.

Configure SAI_A in TDM master mode (see Table 710).

2.

Configure the PDM interface as follows:
a)

Define the number of digital microphones via MICNBR.

b)

Enable the bitstream clock needed in the application by setting the corresponding
bits on CKEN to 1.

3.

Enable the PDM interface, via the PDMEN bit.

4.

Enable the SAI_A.

Once the PDM interface and SAI_A are enabled, the first two frames received on SAI_ADR
are invalid and must be dropped.

Startup sequence
Figure 868 shows the startup sequence: Once the PDM interface is enabled, it waits for the
frame synchronization event prior to starting the acquisition of the microphone samples.
After 8 SAI_CK clock periods, a data byte coming from each microphone is available, and
transferred to the SAI, via the serial audio interface.
Figure 868. Start-up sequence

1

2

3

4

5

6

7

8

1

2

3

4

5 6

7

8

1

2

3

4

5

6

7

8

Pdm_ck
saia_clk_out
Don’t care

saia_sd_in

8 bits

8 bits

8 bits

8 bits

8 bits

8 bits

8 bits

8 bits

M1L-x

M1R-x

M2L-x

M2R-x

M1L-y

M1R-y

M2L-y

M2R-y

saia_fs_out
PDMEN
N

SAIEN
Wait for frame
sync.

N+1

Frame sync is detected, waiting
for receiving 8 bits from each
microphone

N+2

Transmission to SAI of the data received Transmission to SAI of the data received
on frame N, and acquisition of the next on frame N+1, and acquisition of the next
8 bits from each microphone.
8 bits from each microphone.
No re-sync with the frame sync
No re-sync with the frame sync
MSv35469V3

SAI_ADR data format
The arrangement of the data coming from the microphone into the SAI_ADR register
depends on the following parameters:
•

Number of microphones

•

Slot width selected

•

LSBFIRST bit

The slot width defines the number of significant bits into each word available into the
SAI_ADR.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

When a slot width of 32 bits is selected, each data available into the SAI_ADR contains 32
useful bits. This reduces the number of words stored into the memory. However, the
counterpart is that the software has to perform some operations to de-interleave the data of
each microphone.
On the other hand, when the slot width is set to 8 bits, each data available into the SAI_ADR
contains 8 useful bits. This increases the number of words stored in the memory. However, it
offers the advantage of avoiding extra processing since each word contains information
from one microphone.
SAI_ADR data format example
•

32-bit slot width (DS = 0b111 and SLOTSZ = 0). Refer to Figure 869.
For an 8 microphone configuration, two consecutive words read from the SAI_ADR
register contain a data byte from each microphone.
For a 4 microphones configuration, each word read from the SAI_ADR register
contains a data byte from each microphone.
Figure 869. SAI_ADR format in TDM mode, 32-bit slot width

8 Microphones configuration
M4R-8 M2R-8

M4R-7 M2R-7

M4R-6 M2R-6

M4R-5 M2R-5

M4R-4 M2R-4

M4R-3 M2R-3

M4R-2 M2R-2

M4L-8 M2L-8

M4R-1 M2R-1

M4L-7 M2L-7

M4L-6 M2L-6

M4L-5 M2L-5

M4L-4 M2L-4

M4L-3 M2L-3

M4L-2 M2L-2

M4L-1 M2L-1

M3R-8 M1R-8

M3R-7 M1R-7

M3R-6 M1R-6

M3R-5 M1R-5

M3R-4 M1R-4

M3R-3 M1R-3

M3R-2 M1R-2

M3L-8 M1L-8

M3R-1 M1R-1

M3L-7 M1L-7

M3L-6 M1L-6

M3L-5 M1L-5

M3L-4 M1L-4

M3L-3 M1L-3

word 2n+1

b0

LSBFIRST = 0
M3L-2 M1L-2

word 2n

M3L-1 M1L-1

b31

4 Microphones configuration
M2R-7

M2R-6

M2R-5

M2R-4

M2R-3

M2R-2

M2R-1

M2L-8

M2L-7

M2L-6

M2L-5

M2L-4

M2L-3

M2L-2

M2L-1

M1R-8

M1R-7

M1R-6

M1R-5

M1R-4

M1R-3

M1R-2

M1L-8

M1R-1

M1L-7

M1L-6

M1L-5

M1L-4

M1L-3

M2R-8

b0

LSBFIRST = 0
M1L-2

word n

M1L-1

b31

MSv35470V1

•

16-bit slot width (DS = 0b100 and SLOTSZ = 0). Refer to Figure 870.
For an 8-microphone configuration, four consecutive words read from the SAI_ADR
register contain a data byte from each microphone. Note that the 16-bit data of
SAI_ADR are right-aligned.
For a 4- or 2-microphone configuration, the SAI behavior is similar to the 8-microphone
configuration. Up to 2 words of 16 bits are required to acquire a byte from 4
microphones and a single word for 2 microphones.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Figure 870. SAI_ADR format in TDM mode, 16-bit slot width
8 Microphones configuration
M2R-2 M1R-2

M2R-3 M1R-3

M2R-4 M1R-4

M2R-5 M1R-5

M2R-6 M1R-6

M2R-7 M1R-7

M2R-8 M1R-8

M4R-2

M4R-3

M4R-4

M4R-5

M4R-6

M4R-7

M4R-8

M2L-7 M1L-7

M2L-8 M1L-8

M2L-6 M1L-6

M4L-7

M2R-1 M1R-1

M2L-5 M1L-5

M4L-6

M4R-1

M2L-4 M1L-4

M4L-5

b0

M4L-8

M2L-3 M1L-3

M4L-4

zeros

M2L-2 M1L-2

word 4n+1

M4L-3

zeros

M2L-1 M1L-1

word 4n

LSBFIRST = 0

M4L-2

b16 b15

M4L-1

b31

...
zeros

word 4n+3

4 Microphones configuration
b0
M2R-7 M1R-7

M2R-6 M1R-6

M2R-5 M1R-5

M2R-4 M1R-4

M2R-3 M1R-3

M2R-2 M1R-2

M2R-1 M1R-1

M2L-8 M1L-8

M2L-7 M1L-7

M2L-6 M1L-6

M2L-5 M1L-5

M2L-4 M1L-4

zeros

M2L-3 M1L-3

word 2n+1

M2L-2 M1L-2

zeros

M2L-1 M1L-1

LSBFIRST = 0

word 2n

M2R-8 M1R-8

b31

2 Microphones configuration
M1R-7

M1R-6

M1R-5

M1R-4

M1R-3

M1R-2

M1R-1

M1L-8

M1L-7

M1L-6

M1L-5

M1L-4

M1L-3

zeros

M1L-2

M1L-1

word 2n

b0

LSBFIRST = 0

M1R-8

b31

MSv35471V1

•

Using an 8-bit slot width (DS = 0b010 and SLOTSZ = 0). Refer to Figure 871.
For an 8-microphone configuration, eight consecutive words read from the SAI_ADR
register contain a byte of data from each microphone. Note that the 8-bit data of
SAI_ADR are right-aligned.
For a 4- or 2-microphone configuration, the SAI behavior is similar to the 8-microphone
configuration. Up to four words of eight bits are required to acquire a byte from four
microphones and two words from two microphones.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Figure 871. SAI_ADR format in TDM mode, 8-bit slot width
8 Microphones configuration
M1R-2 M1L-2

M1R-3 M1L-3

M1R-4 M1L-4

M1R-5 M1L-5

M1R-6 M1L-6

M1R-7 M1L-7

M1R-8 M1L-8

M4R-4

M4R-5

M4R-6

M4R-7

M4R-8

zeros

b0

M4R-3

word 8n+1

b8 b7
M1R-1 M1L-1

zeros

M4R-2

word 8n

M4R-1

LSBFIRST = 0

b31

...
zeros

word 8n+7

4 Microphones configuration
M1R-3 M1L-3

M1R-4 M1L-4

M1R-5 M1L-5

M1R-6 M1L-6

M1R-7 M1L-7

M2R-4

M2R-5

M2R-6

M2R-7

zeros

word 4n+3

M2R-3

...

M1R-2 M1L-2

zeros

M1R-1 M1L-1

word 4n+1

M2R-2

zeros

M2R-1

word 4n

M1R-8 M1L-8

b0

LSBFIRST = 0

M2R-8

b31

2 Microphones configuration
b0
M1R-7 M1L-7

M1R-6 M1L-6

M1R-5 M1L-5

M1R-4 M1L-4

zeros

M1R-3 M1L-3

word 2n+1

M1R-2 M1L-2

zeros

M1R-1 M1L-1

LSBFIRST = 0

word 2n

M1R-8 M1L-8

b31

MSv35472V1

TDM mode configuration for PDM interface
The SAI_A serial audio interface is internally connected to the PDM interface to get the
microphone samples. The user application must configure the SAI_A serial audio interface
as shown in Table 710 to ensure a good connection with the PDM interface.
Table 710. SAI_A configuration for TDM mode

<!-- pagebreak -->

Bit Fields

Values

Comments

MODE

0b01

Mode must be MASTER receiver.

PRTCFG

0b00

Free protocol for TDM.

DS

X

To be adjusted according to the required data format, in accordance with
the frame length and the number of slots (FRL and NBSLOT). See
Table 711.

LSBFIRST

X

This parameter can be used according to the desired data format.

CKSTR

0

Signal transitions occur on the rising edge of the SCK_A bit clock. Signals
are stable on the falling edge of the bit clock.

MONO

0

Stereo mode.

FRL

X

To be adjusted according to the number of microphones (MICNBR). See
Table 711.

FSALL

0

Pulse width is one bit clock cycle.

RM0456 Rev 6

RM0456

Serial audio interface (SAI)
Table 710. SAI_A configuration for TDM mode (continued)
Bit Fields

Values

Comments

FSDEF

0

FS signal is a start of frame.

FSPOL

1

FS is active high.

FSOFF

0

FS is asserted on the first bit of slot 0.

FBOFF

0

No offset on slot.

SLOTSZ

0

Slot size = data size.

NBSLOT

X

To be adjusted according to the required data format, in accordance with
the slot size, and the frame length (FRL and DS). See Table 711.

SLOTEN

X

To be adjusted according to NBSLOT.

NODIV

1

No need to generate a master clock MCLK.

MCKDIV

X

Depends on the frequency provided to sai_a_ker_ck input.
This parameter must be adjusted to generate the proper bitstream clock
frequency. See Table 711.

Adjusting the bitstream clock rate
To program the serial audio interface properly, the user application must take into account
the settings given in Table 710, and follow the sequence below:
1.

Adjust the bit clock frequency (FSCK_A) according to the required frequency for the
PDM bitstream clock, using the following formula:
F SCK_A = F PDM_CK × ( MICNBR + 1 ) × 2
MICNBR can be 0, 1, 2 or 3 (0 = 2 microphones; see Section 69.6.18)

2.

Set the frame length (FRL) using the following formula:
FRL = ( 16 × ( MICNBR + 1 ) ) – 1

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

up to 6
48 kHz

up to 4

up to 2

up to 8

up to 6
16 kHz
up to 4

up to 2

bit clock
(SCK_A)

Frame sync.
(FS_A)

Comments

frequency

frequency

NBSLOT

up to 8

Wanted
SAI_CKn
frequency

DS

Microphone
Nber of
sampling
microphones
rate

FRL

Table 711. TDM frame configuration examples(1)(2)

3.072 MHz

24.576 MHz

384 kHz

63

0b111

1

2 slots of 32 bits per frame

3.072 MHz

24.576 MHz

384 kHz

63

0b100

3

4 slots of 16 bits per frame

3.072 MHz

24.576 MHz

384 kHz

63

0b010

7

8 slots of 8 bits per frame

3.072 MHz

18.432 MHz

384 kHz

47

0b110

1

2 slots of 24 bits per frame

3.072 MHz

18.432 MHz

384 kHz

47

0b100

2

3 slots of 16 bits per frame

3.072 MHz

18.432 MHz

384 kHz

47

0b010

5

6 slots of 8 bits per frame

3.072 MHz

12.288 MHz

384 kHz

31

0b111

0

1 slot of 32 bits per frame

3.072 MHz

12.288 MHz

384 kHz

31

0b100

1

2 slots of 16 bits per frame

3.072 MHz

12.288 MHz

384 kHz

31

0b010

3

4 slots of 8 bits per frame

3.072 MHz

6.144 MHz

384 kHz

15

0b100

0

1 slots of 16 bits per frame

3.072 MHz

6.144 MHz

384 kHz

15

0b010

1

2 slots of 8 bits per frame

1.024 MHz

8.192 MHz

128 kHz

63

0b111

1

2 slots of 32 bits per frame

1.024 MHz

8.192 MHz

128 kHz

63

0b100

3

4 slots of 16 bits per frame

1.024 MHz

8.192 MHz

128 kHz

63

0b010

7

8 slots of 8 bits per frame

1.024 MHz

6.144 MHz

128 kHz

47

0b110

1

2 slots of 24 bits per frame

1.024 MHz

6.144 MHz

128 kHz

47

0b010

5

6 slots of 8 bits per frame

1.024 MHz

4.096 MHz

128 kHz

31

0b111

0

1 slot of 32 bits per frame

1.024 MHz

4.096 MHz

128 kHz

31

0b100

1

2 slots of 16 bits per frame

1.024 MHz

4.096 MHz

128 kHz

31

0b010

3

4 slots of 8 bits per frame

1.024 MHz

2.048 MHz

128 kHz

15

0b100

0

1 slot of 16 bits per frame

1.024 MHz

2.048 MHz

128 kHz

15

0b010

1

2 slots of 8 bits per frame

1. Refer to Table 710: SAI_A configuration for TDM mode for additional information on TDM configuration. The sai_a_ker_ck
clock frequency provided to the SAI must be a multiple of the SCK_A frequency, and MCKDIV must be programmed
accordingly.
2. The above sai_a_ker_ck frequencies are given as examples only. Refer to section Reset and clock controller (RCC) to
check if they can be generated on the device.
3. The table above gives allowed settings for a decimation ratio of 64.

Adjusting the delay lines
When the PDM interface is enabled, the application can adjust on-the-fly the delay cells of
each microphone input via the SAI_PDMDLY register.
The new delay values become effective after two audio frames.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

69.4.11

Serial audio interface (SAI)

AC’97 link controller
The SAI is able to work as an AC’97 link controller. In this protocol:
•

The slot number and the slot size are fixed.

•

The frame synchronization signal is perfectly defined and has a fixed shape.

To select this protocol, set the PRTCFG[1:0] bits in the SAI_xCR1 register to 10. When
AC’97 mode is selected, only data sizes of 16 or 20 bits can be used, otherwise the SAI
behavior is not guaranteed.
•

The NBSLOT[3:0] and SLOTSZ[1:0] bits are consequently ignored.

•

The number of slots is fixed at 13 slots. The first one is 16 bits wide and all the others
are 20 bits wide (data slots).

•

The FBOFF[4:0] bits in the SAI_xSLOTR register are ignored.

•

The SAI_xFRCR register is ignored.

•

The MCLK is not used.

The FS signal from the block defined as asynchronous is configured automatically as an
output, since the AC’97 controller link drives the FS signal whatever the master or slave
configuration.
Figure 872 shows an AC’97 audio frame structure.
Figure 872. AC’97 audio frame

FS
1

2

CMD CMD

3

4

PCM

PCM

SDI

Tag ADDR DATA LFRONT RFRONT

SDO

Tag ADDR

STATUS STATUS PCM PCM
DATA LEFT RIGHT

5

6

9

10

11

12

LINE1
PCM
PCM
PCM
DAC CENTER LSURR RSURR

PCM
LFE

LINE2
DAC

HSET
DAC

IO
CTRL

LINE1
ADC

RSR
LVD

LINE2
ADC

HSET

IO
STATUS

PCM
MIC

7

RSR
VD

8

RSR
VD

MS192343V1

Note:

In the AC’97 protocol, bit 2 of the tag is reserved (always 0), so bit 2 of the TAG is forced to
0 level whatever the value written in the SAI FIFO.
For more details about tag representation, refer to the AC’97 protocol standard.
One SAI can be used to target an AC’97 point-to-point communication.
Using two SAIs (for devices featuring two embedded SAIs) enables the control of three
external AC’97 decoders as illustrated in Figure 873.
In SAI1, the audio block A must be declared as asynchronous master transmitter, whereas
the audio block B is defined to be slave receiver and internally synchronous to the audio
block A.
The SAI2 is configured for audio block A and B both synchronous with the external SAI1 in
slave receiver mode.

RM0456 Rev 6

<!-- pagebreak -->

3020

Serial audio interface (SAI)

RM0456

Figure 873. Example of typical AC’97 configuration on devices featuring at least
two embedded SAIs (three external AC’97 decoders)

AC’97 Link Controller
(Bit clock provider)
Audio block A

Primary codec 1

Master
SDA
FIFO
Block B
synchronous with
block A

Transmitter

FSA
SCLKA

Clock
generator

Sdata_out
Sync
Bit_clk
Sdata_in

Slave
SDB
FIFO

Receiver

FSB
SCLKB

Secondary codec 1

Audio block B
Sdata_out
Sync
Bit_clk

SAI1
Audio block A
Slave

Sdata_in
SDA

FIFO
Synchronous with
other SAI clocks

Receiver

FSA
SCLKA
Secondary codec 2

Clock
generator

Slave
SDB
FIFO

Receiver

FSB
SCLKB

Sdata_out
Sync
Bit_clk
Sdata_in

Audio block B
SAI2

MSv31173V1

In receiver mode, the SAI acting as an AC’97 link controller requires no FIFO request and so
no data storage in the FIFO when the codec-ready bit in slot 0 is decoded low. If bit
CNRDYIE is enabled in the SAI_xIM register, flag CNRDY is set in the SAI_xSR register
and an interrupt is generated. This flag is dedicated to the AC’97 protocol.

Clock generator programming in AC’97 mode
In AC’97 mode, the frame length is fixed at 256 bits, and its frequency must be set to
48 kHz. The formulas given in Section 69.4.8: SAI clock generator must be used with
FRL = 255, to generate the proper frame rate (FFS_x).

<!-- pagebreak -->

