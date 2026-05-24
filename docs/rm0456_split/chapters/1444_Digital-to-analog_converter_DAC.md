1443

Digital-to-analog converter (DAC)

RM0456

35

Digital-to-analog converter (DAC)

35.1

Introduction
The DAC module is a 12-bit, voltage output digital-to-analog converter. The DAC can be
configured in 8- or 12-bit mode and may be used in conjunction with the DMA controller.
In 12-bit mode, the data can be left- or right-aligned. The DAC features two output channels,
each with its own converter. In dual DAC channel mode, conversions can be done
independently or simultaneously when both channels are grouped together for synchronous
update operations. An input reference pin, VREF+ (shared with others analog peripherals) is
available for better resolution. An internal reference can also be set on the same input.
Refer to voltage reference buffer (VREFBUF) section.
The DACx_OUTy pin can be used as general purpose input/output (GPIO) when the DAC
output is disconnected from output pad and connected to on chip peripheral. The DAC
output buffer can be optionally enabled to obtain a high drive output current. An individual
calibration can be applied on each DAC output channel. The DAC output channels support
a low power mode, the sample and hold mode.

35.2

DAC main features
The DAC main features are the following (see Figure 308: Dual-channel DAC block
diagram)
•

One DAC interface, maximum two output channels

•

Left or right data alignment in 12-bit mode

•

Synchronized update capability

•

Noise-wave and Triangular-wave generation

•

Dual DAC channel for independent or simultaneous conversions

•

DMA capability for each channel including DMA underrun error detection

•

Double data DMA capability to reduce the bus activity

•

External triggers for conversion

•

DAC output channel buffered/unbuffered modes

•

Buffer offset calibration

•

Each DAC output can be disconnected from the DACx_OUTy output pin

•

DAC output connection to on-chip peripherals

•

Sample and hold mode for low power operation in Stop mode

•

Autonomous mode to reduce the power consumption for the system

•

Input voltage reference from VREF+ pin or internal VREFBUF reference

Figure 308 shows the block diagram of a DAC channel and Table 341 gives the pin
description.

<!-- pagebreak -->

