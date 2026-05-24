RM0456 Rev 6

RM0456
Note:

Inter-integrated circuit interface (I2C)
tAF(min) and tAF(max) are only part of the condition when the analog filter is enabled. Refer to
the device datasheet for tAF values.
The tHD;DAT time can at maximum be 3.45 µs for Standard-mode, 0.9 µs for Fast-mode, and
0.45 µs for Fast-mode Plus. It must be lower than the maximum of tVD;DAT by a transition
time. This maximum must only be met if the device does not stretch the LOW period (tLOW)
of the SCL signal. When it stretches SCL, the data must be valid by the set-up time before it
releases the clock.
The SDA rising edge is usually the worst case. The previous condition then becomes:
SDADEL ≤ {tVD;DAT (max) - tr (max) - tAF (max) - [(DNF + 4) x tI2CCLK]} / {(PRESC + 1) x tI2CCLK}

Note:

This condition can be violated when NOSTRETCH = 0, because the device stretches SCL
low to guarantee the set-up time, according to the SCLDEL[3:0] value.
After tSDADEL, or after sending SDA output when the target had to stretch the clock because
the data was not yet written in I2C_TXDR register, the SCL line is kept at low level during
the setup time. This setup time is tSCLDEL = (SCLDEL + 1) x tPRESC, where
tPRESC = (PRESC + 1) x tI2CCLK. tSCLDEL impacts the setup time tSU;DAT.
To bridge the undefined region of the SDA transition (rising edge usually worst case), the
user must program SCLDEL[3:0] so as to fulfill the following condition:
{[tr (max) + tSU;DAT (min)] / [(PRESC + 1) x tI2CCLK]} - 1 ≤ SCLDEL

Refer to the following table for tf, tr, tHD;DAT, tVD;DAT , and tSU;DAT standard values.
Use the SDA and SCL real transition time values measured in the application to widen the
scope of allowed SDADEL[3:0] and SCLDEL[3:0] values. Use the maximum SDA and SCL
transition time values defined in the standard to make the device work reliably regardless of
the application.
Note:

At every clock pulse, after SCL falling edge detection, I2C operating as controller or target
stretches SCL low during at least [(SDADEL + SCLDEL + 1) x (PRESC + 1) + 1] x tI2CCLK, in both
transmission and reception modes. In transmission mode, if the data is not yet written in
I2C_TXDR when SDA delay elapses, the I2C peripheral keeps stretching SCL low until the
next data is written. Then new data MSB is sent on SDA output, and SCLDEL counter starts,
continuing stretching SCL low to guarantee the data setup time.
When the NOSTRETCH bit is set in target mode, the SCL is not stretched. The
SDADEL[3:0] must then be programmed so that it ensures a sufficient setup time.
Table 660. I²C-bus and SMBus specification data setup and hold times

Symbol

Parameter

Standard-mode
(Sm)

Fast-mode
(Fm)

Fast-mode Plus
(Fm+)

Min

Max

Min

Max

Min

Max

Min

Max

SMBus
Unit

tHD;DAT

Data hold time

0

-

0

-

0

-

0.3

-

tVD;DAT

Data valid time

-

3.45

-

0.9

-

0.45

-

-

tSU;DAT

Data setup time

250

-

100

-

50

-

250

-

tr

Rise time of both
SDA and SCL signals

-

1000

-

300

-

120

-

1000

tf

Fall time of both
SDA and SCL signals

-

300

-

300

-

120

-

300

RM0456 Rev 6

µs

ns

<!-- pagebreak -->

