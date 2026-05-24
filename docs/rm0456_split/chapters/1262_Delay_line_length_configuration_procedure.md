1264

Delay block (DLYB)

RM0456

Once the delay line length is configured, a dephased output clock can be selected by the
output clock multiplexer. This is done through SEL[3:0]. The output clock is only available on
the selected phase when SEN is set to 0.
The table below gives a summary of the delay block control.
Table 300. Delay block control
DEN SEN

UNIT

SEL

LNG

LNGF

Output clock

Don’t
care

Don’t care

Enabled (= Input clock)

0

0

Don’t care

Don’t care

x

1

Unit delay

Output clock phase

1

0

Unit delay(1) Output clock phase(2)

Length Length flag
Don’t
care

Disabled

Don’t care Enabled (= selected phase)

1. The unit delay can only be changed when SEN = 1.
2. The output clock phase can only be changed when SEN = 1.

32.4.4

Delay line length configuration procedure
LNG[11:0] is used to determine the delay line length with respect to the input clock period.
The length must be configured so that one full input clock period is covered by the delay line
length.
Note that despite the delay line has 12 unit delay elements, the following procedure
description returns a length between 0 and 10, as the upper delay output value is used to
ensure that the delay is calibrated over one full input clock cycle. Depending on the clock
frequency and UNIT value, unit delay element 10 may also be truncated from the clock
cycle length.
A clock input (free running clock) must be present during the whole tuning procedure.
To configure the delay line length to one period of the Input clock, follow the sequence
below:
1.

Enable the delay block by setting DEN bit to 1.

2.

Enable the length sampling by setting SEN bit to 1.

3.

Enable all delay cells by setting SEL[3:0] to 12.

4.

For UNIT[6:0] = 0 to 127 (this step must be repeated until the delay line length is
configured):
a)

Update the UNIT[6:0] value and wait till the length flag LNGF is set to 1.

b)

Read LNG[11:0].

If (LNG[10:0] > 0) and (LNG[11] or LNG[10] = 0), the delay line length is configured to
one input clock period.
5.

Determine how many unit delays (N) span one input clock period: for N = 0 to 10, if
LNG[N] = 1, the number of unit delays spanning the input clock period = N.

6.

Disable the length sampling by clearing SEN to 0.

If an output clock delay smaller than one input clock period is needed the delay line length
can be reduced smaller than one input clock period. This allows a smaller unit delay,
providing a higher resolution spanning a shorter time interval.

<!-- pagebreak -->

