1921

True random number generator (RNG)

RM0456

48.3

RNG functional description

48.3.1

RNG block diagram
Figure 456 shows the RNG block diagram.
Figure 456. RNG block diagram

True RNG

rng_hclk
rng_clk

Banked Registers

Conditioning logic
128-bit data output

CONDRST

4x32-bit
FIFO

RNG_CR
RNG_DR
AHB
interface

Alarms

RNG_SR

Fault detection
Clock checker
Health tests

AHB clock domain

1-bit

Post-processing (optional)

DIV

32-bit AHB bus

rng_it

Sampling (x N) + XOR

RNG clock domain

rng_itamp_out

en_osc

Analog
Analog ... Analog
noise
noise
noise
source 1 source 2
source N
Analog noise source

MSv42098V3

48.3.2

RNG internal signals
Table 462 describes a list of useful-to-know internal signals available at the RNG level, not
at the STM32 product level (on pads).
Table 462. RNG internal input/output signals
Signal name

Signal type

rng_it

Digital output

RNG global interrupt request

rng_hclk

Digital input

AHB clock

rng_clk

Digital input

RNG dedicated clock, asynchronous to rng_hclk

digital output

RNG internal tamper event signal to TAMP (XORed), triggered
when an unexpected hardware fault occurs. When this signal is
triggered, RNG stops delivering random samples, requiring a
reset and a new initialization to be usable again.

rng_itamp_out

48.3.3

Description

Random number generation
The true random number generator (RNG) delivers truly random data through its AHB
interface at deterministic intervals.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

True random number generator (RNG)
Within its boundary RNG integrates all the required NIST components depicted on
Figure 457. Those components are an analog noise source, a digitization stage, a
conditioning algorithm, a health monitoring block and two interfaces that are used to interact
with the entropy source: GetEntropy and HealthTest.
Figure 457. NIST SP800-90B entropy source model

Error message

Output

(HealthTest)

(GetEntropy)

Heath
tests

Conditioning
(optional)
Raw data

Post-processing
(optional)
Digitization

Noise source

Entropy source
MSv44200V3

The components pictured above are detailed hereafter.

Noise source
The noise source is the component that contains the non-deterministic, entropy-providing
activity that is ultimately responsible for the uncertainty associated with the bitstring output
by the entropy source. This noise source provides 1-bit samples. It is composed of:
•

Multiple analog noise sources (x6), each based on three XORed free-running ring
oscillator outputs. It is possible to disable those analog oscillators to save power, as
described in Section 48.3.8. Multiple oscillators are also disabled for the configuration
A (see Table 465: RNG configurations).

•

The XORing of all the noise sources into a single analog output.

•

A sampling stage of this output clocked by a dedicated clock input (rng_clk with
integrated divider), delivering a 1-bit raw data output.

This noise source sampling is independent to the AHB interface clock frequency (rng_hclk),
with a possibility for the software to decrease the sampling frequency by using the
integrated divider.
Note:

In Section 48.6 the recommended RNG clock frequencies and associated divider value are
given.

Post processing
In the NIST configuration no post-processing is applied to the sampled noise source. In nonNIST configuration B (as defined in Section 48.6.2) a normalization debiasing is applied,
RM0456 Rev 6

<!-- pagebreak -->

1921

True random number generator (RNG)

RM0456

that is half of the bits are taken from the sampled noise source, half of the bits are taken
from the inverted sampled noise source.

Conditioning
The conditioning component in the RNG is a deterministic function that increases the
entropy rate of the resulting fixed-length bitstrings output (128-bit). The NIST SP800-90B
target is full entropy on the output (128-bit).
The times required between two random number generations, and between the RNG
initialization and availability of first sample are described in Section 48.5.

Output buffer
A data output buffer can store up to four 32-bit words that have been output from the
conditioning component. When four words have been read from the output FIFO through
the RNG_DR register, the content of the 128-bit conditioning output register is pushed into
the output FIFO, and a new conditioning round is automatically started. Four new words are
added to the conditioning output register after a number of clock cycles specified in
Section 48.5.
Whenever a random number is available through the RNG_DR register, the DRDY flag
changes from 0 to 1. This flag remains high until the output buffer becomes empty after
reading four words from the RNG_DR register.
Note:

When interrupts are enabled an interrupt is generated when this data ready flag transitions
from 0 to 1. Interrupt is then cleared automatically by the RNG as explained above.

Health checks
This component ensures that the entire entropy source (with its noise source) starts then
operates as expected, obtaining assurance that failures are caught quickly and with a high
probability and reliability.

<!-- pagebreak -->

