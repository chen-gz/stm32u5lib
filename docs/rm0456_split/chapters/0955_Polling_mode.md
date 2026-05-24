RM0456 Rev 6

RM0456

CORDIC coprocessor (CORDIC)
Once the calculation starts, any attempt to read the CORDIC_RDATA register inserts bus
wait-states until the calculation is completed, before returning the result. It is then possible
for the software to write the input and immediately read the result without polling to see if it is
valid. Alternatively, the processor can wait for the appropriate number of clock cycles before
reading the result. This time can be used to program the CORDIC_CSR register for the next
calculation, and prepare the next input data, if needed. The CORDIC_CSR register can be
reprogrammed while a calculation is in progress, without affecting the result of the ongoing
calculation. In the same way, the CORDIC_WDATA register can be updated with the next
argument(s) once the previous ones have been taken into account. The next arguments and
settings remain pending until the previous calculation has completed.
When a calculation is finished, the result(s) can be read from the CORDIC_RDATA register.
If two 32-bit results are expected (NRES = 1, RESSIZE = 0), the primary result (RES1) is
read out first, followed by the secondary result (RES2). If only one 32-bit result is expected
(NRES = 0, RESSIZE = 0), then RES1 is output on the first read.
If 16-bit results are expected (RESSIZE = 1), a single read to CORDIC_RDATA fetches both
results packed into a 32-bit word. RES1 is in the lower half-word, and RES2 in the upper
half-word. In this case, it is recommended to program NRES = 0. IF NRES = 1, a second
read of CORDIC_RDATA must be performed to free up the CORDIC for the next operation.
The data from this second read must be discarded.
The next calculation starts when the expected number of results has been read, provided
the expected number of arguments has been written. This means that at any time, there can
be a calculation in progress, or waiting for the results to be read, and an operation pending.
Any further access to CORDIC_WDATA while an operation is pending cancels it and
overwrites the data.
The following sequence summarizes the use of the CORDIC_IP in zero-overhead mode:

25.3.7

1.

Program the CORDIC_CSR register with the appropriate settings

2.

Program the argument(s) for the first calculation in the CORDIC_WDATA register. This
launches the first calculation.

3.

If needed, update the CORDIC_CSR register settings for the next calculation.

4.

Program the argument(s) for the next calculation in the CORDIC_WDATA register.

5.

Read the result(s) from the CORDIC_RDATA register. This triggers the next
calculation.

6.

Go to step 3.

Polling mode
When a new result is available in the CORDIC_RDATA register, the RRDY flag is set in the
CORDIC_CSR register. The flag can be polled by reading the register. It is reset by reading
the CORDIC_RDATA register (once or twice, depending on the NRES field of the
CORDIC_CSR register).
Polling the RRDY flag takes slightly longer than reading the CORDIC_RDATA register
directly, since the result is not read as soon as it is available. The processor and bus
interface are not stalled while reading the CORDIC_CSR register, so this mode may be of
interest if stalling the processor is not acceptable (for example, if low latency interrupts must
be serviced).

RM0456 Rev 6

<!-- pagebreak -->

