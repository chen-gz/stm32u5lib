988

Filter math accelerator (FMAC)
Note:

RM0456

If the flow of samples is controlled by a timer or other peripheral such as an ADC, the buffer
regularly goes empty, since the filter processes each new sample faster than the source can
provide it. This is an essential feature of filter operation.
If the number of free spaces in the buffer is less than the watermark threshold programmed
in the FULL_WM bitfield of the FMAC_X1BUFCFG register, the buffer is flagged as full. As
long as the full flag is not set, interrupts are generated, if enabled, to request more data for
the buffer. The watermark allows several data to be transferred under one interrupt, without
danger of overflow. Nevertheless, if an overflow does occur, the OVFL error flag is set and
the write data is ignored. The write pointer is not incremented in the event of an overflow.
The operation of the X1 buffer during a filtering operation is illustrated in Figure 111. This
example shows an 8-tap FIR filter with a watermark set to four.
Figure 111. Circular input buffer operation
x[n-7]
x[n-6]
x[n-5]
x[n-4]
x[n-3]
x[n-2]
x[n-1]
x[n]
x[n+1]

Write pointer

Write pointer

x[n-7]
x[n-6]
x[n-5]
x[n-4]
x[n-3]
x[n-2]
x[n-1]
x[n]
x[n+1]
x[n+2]
x[n+3]
x[n+4]

Filter finishes current output
sample and starts using
next sample in buffer
freeing up a space, since
oldest sample is no longer
needed.
No new samples available,
so buffer empty flag is set.

Four new samples written
into buffer. Write pointer is
incremented by 4. If write
pointer reaches the end of
the buffer space it wraps to
the beginning. Number of
free spaces left in buffer is
less than watermark, so
buffer full flag is set.

Write pointer

Filter using eight
samples x[n-7] to x[n]

Write pointer

x[n-7]
x[n-6]
x[n-5]
x[n-4]
x[n-3]
x[n-2]
x[n-1]
x[n]

x[n-7]
x[n-6]
x[n-5]
x[n-4]
x[n-3]
x[n-2]
x[n-1]
x[n]
x[n+1]
x[n+2]
x[n+3]
Buffer empty flag is reset,
so filter continues with
next sample

MSv45871V1

26.3.4

Output buffer
The Y (output) buffer is used to store the output of an accumulation. Each new output value
is stored in the buffer until it is read by the processor or DMA controller. Each time a read
access is made to the read data register, the read data is fetched from the address indicated
by the read pointer. This pointer is incremented after each read, and wraps back to the base
address when it reaches the end of the allocated Y buffer space.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Filter math accelerator (FMAC)
Figure 112. Circular output buffer
y_base
Available buffer space

Read pointer

y[n-M-4]
y[n-M-3]
y[n-M-2]

These samples not yet read

y[n-M-1]
y[n-M]

y_buf_size

y[n-6]
y[n-5]

These samples in use
for calculating y[n]

y[n-4]
y[n-3]
y[n-2]
y[n-1]
Next sample

y[n]

Available buffer space

MSv45873V1

The Y buffer can also operate as a circular buffer. If the address for the next output value is
the same as that indicated by the read pointer (an unread sample), then the buffer is flagged
as full and execution stalled until the sample is read.
In the case of IIR filters, the Y buffer is used to store the set of M previous output samples,
y[n-M] to y[n-1], used for calculating the next output sample y[n]. Each time a new sample is
added to the set, the least recent sample y[n-M] drops out.
If the number of unread data in the buffer is less than the watermark threshold programmed
in the EMPTY_WM bitfield of the FMAC_YBUFCFG register, the buffer is flagged as empty.
As long as the empty flag is not set, interrupts or DMA requests are generated, if enabled, to
request reads from the buffer. The watermark allows several data to be transferred under
one interrupt, without danger of underflow. Nevertheless, if an underflow does occur, the
UNFL error flag is set. In this case, the read pointer is not incremented and the read
operation returns the content of the memory at the read pointer address.
The operation of the Y buffer in circular mode is illustrated in Figure 113. This example
shows a 7-tap IIR filter with a watermark set to four.

RM0456 Rev 6

<!-- pagebreak -->

