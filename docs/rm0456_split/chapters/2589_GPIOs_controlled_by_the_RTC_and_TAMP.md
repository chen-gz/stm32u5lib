2756

Inter-integrated circuit interface (I2C)

RM0456

Communication flow
In controller mode, the I2C peripheral initiates a data transfer and generates the clock
signal. Serial data transfers always begin with a START condition and end with a STOP
condition. Both START and STOP conditions are generated in controller mode by software.
In target mode, the peripheral recognizes its own 7-bit or 10-bit address, and the general
call address. The general call address detection can be enabled or disabled by software.
The reserved SMBus addresses can also be enabled by software.
Data and addresses are transferred as 8-bit bytes, MSB first. The address is contained in
the first byte (7-bit addressing) or in the first two bytes (10-bit addressing) following the
START condition. The address is always transmitted in controller mode.
The following figure shows the transmission of a single byte. The controller generates nine
SCL pulses. The transmitter sends the eight data bits to the receiver with the SCL pulses 1
to 8. Then the receiver sends the acknowledge bit to the transmitter with the ninth SCL
pulse.
Figure 780. I²C-bus protocol

SDA
MSB

ACK

SCL
1

2

Start
condition

8

9

Stop
condition
MS19854V1

The acknowledge can be enabled or disabled by software. The own addresses of the I2C
peripheral can be selected by software.

65.4.5

I2C initialization
Enabling and disabling the peripheral
Before enabling the I2C peripheral, configure and enable its clock through the RCC, and
initialize its control registers.
The I2C peripheral can then be enabled by setting the PE bit of the I2C_CR1 register.
Disabling the I2C peripheral by clearing the PE bit resets the I2C peripheral. Refer to
Section 65.4.6 for more details.

Noise filters
Before enabling the I2C peripheral by setting the PE bit of the I2C_CR1 register, the user
must configure the analog and/or digital noise filters, as required.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Inter-integrated circuit interface (I2C)
The analog noise filter on the SDA and SCL inputs complies with the I²C-bus specification
which requires, in Fast-mode and Fast-mode Plus, the suppression of spikes shorter than
50 ns. Enabled by default, it can be disabled by setting the ANFOFF bit.
The digital filter is controlled through the DNF[3:0] bitfield of the I2C_CR1 register. When it is
enabled, the internal SCL and SDA signals only take the level of their corresponding I²C-bus
line when remaining stable for more than DNF[3:0] periods of i2c_ker_ck. This allows
suppressing spikes shorter than the filtering capacity period programmable from one to
fifteen i2c_ker_ck periods.
The following table compares the two filters.
Table 659. Comparison of analog and digital filters
Item
Filtering capacity

Analog filter
(1)

≥ 50 ns

Digital filter
One to fifteen i2c_ker_ck periods
– Programmable filtering capacity
– Extra filtering capability versus I²C-bus
specification requirements
– Stable filtering capacity

Benefits

Available in Stop mode

Drawbacks

Filtering capacity
variation with
Functionality in Stop mode not supported when the
temperature, voltage, and digital filter is enabled
silicon process

1. Maximum duration of spikes that the filter can suppress

Caution:

The filter configuration cannot be changed when the I2C peripheral is enabled.

I2C timings
To ensure correct data hold and setup times, the corresponding timings must be configured
through the PRESC[3:0], SCLDEL[3:0], and SDADEL[3:0] bitfields of the I2C_TIMINGR
register.
The STM32CubeMX tool calculates and provides the I2C_TIMINGR content in the I2C
configuration window.

RM0456 Rev 6

<!-- pagebreak -->

2756

Inter-integrated circuit interface (I2C)

RM0456
Figure 781. Setup and hold timings

DATA HOLD TIME
SCL falling edge internal
detection

tSYNC1 SDADEL: SCL stretched low by the I2C

SDA output delay
SCL

SDA

tHD;DAT
Data hold time: in case of transmission, the data is sent on SDA output after
the SDADEL delay, if it is already available in I2C_TXDR.
DATA SETUP TIME
SCLDEL
SCL stretched low by the I2C

SCL

SDA

tSU;STA
Data setup time: in case of transmission, the SCLDEL counter starts
when the data is sent on SDA output.
MSv40108V1

When the SCL falling edge is internally detected, the delay tSDADEL (impacting the hold time
tHD;DAT) is inserted before sending SDA output:
tSDADEL = SDADEL x tPRESC + tI2CCLK, where tPRESC = (PRESC + 1) x tI2CCLK.

The total SDA output delay is:
tSYNC1 + {[SDADEL x (PRESC + 1) + 1] x tI2CCLK}

The tSYNC1 duration depends upon:
•

SCL falling slope

•

input delay tAF(min) < tAF < tAF(max) introduced by the analog filter (if enabled)

•

input delay tDNF = DNF x tI2CCLK introduced by the digital filter (if enabled)

•

delay due to SCL synchronization to i2c_ker_ck clock (two to three i2c_ker_ck periods)

To bridge the undefined region of the SCL falling edge, the user must set SDADEL[3:0] so
as to fulfill the following condition:
{tf(max) + tHD;DAT(min) - tAF(min) - [(DNF + 3) x tI2CCLK]} / {(PRESC + 1) x tI2CCLK} ≤ SDADEL
SDADEL ≤ {tHD;DAT (max) - tAF(max) - [(DNF + 4) x tI2CCLK]} / {(PRESC + 1) x tI2CCLK}

<!-- pagebreak -->

