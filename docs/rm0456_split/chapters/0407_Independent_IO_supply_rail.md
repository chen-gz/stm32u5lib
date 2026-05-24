RM0456 Rev 6

RM0456

Power control (PWR)
When a single supply is used, VDDA can be externally connected to VDD through the
external filtering circuit in order to ensure a noise-free VDDA reference voltage.

ADC and DAC reference voltage
To ensure a better accuracy on low-voltage inputs and outputs, the user can connect to
VREF+, a separate reference voltage lower than VDDA. VREF+ is the highest voltage,
represented by the full scale value, for an analog input (ADC) or output (DAC) signal.
VREF+ can be provided either by an external reference of by an internal buffered voltage
reference (VREFBUF). The internal voltage reference can output a configurable voltage:
1.5 V, 1.8 V, 2.048 V or 2.4 V. The internal voltage reference can also provide the voltage to
external components through VREF+ pin. Refer to the device datasheet and to Section 36:
Voltage reference buffer (VREFBUF) for further information.

10.4.5

Independent I/O supply rail
Some I/Os from port G (PG[15:2]) are supplied from a separate supply rail. The power
supply for this rail can range from 1.08 V to 3.6 V and is provided externally through the
VDDIO2 pin. The VDDIO2 voltage level is completely independent from VDD or VDDA. The
VDDIO2 pin is available only for some packages. Refer to the pinout diagrams or tables in
the related device datasheet(s) for I/O list(s).
After reset, the I/Os supplied by VDDIO2 are logically and electrically isolated and therefore
are not available. The isolation must be removed before using any I/O from PG[15:2], by
setting the IO2SV bit in the PWR_SVMCR register, once the VDDIO2 supply is present.
The VDDIO2 supply is monitored by the IO2 voltage monitoring (IO2VM) and compared with
the internal reference voltage (3/4 VREFINT, around 0.9 V). Refer to Section 10.6.3 for more
details.

10.4.6

Independent USB transceivers supply
The USB transceivers are supplied from a separate VDDUSB power supply pin.
The VDDUSB range is from 3.0 V to 3.6 V and is completely independent from VDD or VDDA.
After reset, the USB features supplied by VDDUSB are logically and electrically isolated, and
therefore are not available. The isolation must be removed before using
USB/OTG_FS/OTG_HS, by setting USV in PWR_SVMCR, once the VDDUSB supply is
present.
The VDDUSB supply is monitored by the USB voltage monitoring (UVM), and compared with
the internal reference voltage (VREFINT, around 1.2 V). Refer to Section 10.6.3 for more
details.

Internal OTG_HS transceiver supply (STM32U59x/5Ax/5Fx/5Gx only)
The OTG_HS high-speed transceiver is functional in voltage scaling range 1 and range 2.
The USB EPOD (embedded power distribution) booster must be enabled and ready before
using the OTG_HS: USBPWREN and USBBOOSTEN bits must be set to one in
PWR_VOSR (refer to Section 10.6.3 and Section 10.7.12 for more details).

RM0456 Rev 6

<!-- pagebreak -->

