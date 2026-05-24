RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
The filter is started by writing to the FMAC_PARAM register with the following bitfield values:
•

FUNC = 8 (FIR filter);

•

P = N (number of coefficients);

•

Q = “Don’t care”;

•

R = Gain;

•

START = 1;

If less than N + d - 2FULL_WM values have been pre-loaded in the X1 buffer, the X1FULL flag
remains low. If the WIEN bit is set in the FMAC_CR register, then the interrupt request is
asserted immediately to request the processor to write 2FULL_WM additional samples into the
buffer, via the FMAC_WDATA register. It remains asserted until the X1FULL flag goes high
in the FMAC_SR register. The interrupt service routine must check the X1FULL flag after
every 2FULL_WM writes to the FMAC_WDATA register, and repeat the transfer until the flag
goes high. Similarly, if the DMAWEN bit is set in the FMAC_CR register, DMA write channel
requests are generated until the X1FULL flag goes high.
The filter calculates the first output sample when at least N samples have been written into
the X1 buffer (including any pre-loaded samples).
When 2EMPTY_WM output samples have been written into the Y buffer, the YEMPTY flag in
the FMAC_SR register goes low. If the RIEN bit is set in the FMAC_CR register, the
interrupt request is asserted to request the processor to read 2EMPTY_WM samples from the
buffer, via the FMAC_RDATA register. It remains asserted until the YEMPTY flag goes high.
The interrupt service routine must check the YEMPTY flag after every 2EMPTY_WM reads from
the FMAC_RDATA register, and repeat the transfer until the flag goes high. If the DMAREN
bit is set in the FMAC_CR, DMA read channel requests are generated until the YEMPTY
flag goes high.
The filter continues to operate in this fashion until it is stopped by the software resetting the
START bit.

26.3.9

Implementing IIR filters with the FMAC
The FMAC supports IIR filters of length N, where N is the number of feed-forward taps or
coefficients. The number of feedback coefficients, M, can be any value from 1 to N-1. Only
direct form 1 implementations can be realized, so filters designed for other forms need to be
converted.
The minimum memory requirement for an IIR filter with N feed-forward coefficients and M
feed-back coefficients is 2N + 2M:
•

N + M coefficients

•

N input samples

•

M output samples

If M = N-1, then the maximum filter length that can be implemented is N = 64.
As for the FIR, for maximum throughput, a small amount of additional space, d1 and d2, is
allowed in the input and output buffer size respectively, making the total memory
requirement 2M + 2N + d1 + d2.
The buffers must be configured as follows:
•

X1_BUF_SIZE = N + d1;

•

X2_BUF_SIZE = N + M;

•

Y_BUF_SIZE = M + d2;
RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

RM0456

The buffer base addresses can be allocated anywhere, but must not overlap. An example
configuration is given below:

Note:

•

X2_BASE = 0;

•

X1_BASE = N + M;

•

Y_BASE = 2N + M + d1;

The FULL_WM bitfield of X1 buffer configuration register must be programmed with a value
less than or equal to log2(d1), otherwise the buffer is flagged full before N input samples
have been written, and no more samples are requested. Similarly, the EMPTY_WM bitfield
of the Y buffer configuration register must be less than or equal to log2(d2).
The filter coefficients (N feed-forward followed by M feedback) must be pre-loaded into the
X2 buffer, using the Load X2 Buffer function. The X1 buffer can optionally be pre-loaded with
any number of samples up to a maximum of N. The Y buffer can optionally be pre-loaded
with any number of values up to a maximum of M. This has the effect of initializing the
feedback delay line.
After configuring the buffers, the FMAC_CR register must be programmed in the same way
as for the FIR filter (see Section 26.3.8: Implementing FIR filters with the FMAC).
The filter is started by writing to the FMAC_PARAM register with the following bitfield values:
•

FUNC = 9 (IIR filter);

•

P = N (number of feed-forward coefficients);

•

Q = M (number of feed-back coefficients);

•

R = Gain;

•

START = 1;

