No DMA request is generated to program the CORDIC_CSR register. DMA mode is
therefore useful only when repeatedly performing the same function with the same settings.
The scale factor cannot be changed during a series of DMA transfers.

RM0456 Rev 6

RM0456

CORDIC coprocessor (CORDIC)

Note:

Each DMA request must be acknowledged, as a result of the DMA performing an access to
the CORDIC_WDATA or CORDIC_RDATA register. If an extraneous access to the relevant
register occurs before this, the acknowledge is asserted prematurely, and may block the
DMA channel. Therefore, when the DMA read channel is enabled, CPU access to the
CORDIC_RDATA register must be avoided. Similarly, the processor must avoid accessing
the CORDIC_WDATA register when the DMA write channel is enabled.

25.4

CORDIC registers
The CORDIC registers can be accessed only in 32-bit word format

25.4.1

CORDIC control/status register (CORDIC_CSR)
Address offset: 0x00
Reset value: 0x0000 0050

31
RRDY

30
Res.

29
Res.

28
Res.

27
Res.

26
Res.

25

24

Res.

Res.

23

22

21

20

19

18

17

16

Res.

ARG
SIZE

RES
SIZE

N
ARGS

NRES

DMA
WEN

DMA
REN

IEN

rw

rw

rw

rw

rw

rw

rw

6

5

4

3

2

1

0

r
15

14

13

12

11

Res.

Res.

Res.

Res.

Res.

10

9

8

7

SCALE[2:0]
rw

rw

PRECISION[3:0]
rw

rw

rw

rw

FUNC[3:0]
rw

rw

rw

rw

rw

Bit 31 RRDY: Result ready flag
0: No new data in output register
1: CORDIC_RDATA register contains new data.
This bit is set by hardware when a CORDIC operation completes. It is reset by hardware
when the CORDIC_RDATA register is read (NRES+1) times.
When this bit is set, if the IEN bit is also set, the CORDIC interrupt is asserted. If the
DMAREN bit is set, a DMA read channel request is generated. While this bit is set, no new
calculation is started.
Bits 30:23 Reserved, must be kept at reset value.
Bit 22 ARGSIZE: Width of input data
0: 32-bit
1: 16-bit
ARGSIZE selects the number of bits used to represent input data.
If 32-bit data is selected, the CORDIC_WDATA register expects arguments in q1.31 format.
If 16-bit data is selected, the CORDIC_WDATA register expects arguments in q1.15 format.
The primary argument (ARG1) is written to the least significant half-word, and the secondary
argument (ARG2) to the most significant half-word.
Bit 21 RESSIZE: Width of output data
0: 32-bit
1: 16-bit
RESSIZE selects the number of bits used to represent output data.
If 32-bit data is selected, the CORDIC_RDATA register contains results in q1.31 format.
If 16-bit data is selected, the least significant half-word of CORDIC_RDATA contains the
primary result (RES1) in q1.15 format, and the most significant half-word contains the
secondary result (RES2), also in q1.15 format.

RM0456 Rev 6

<!-- pagebreak -->

960

CORDIC coprocessor (CORDIC)

RM0456

Bit 20 NARGS: Number of arguments expected by the CORDIC_WDATA register
0: Only one 32-bit write (or two 16-bit values if ARGSIZE = 1) is needed for the next
calculation.
1: Two 32-bit values must be written to the CORDIC_WDATA register to trigger the next
calculation.
Reads return the current state of the bit.
Bit 19 NRES: Number of results in the CORDIC_RDATA register
0: Only one 32-bit value (or two 16-bit values if RESSIZE = 1) is transferred to the
CORDIC_RDATA register on completion of the next calculation. One read from
CORDIC_RDATA resets the RRDY flag.
1: Two 32-bit values are transferred to the CORDIC_RDATA register on completion of the
next calculation. Two reads from CORDIC_RDATA are necessary to reset the RRDY flag.
Reads return the current state of the bit.
Bit 18 DMAWEN: Enable DMA write channel
0: Disabled. No DMA write requests are generated.
1: Enabled. Requests are generated on the DMA write channel whenever no operation is
pending
This bit is set and cleared by software. A read returns the current state of the bit.
Bit 17 DMAREN: Enable DMA read channel
0: Disabled. No DMA read requests are generated.
1: Enabled. Requests are generated on the DMA read channel whenever the RRDY flag is
set.
This bit is set and cleared by software. A read returns the current state of the bit.
Bit 16 IEN: Enable interrupt.
0: Disabled. No interrupt requests are generated.
1: Enabled. An interrupt request is generated whenever the RRDY flag is set.
This bit is set and cleared by software. A read returns the current state of the bit.
Bits 15:11 Reserved, must be kept at reset value.
Bits 10:8 SCALE[2:0]: Scaling factor
The value of this field indicates the scaling factor applied to the arguments and/or results. A
value n implies that the arguments have been multiplied by a factor 2-n, and/or the results
need to be multiplied by 2n. Refer to Section 25.3.2 for the applicability of the scaling factor
for each function and the appropriate range.
Bits 7:4 PRECISION[3:0]: Precision required (number of iterations)
0: reserved
1 to 15: (Number of iterations)/4
To determine the number of iterations needed for a given accuracy refer to Table 210.
Note that for most functions, the recommended range for this field is 3 to 6.

<!-- pagebreak -->

