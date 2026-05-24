1485

Digital-to-analog converter (DAC)

35.4.12

RM0456

DAC channel modes
Each DAC channel can be configured in normal mode or sample and hold mode. The output
buffer can be enabled to obtain a high drive capability. Before enabling output buffer, the
voltage offset needs to be calibrated. This calibration is performed at the factory (loaded
after reset) and can be adjusted by software during application operation.

Normal mode
In normal mode, there are four combinations, by changing the buffer state and by changing
the DACx_OUTy pin interconnections.
To enable the output buffer, the MODEx[2:0] bits in DAC_MCR register must be:
•

000: DAC is connected to the external pin

•

001: DAC is connected to external pin and to on-chip peripherals

To disable the output buffer, the MODEx[2:0] bits in DAC_MCR register must be:
•

010: DAC is connected to the external pin

•

011: DAC is connected to on-chip peripherals

Sample and hold mode
In sample and hold mode, the DAC core converts data on a triggered conversion, and then
holds the converted voltage on a capacitor. When not converting, the DAC cores and buffer
are completely turned off between samples and the DAC output is tri-stated, therefore
reducing the overall power consumption. A stabilization period, which value depends on the
buffer state, is required before each new conversion.
In this mode, the DAC core and all corresponding logic and registers are driven by the
LSI/LSE low-speed clock (dac_hold_ck) in addition to the dac_hclk clock, allowing using the
DAC channels in deep low power modes such as Stop mode.
The LSI/LSE low-speed clock (dac_hold_ck) must not be stopped when the sample and hold
mode is enabled.
The sample/hold mode operations can be divided into three phases:

<!-- pagebreak -->

1.

Sample phase: the sample/hold element is charged to the desired voltage. The
charging time depends on capacitor value (internal or external, selected by the user).
The sampling time is configured with the TSAMPLEx[9:0] bits in DAC_SHSRx register.
During the write of the TSAMPLEx[9:0] bits, the BWSTx bit in DAC_SR register is set to
1 to synchronize between both clocks domains (AHB and low speed clock) and
allowing the software to change the value of sample phase during the DAC channel
operation

2.

Hold phase: the DAC output channel is tri-stated, the DAC core and the buffer are
turned off, to reduce the current consumption. The hold time is configured with the
THOLDx[9:0] bits in DAC_SHHR register

3.

Refresh phase: the refresh time is configured with the TREFRESHx[7:0] bits in
DAC_SHRR register

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
The timings for the three phases above are in units of LSI/LSE clock periods. As an
example, to configure a sample time of 350 µs, a hold time of 2 ms and a refresh time of
100 µs assuming LSI/LSE ~32 KHz is selected:
•

12 cycles are required for sample phase: TSAMPLEx[9:0] = 11.

•

62 cycles are required for hold phase: THOLDx[9:0] = 62.

•

and 4 cycles are required for refresh period: TREFRESHx[7:0] = 4.

In this example, the power consumption is reduced by almost a factor of 15 versus normal
modes.
The formulas to compute the right sample and refresh timings are described in the table
below, the Hold time depends on the leakage current.
Table 346. Sample and refresh timings
Buffer
State

tSAMP(1)(2)

tREFRESH(2)(3)

Enable

7 μs + (10*RBON*CSH)

7 μs + (RBON*CSH)*ln(2*NLSB)

Disable

3 μs + (10*RBOFF*CSH)

3 μs + (RBOFF*CSH)*ln(2*NLSB)

1. In the above formula, the settling to the desired code value with ½ LSB or accuracy requires 10 constant
time for 12 bits resolution. For 8-bit resolution, the settling time is 7 constant time.
2. CSH is the capacitor in sample and hold mode.
3. The tolerated voltage drop during the hold phase “Vd” is represented by the number of LSBs after the
capacitor discharging with the output leakage current. The settling back to the desired value with ½ LSB
error accuracy requires ln(2*Nlsb) constant time of the DAC.