If less than N + d - 2FULL_WM values have been pre-loaded in the X1 buffer, the X1FULL flag
remains low. If the WIEN bit is set in the FMAC_CR register, then the interrupt request is
asserted immediately to request the processor to write 2FULL_WM additional samples into the
buffer, via the FMAC_WDATA register. It remains asserted until the X1FULL flag goes high
in the FMAC_SR register. The interrupt service routine must check the X1FULL flag after
every 2FULL_WM writes to the FMAC_WDATA register, and repeat the transfer until the flag
goes high. Similarly, if the DMAWEN bit is set in the FMAC_CR register, DMA write channel
requests are generated until the X1FULL flag goes high.
The filter calculates the first output sample when at least N samples have been written into
the X1 buffer (including any pre-loaded samples). The first sample is calculated using the
first N samples in the X1 buffer, and the first M samples in the Y buffer (whether or not they
are preloaded. The first output sample is written into the Y buffer at Y_BASE + M.
When 2EMPTY_WM new output samples have been written into the Y buffer, the YEMPTY flag
in the FMAC_SR register goes low. If the RIEN bit is set in the FMAC_CR register, the
interrupt request is asserted to request the processor to read 2EMPTY_WM samples from the
buffer, via the FMAC_RDATA register. It remains asserted until the YEMPTY flag goes high.
The interrupt service routine must check the YEMPTY flag after every 2EMPTY_WM reads from
the FMAC_RDATA register, and repeat the transfer until the flag goes high. If the DMAREN
bit is set in the FMAC_CR, DMA read channel requests are generated until the YEMPTY
flag goes high
The filter continues to operate in this fashion until it is stopped by the software resetting the
START bit.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)

26.3.10

Examples of filter initialization
Figure 116. X1 buffer initialization

Software register
access

FMAC_PARAM
register write:
FUNC = 1
(Load X1 Buffer)
P=4
START = 1

FMAC_WDATA
register write:
WDATA = x[0]

FMAC_WDATA
register write:
WDATA = x[1]

FMAC_WDATA
register write:
WDATA = x[2]

FMAC_WDATA
register write:
WDATA = x[3]

START
X1_BASE
X1_BASE + 0x1
X1_BASE + 0x2
X1_BASE + 0x3
X1_BASE + 0x4
X1_BASE + 0x5

XX

x[0]
XX

x[1]
XX

x[2]
XX

x[3]
XX
XX

X1_FULL
MSv47128V1

The example in Figure 116 illustrates an X1 buffer pre-load with four samples (P = 4). The
buffer size is six (X1_BUF_SIZE = 6). The initialization is launched by programming the
FMAC_PARAM register with the START bit set. The four samples are then written to
FMAC_WDATA, and transferred into local memory from X1_BASE onwards. The START bit
resets after the fourth sample has been written. At this point, the X1 buffer contains the four
samples, in order of writing, and the write pointer (next empty space) is at X1_BASE + 0x4.

RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

26.3.11

RM0456

Examples of filter operation
Figure 117. Filtering example 1
FMAC_WDATA
register write:
WDATA = x[4]

Software register access

FMAC_PARAM
register write:
FUNC = 8 (FIR Filter)
P=4
START = 1

FMAC_RDATA
register read:
RDATA = y[0]

FMAC_WDATA
register write:
WDATA = x[5]

FMAC_WDATA
register write:
WDATA = x[7]

FMAC_WDATA
register write:
WDATA = x[6]

FMAC_WDATA
register write:
WDATA = x[8]

FMAC_RDATA
register read:
RDATA = y[1]

FMAC_WDATA
register write:
WDATA = x[9]

FMAC_RDATA
register read:
RDATA = y[2]

FMAC_RDATA
register read:
RDATA = y[3]

START
x[0]

X1_BASE

X1 buffer
X1_BUF_SIZE = 6

Spare
x[1]

X1_BASE + 0x1

x[7]

x[2]

X1_BASE + 0x2

Spare

x[8]

x[3]

X1_BASE + 0x3
X1_BASE + 0x4

x[6]
Spare

Spare

XX

x[9]

x[4]
XX

X1_BASE + 0x5

Spare
x[5]

No more space in X1 buffer
X1_FULL
Interrupt

No more space in Y buffer
Calculate y[0] using x[0:3]

MAC activity
Y buffer
Y_BUF_SIZE = 2

Y_BASE
Y_BASE + 0x1

XX

Calculate y[1] using x[1:4]

y[0]

Calculate y[2] using x[2:5]

Spare

XX

Stalled

Calculate y[3] using x[3:6]

y[2]
y[1]

Spare

Calculate y[4] using x[4:7]

Calculate y[5]

Spare

y[3]

y[3]

Spare

Y_EMPTY

MSv47129V1

