1485

Voltage reference buffer (VREFBUF)

RM0456

36

Voltage reference buffer (VREFBUF)

36.1

Introduction
The devices embed a voltage reference buffer which can be used as voltage reference for
the on-chip ADCs and DACs, and also as voltage reference for external components
through the VREF+ pin.

36.2

VREFBUF implementation
The table below describes the VREFBUF voltages typical values:
Table 351. VREFBUF typical values
Symbol

Value

VREFBUF0

1.5 V

VREFBUF1

1.8 V

VREFBUF2

2.048 V

VREFBUF3

2.5 V

Note:

Refer to the product datasheet for more details.

36.3

VREFBUF functional description
Figure 317. VREFBUF block diagram

VREFINT

+
VREF+
-

VSSA

<!-- pagebreak -->