Example of the sample and refresh time calculation with output buffer on
The values used in the example below are provided as indication only. Refer to the product
datasheet for product data.
CSH = 100 nF
VREF+ = 3.0 V
Sampling phase:
tSAMP = 7 μs + (10 * 2000 * 100 * 10-9) = 2.007 ms
(where RBON = 2 kΩ)
Refresh phase:
tREFRESH = 7 μs + (2000 * 100 * 10-9) * ln(2*10) = 606.1 μs
(where NLSB = 10 (10 LSB drop during the hold phase)
Hold phase:
Dv = ileak * thold / CSH = 0.0073 V (10 LSB of 12bit at 3 V)
ileak = 150 nA (worst case on the IO leakage on all the temperature range)
thold = 0.0073 * 100 * 10-9 / (150 * 10-9) = 4.867 ms

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

Figure 316. DAC sample and hold mode phase diagram

V1
Vd
V2

t
dac_hold
_ck

Sampling phase

Hold phase

Refresh
phase

Sampling phase

t

DAC

ON

ON

ON
MSv45340V3

Like in normal mode, the sample and hold mode has different configurations.
To enable the output buffer, MODEx[2:0] bits in DAC_MCR register must be set to:
•

100: DAC is connected to the external pin

•

101: DAC is connected to external pin and to on chip peripherals

To disabled the output buffer, MODEx[2:0] bits in DAC_MCR register must be set to:
•

110: DAC is connected to external pin and to on chip peripherals

•

111: DAC is connected to on chip peripherals
When MODEx[2:0] bits are equal to 111, an internal capacitor, CLint, holds the voltage
output of the DAC core and then drive it to on-chip peripherals.

All sample and hold phases are interruptible, and any change in DAC_DHRx immediately
triggers a new sample phase.
Table 347. Channel output modes summary
MODEx[2:0]
0

0

0

0

0

1

0

1

0

0

1

1

<!-- pagebreak -->

Mode

Buffer

Output connections
Connected to external pin

Enabled
Normal mode
Disabled

Connected to external pin and to on chip-peripherals (such as
comparators)
Connected to external pin
Connected to on chip peripherals (such as comparators)

RM0456 Rev 6

RM0456

Digital-to-analog converter (DAC)
Table 347. Channel output modes summary (continued)

MODEx[2:0]
1

0

0

1

0

1

1

1

0

1

1

1

35.4.13

Mode

Buffer

Output connections
Connected to external pin

Enabled

Connected to external pin and to on chip peripherals (such as
comparators)

Sample and
hold mode

Connected to external pin and to on chip peripherals (such as
comparators)

Disabled

Connected to on chip peripherals (such as comparators)

DAC channel buffer calibration
The transfer function for an N-bit digital-to-analog converter (DAC) is:
V

out

⎞ +V
= ⎛ ( D ⁄ 2N ) × G × V
⎝
OS
REF⎠

Where VOUT is the analog output, D is the digital input, G is the gain, VREF is the nominal
full-scale voltage, and VOS is the offset voltage. For an ideal DAC channel, G = 1 and
VOS = 0.
Due to output buffer characteristics, the voltage offset may differ from part-to-part and
introduce an absolute offset error on the analog output. To compensate the VOS, a
calibration is required by a trimming technique.
The calibration is only valid when the DAC channelx is operating with buffer enabled
(MODEx[2:0] = 0b000 or 0b001 or 0b100 or 0b101). if applied in other modes when the
buffer is off, it has no effect. During the calibration:
•

The buffer output is disconnected from the pin internal/external connections and put in
tristate mode (HiZ).

•

The buffer acts as a comparator to sense the middle-code value 0x800 and compare it
to VREF+/2 signal through an internal bridge, then toggle its output signal to 0 or 1
depending on the comparison result (CAL_FLAGx bit).

Two calibration techniques are provided:
•

Factory trimming (default setting)
The DAC buffer offset is factory trimmed. The default value of OTRIMx[4:0] bits in
DAC_CCR register is the factory trimming value and it is loaded once DAC digital
interface is reset.

•

User trimming
The user trimming can be done when the operating conditions differs from nominal
factory trimming conditions and in particular when VDDA voltage, temperature, VREF+
values change and can be done at any point during application by software.

Note:

Refer to the datasheet for more details of the nominal factory trimming conditions.
In addition, when VDD is removed (example the device enters in Standby or VBAT modes)
the calibration is required.

RM0456 Rev 6

<!-- pagebreak -->

1485

Digital-to-analog converter (DAC)

RM0456

The steps to perform a user trimming calibration are as below:
1.

If the DAC channel is active, write 0 to ENx bit in DAC_CR to disable the channel.

2.

Select a mode where the buffer is enabled, by writing to DAC_MCR register,
MODEx[2:0] = 0b000 or 0b001 or 0b100 or 0b101.

3.

Start the DAC channelx calibration, by setting the CENx bit in DAC_CR register to 1.

4.

Apply a trimming algorithm:
a)

Write a code into OTRIMx[4:0] bits, starting by 0b00000.

b)

Wait for tTRIM delay.

c)

Check if CAL_FLAGx bit in DAC_SR is set to 1.

d)

Until the CAL_FLAGx is read as 1 or the maximum trimming code is reached,
increment OTRIMx[4:0] and repeat substeps from (b) to (d).

The software algorithm may use either a successive approximation or dichotomy techniques
to compute and set the content of OTRIMx[4:0] bits in a faster way.
Note:

A tTRIM delay must be respected between the write to the OTRIMx[4:0] bits and the read of
the CAL_FLAGx bit in DAC_SR register in order to get a correct value.This parameter is
specified into datasheet electrical characteristics section.
If VDDA, VREF+ and temperature conditions do not change during device operation while it
enters more often in Standby and VBAT modes, the software may store the OTRIMx[4:0]
bits found in the first user calibration in the flash or in back-up registers. then to load/write
them directly when the device power is back again thus avoiding to wait for a new calibration
time.
When CENx bit is set, it is not allowed to set ENx bit.

35.4.14

DAC channel conversion modes
Four conversion modes are possible.

Independent trigger without wave generation
To configure the DAC in this conversion mode, the following sequence is required:
1.

Set the DAC channel trigger enable bit, TENx.

2.

Configure the trigger sources by setting different values in the TSELx[3:0] bits.

3.

Load the DAC channel data into the desired DHR registers (DAC_DHR12R1,
DAC_DHR12L1 or DAC_DHR8R1).

When a DAC channel trigger arrives, the DHRx register is transferred into DAC_DORx
(three dac_hclk clock cycles later).

Independent trigger with single LFSR generation
To configure the DAC in this conversion mode, the following sequence is required:

<!-- pagebreak -->

