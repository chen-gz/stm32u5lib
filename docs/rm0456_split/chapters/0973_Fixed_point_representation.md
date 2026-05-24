•

The parameter P contains the length, N + 1, of the coefficient vector B in the range
[2:64].

•

The parameter Q contains the length, M, of the coefficient vector A in the range [1:63].

•

The parameter R contains the gain to be applied to the accumulator output. The value
output to the Y buffer is multiplied by 2R, where R is in the range [0:7].

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
The function completes when the START bit in the FMAC_PARAM register is reset by
software.

26.3.7

Fixed point representation
The FMAC operates in fixed point signed integer format. Input and output values are q1.15.
In q1.15 format, numbers are represented by one sign bit and 15 fractional bits (binary
decimal places). The numeric range is therefore -1 (0x8000) to 1 - 2-15 (0x7FFF).
The accumulator has 26 bits, of which 22 are fractional and 4 are integer/sign (q4.22). This
allows it to support partial accumulation sums in the range -8 (0x2000000) to +7.99999976
(0x1FFFFFF). A programmable gain from 0dB to 42dB in steps of 6dB can be applied at the
output of the accumulator.
Note that the content of the accumulator is not saturated if the numeric range is exceeded.
Partial sums whose value is greater than +7.99999976 or less than -8, wrap but this is
harmless provided subsequent accumulations undo the wrapping. Nevertheless, the SAT
flag in the FMAC_SR register is set if wrapping occurs, and generates an interrupt if the
SATIEN bit is set in the FMAC_CR register. This helps in debugging the filter.
The data output by the accumulator can optionally be saturated, after application of the
programmable gain, by setting the CLIPEN bit in the FMAC_CR register. If this bit is set,
then any value which exceeds the numeric range of the q1.15 output, is set to 1 - 2-15 or -1,
according to the sign. If clipping is not enabled, the unused accumulator bits after applying
the gain is simply truncated.

26.3.8

Implementing FIR filters with the FMAC
The FMAC supports FIR filters of length N, where N is the number of taps or coefficients.
The minimum local memory requirement for a FIR filter of length N is 2N + 1:
–

N coefficients

–

N input samples

–

1 output sample

Since the local memory size is 256, the maximum value for N is 127.
If maximum throughput is required, it may be necessary to allocate a small amount of extra
space, d1 and d2, to the input and output sample buffers respectively, to ensure that the
filter never stalls waiting for a new input sample, or waiting for the output sample to be read.
In this case, the local memory requirement is 2N + d1 + d2.
The buffers must be configured as follows:
•

X1_BUF_SIZE = N + d1;

•

X2_BUF_SIZE = N;

•

Y_BUF_SIZE = d2 (or 1 if no extra space is required)

The buffer base addresses can be allocated anywhere, but the X2 buffer must not overlap
with the others, or else the coefficients are overwritten. An example configuration is:
•

X2_BASE = 0;

•

X1_BASE = N;

•

Y_BASE = 2N + d1

RM0456 Rev 6

<!-- pagebreak -->

