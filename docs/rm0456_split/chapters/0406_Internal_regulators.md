482

Power control (PWR)

RM0456

VREF- and VREF+ pins are not available on all packages. When not available, they are
bonded to VSSA and VDDA, respectively.
When the VREF+ is double-bonded with VDDA in a package, the internal voltage
reference buffer is not available and must be kept disabled.
VREF- must always be equal to VSSA.

10.4.2

Internal regulators
The devices embed two regulators: one LDO and one SMPS in parallel to provide
the VCORE supply for digital peripherals, SRAMs (except BKPSRAM) and embedded flash
memory. The SMPS generates this voltage on VDD11 (two or three pins) with a total
external capacitor of 4.7 μF typical and requires an external coil of 2.2 µH typical.
The LDO generates this voltage on VCAP (one or two pins depending on packages)
with a total of external capacitor of 4.7 μF typical.
Both regulators can provide four different voltages (voltage scaling) and can operate
in Stop mode.
It is possible to switch from SMPS to LDO and from LDO to SMPS on-the-fly.

10.4.3

Power-up and power-down power sequences
During power-up and power-down phases, the following power sequence requirements
must be respected:
•

When VDD is below 1 V, other power supplies (VDDA, VDDIO2, VDDUSB) must remain
below VDD + 300 mV.

•

When VDD is above 1 V, all power supplies are independent.

During the power-down phase, VDD can temporarily become lower than other supplies only
if the energy provided to the MCU remains below 1 mJ. This allows external decoupling
capacitors to be discharged with different time constants during the power-down transient
phase.

10.4.4

Independent analog peripherals supply
To improve ADC and DAC conversion accuracy and to extend the supply flexibility, the
analog peripherals have an independent power supply that can be separately filtered and
shielded from noise on the PCB:
•

The analog peripherals voltage supply input is available on a separate VDDA pin.

•

An isolated supply ground connection is provided on VSSA pin.

The VDDA supply voltage can be different from VDD. The presence of VDDA must be checked
before enabling any of the analog peripherals supplied by VDDA (A/D converter, D/A
converter, comparators, operational amplifiers, voltage reference buffer).
After reset, the ADC and analog switch control supplied by VDDA are logically and
electrically isolated and therefore are not available. The isolation must be removed before
using the analog peripherals, by setting the ASV bit in the PWR_SVMCR register, once the
VDDA supply is present.
The VDDA supply can be monitored by the analog voltage monitors (AVM), and compared
with two thresholds (1.6 V for AVM1 or 1.8 V for AVM2), refer to Section 10.6.3 for more
details.

<!-- pagebreak -->

