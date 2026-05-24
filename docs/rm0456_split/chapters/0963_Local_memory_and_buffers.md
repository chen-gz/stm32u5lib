RM0456 Rev 6

RM0456

26.3.2

Filter math accelerator (FMAC)

Local memory and buffers
The unit contains a 256 x 16-bit read/write memory which is used for local storage:
•

Input values (the elements of the input vectors) are stored in two buffers, X1 and X2.

•

Output values (the results of the operations) are stored in another buffer, Y.

•

The locations and sizes of the buffers are designated as follows:
–

x1_base: the base address of the X1 buffer

–

x2_base: the base address of the X2 buffer

–

y_base: the base address of the Y buffer

–

x1_buf_size: the number of 16-bit addresses allocated to the X1 buffer

–

x2_buf_size: the number of 16-bit addresses allocated to the X2 buffer

–

y_buf_size: the number of 16-bit addresses allocated to the Y buffer.

These parameters are programmed in the corresponding registers when configuring the
unit.
The CPU (or DMA controller) can initialize the contents of each buffer using the Initialization
functions (Section 26.3.5: Initialization functions) and writing to the write data register. The
data is transferred to the location within the target buffer indicated by a write pointer. After
each new write, the write pointer is incremented. When the write pointer reaches the end of
the allocated buffer space, it wraps back to the base address. This feature is used to load
the elements of a vector prior to an operation, or to initialize a filter and load filter
coefficients.

Buffer configuration
The buffer sizes and base address offsets must be configured in the X1, X2 and Y buffer
configuration registers. For each function, the required buffer size is specified in the function
description in Section 26.3.6: Filter functions. The base addresses can be chosen anywhere
in internal memory, provided that all buffers fit within the internal memory address range
(0x00 to 0xFF), that is, base address + buffer size must be less than 256.
There is no constraint on the size and location of the buffers (they can overlap or even
coincide exactly). For filter functions it is recommended not to overlap buffers as this can
lead to erroneous behavior.
When circular buffer operation is required, an optional “headroom”, d, can be added to the
buffer size. Furthermore, a watermark level can be set, to regulate the CPU or DMA activity.
The value of d and the watermark level must be chosen according to the application
performance requirements. For maximum throughput, the input buffer must never go empty,
so d must be somewhat greater than the watermark level, allowing for any interrupt or DMA
latency. On the other hand, if the input data can not be provided as fast as the unit can
process them, the buffer can be allowed to empty waiting for the next data to be written, so
d can be equal to the watermark level (to ensure that no overflow occurs on the input).

26.3.3

Input buffers
The X1 and X2 buffers are used to store data for input to the MAC. Each multiplication takes
a value from the X1 buffer and a value from the X2 buffer and multiplies them together. A
pointer in the control unit generates the read address offset (relative to the buffer base
address) for each value. The pointers are managed by hardware according to the current
function.

RM0456 Rev 6

<!-- pagebreak -->

988

Filter math accelerator (FMAC)

RM0456
Figure 109. Input buffer areas
x2_base

x1_base
X1 buffer

X2 buffer

x1_buf_size

x2_buf_size
MSv45869V1

The X1 buffer can be used as a circular buffer, in which case new data are continually
transferred into the input buffer whenever space is available. Pre-loading this buffer is
optional for digital filters, since if no input samples have been written in the buffer when the
operation is started, it is flagged as empty, which triggers the CPU or DMA to load new
samples until there are enough to begin operation. Pre-loading is nevertheless useful in the
case of a vector operation, that is, the input data is already available in system memory and
circular operation is not required.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
Figure 110. Circular input buffer
x1_base
Available buffer space

x[n-N]

x[n-6]
x[n-5]
x1_buf_size

x[n-4]

These values in use for
calculating y[n]

x[n-3]
x[n-2]
x[n-1]
x[n]
x[n+1]
x[n+2]
x[n+3]

Next values already
loaded

x[n+4]
Write pointer
Available buffer space

MSv45870V1

The X2 buffer can only be used in vector mode (that is not circular), and needs to be preloaded, except if the contents of the buffer do not change from one operation to the next. For
filter functions, the X2 buffer is used to store the filter coefficients.
When operating as a circular buffer, the space allocated to the buffer (x1_buf_size) must
generally be bigger than the number of elements in use for the current calculation, so that
there are always new values available in the buffer. Figure 110 illustrates the layout of the
buffer for a filter operation. While calculating an output sample y[n], the unit uses a set of
N+1 input samples, x[n-N] to x[n]. When this is finished, the unit starts the calculation of
y[n+1], using the set of input samples x[n-N+1] to x[n+1]. The least-recent input sample, x[nN], drops out of the input set, and a new sample, x[n+1], is added to it.
The processor, or DMA controller, must ensure that the new sample x[n+1] is available in
the buffer space when required. If not, the buffer is flagged as empty, which stalls the
execution of the unit until a new sample is added. No underflow condition is signaled on the
X1 buffer.

RM0456 Rev 6

<!-- pagebreak -->