The example in Figure 117 illustrates the beginning of a filter operation. The filter has four
taps (P=4). The X1 buffer size is six and the Y buffer size is two. The FULL_WM and
EMPTY_WM bitfields are both set to 0. Prior to starting the filter, the X1 buffer has been preloaded with four samples, x[0:3] as in Figure 116. So the filter starts calculating the first
output sample, y[0], immediately after the START bit is set. Since the X1FULL flag is not set
(due to two uninitialized spaces in the X1 buffer), the interrupt is asserted straight away, to
request new data. The processor writes two new samples, x[4] and x[5], to the
FMAC_WDATA register, which are transferred to the empty locations in the X1 buffer.
In the mean time, the FMAC finishes calculating the first output sample, y[0], and writes it
into the Y buffer, causing the Y_EMPTY flag to go low. At the same time, the x[0] sample is
discarded, as it is no longer required, freeing up its location in memory (at X1_BASE). The
FMAC can immediately start work on the second output sample, y[1], since all the required
input samples x[1:5] are present in the X1 buffer.
Since the Y_EMPTY flag is low, the interrupt remains active after the processor finishes
writing x[5]. The processor reads y[0] from the FMAC_RDATA register, freeing up its
location in the Y buffer. There are now no samples in the output buffer since y[1] is still being
calculated, so the Y_EMPTY flag goes high. Nevertheless, the interrupt remains active,
because there is still free space in the X1 buffer, which the processor next fills with x[6], and
so on.
Note:

<!-- pagebreak -->

In this example, the processor can fill the input buffer more quickly than the FMAC can
process them, so the X1_full flag regularly goes active. However, it struggles to read the Y
buffer fast enough, so the FMAC stalls regularly waiting for space to be freed up in the Y
buffer. This means the filter is not executing at maximum throughput. The reason is that the

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
filter length is small and the processor relatively slow, in this example. So increasing the Y
buffer size would not help.
Figure 118. Filtering example 2
FMAC_WDATA
register write:
WDATA = x[4]

Software register
access

FMAC_PARAM
register write:
FUNC = 8 (FIR Filter)
P=6
START = 1

FMAC_RDATA
register read:
RDATA = y[0]

FMAC_WDATA
register write:
WDATA = x[5]

FMAC_RDATA
register read:
RDATA = y[1]

FMAC_WDATA
register write:
WDATA = x[6]

FMAC_WDATA
register write:
WDATA = x[7]

FMAC_WDATA
register write:
WDATA = x[8]

START

x[0]

X1_BASE

Spare
Spare

x[7]
Spare

x[2]

X1 buffer X1_BASE + 0x2
X1_BUF_SIZE = 6 X1_BASE + 0x3
X1_BASE + 0x4

x[6]

x[1]

X1_BASE + 0x1

x[8]

x[3]
XX

x[4]
XX

X1_BASE + 0x5

x[5]

No more space in X1 buffer
X1_FULL
Interrupt
Not enough samples in input buffer
MAC activity
Y buffer
Y_BUF_SIZE = 2

Y_BASE
Y_BASE + 0x1

Stalled

Calculate y[0] using x[0:5]

Stalled

XX

Calculate y[1] using x[1:6]

y[0]
XX

Stalled

Calculate y[2] using x[2:7]

Spare
y[1]

Stalled

Calculate

y[2]
Spare

Y_EMPTY
MSv47130V1

The example in Figure 118 illustrates the beginning of the same filter operation, but this time
the filter has six taps (P=6). The X1 buffer size is six and the Y buffer size is two. The
FULL_WM and EMPTY_WM bitfields are both set to 0. Prior to starting the filter, the X1
buffer has been pre-loaded with four samples, x[0:3] as in Figure 116. Because there are
not enough samples in the input buffer, the X1FULL flag is not set, so the interrupt is
asserted straight away, to request new data. The FMAC is stalled.
The processor writes two new samples, x[4] and x[5], to the FMAC_WDATA register, which
are transferred to the empty locations in the X1 buffer. As soon as there are six unused
samples in the X1 buffer, the X1_FULL flag goes active (since the buffer size is six), causing
the interrupt to go inactive. The FMAC starts calculating the first output sample, y[0]. Since
this requires all six input samples, there are no free spaces in the X1 buffer and so the
X1_FULL flag remains active. Only when the FMAC finishes calculating y[0] and writes it
into the Y buffer, can x[0] be discarded, freeing up a space in the X1 buffer, and deasserting
X1_FULL. At the same time, the Y_EMPTY flag goes inactive. Both these flag states cause
the interrupt to be asserted, requesting the processor to write a new input sample, first of all,
and then read the output sample just calculated. The FMAC remains stalled until a new
input sample is written.
In this example, the processor has to wait for the FMAC to finish calculating the current
output sample, before it can write a new input sample, and therefore the X1 buffer regularly
goes empty, stalling the FMAC. This can be avoided by allowing some extra space in the
input buffer.

RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

26.3.12

RM0456

Filter design tips
The FMAC architecture imposes some constraints detailed below, on the design of digital
filters.

<!-- pagebreak -->

