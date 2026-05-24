64

FPCM (kHz)

2

x

FADF_CCKx (MHz)

1

1

FRS (kHz)

0x2D (- 8.5 dB)

adf_proc_ck
(MHz)

64

HPFBYP

4

-

Total dec. ratio

#5

2

RSFLTD

2.048

1

RSFLTBYP

#4

SCALE

#3

MCICD + 1

1.024

CIC order (1)

#2

CCKDIV + 1

#1

PROCDIV + 1

adf_ker_ck (MHz)

Configuration

Table 405. Examples of ADF settings for microphone capture

1.024
1.536

16
16

RM0456

40.7.2

Audio digital filter (ADF)

Programming examples
Example 1
This example describes the programming of ADF for the capture of a signal coming from a
digital microphone, using only the CIC4, with a decimation of 48, assuming that the kernel
clock is 1.536 MHz. Typically, this configuration can be used to detect sound using the SAD.
Table 406. Programming sequence (CIC4)
Operations

Comments

Adjust the proper kernel clock frequency Assuming that the RCC is programmed to provide a kernel clock
via the RCC
(adf_ker_ck) of 1.536 MHz coming from a RC oscillator.
Select the proper ADF kernel clock
source via the RCC

Refer to the RCC of the product.

Enable the ADF clocks via the RCC

Refer to the RCC of the product.

Reset the ADF via the RCC

Refer to the RCC of the product.

AFMUX programming

Program the AFMUX to select ADF_SD0 and ADF_CCK0 functions.

Enable ADF processing clock:
ADF_CKGCR = 0x0001 0023

PROCDIV = 0 (bypass): adf_proc_ck frequency is 1.536 MHz.
CCKDIV = 1 (division by 2): ADF_CCK0 clock frequency is 768 kHz.
The ADF_CCK0 pin is set in output and generates a clock so that the
microphone can exit from low-power mode.

Serial interfaces configuration:
ADF_SITF0CR = 0x0000 1F01

SCKSRC = 0 to select ADF_CCK0 as serial clock.
SIFTMOD = 0 to select LF_MASTER SPI mode.
Clock absence feature is not working in this mode.
The serial interface is enabled.

Bitstream matrix configuration:
ADF_BSMX0CR = 0x0000 0000

DFLT0 filter takes the bitstream of SITF0, sampled on rising edge

Filters configuration (CIC):
ADF_DFLT0CICR = 0x0040 2F40

SCALE = 0x04 (12 dB): RSFLT is not enabled. Note that the gain is 8.5
dB, higher than recommended in order to improve signal accuracy for the
SAD. Saturation is not an issue in this case, as only the detection of a
signal much lower than the full scale is needed.
MCICD = 0x2F (decimation by 48)
CICMOD = 4 to select a Sinc4
DATSCR = 0 to select data coming from BSMX

Filters configuration (RSFLT and HPF):
ADF_DFLT0RSFR = 0x0000 0301

HPFC = 3: cut-off frequency of 16kHz * 0.0095 = 152 Hz
HPFBYP = 0: HPF not bypassed
RSFLTBYP = 1: RSFLT bypassed

Micro delay adjust:
ADF_DLY0CR = 0x0000 0000

Not used in this example

Enable interrupt events:
ADF_DFLT0IER = 0x0000 1000

Enable the interrupt events the application wants to handle.
In this example, SDDETIE is set to 1 to have an interrupt if a sound is
detected.

RM0456 Rev 6

<!-- pagebreak -->

