3086

FD controller area network (FDCAN)

RM0456

Test modes
To enable write access to FDCAN test register (FDCAN_TEST), the TEST bit of the
FDCAN_CCCR register must be set, thus enabling the configuration of test modes and
functions.
Four output functions are available for the CAN transmit pin FDCAN_TX by programming
the TX[1:0] bitfield of the FDCAN_TEST register. In addition to its default function (the serial
data output), it can drive the CAN sample point signal to monitor the FDCAN bit timing as
well as drive constant dominant or recessive values. The actual value at pin FDCAN_RX
can be read from the RX bit of FDCAN_TEST. Both functions can be used to check the CAN
bus physical layer.
Due to the synchronization mechanism between CAN kernel clock and APB clock domain,
there can be a delay of several APB clock periods between writing to TX[1:0] until the new
configuration is visible at FDCAN_TX output pin. This applies also when reading
FDCAN_RX input pin via RX.
Note:

Test modes must be used for production tests or self-test only. The software control for
FDCAN_TX pin interferes with all CAN protocol functions. It is not recommended to use test
modes for application.

External loop-back mode
The FDCAN can be set in external loop-back mode by setting the LBCK bit of the
FDCAN_TEST register. In loop-back mode, the FDCAN treats its own transmitted
messages as received messages and stores them (if they pass acceptance filtering) into Rx
FIFOs. Figure 886 shows the connection of transmit and receive signals FDCAN_TX and
FDCAN_RX to the FDCAN in external loop-back mode.
This mode is provided for hardware self-test. To be independent from external stimulation,
the FDCAN ignores acknowledge errors (recessive bit sampled in the acknowledge slot of a
data/remote frame) in loop-back mode. In this mode, the FDCAN performs an internal
feedback from its transmit output to its receive input. The actual value of the FDCAN_RX
input pin is disregarded by the FDCAN. The transmitted messages can be monitored at the
FDCAN_TX transmit pin.

Internal loop-back mode
Internal loop-back mode is entered by setting both the LBCK bit of FDCAN_TEST and the
MON bit of FDCAN_CCR. This mode can be used for a “hot self-test”, meaning the FDCAN
can be tested without affecting a running CAN system connected to the FDCAN_TX and
FDCAN_RX pins. In this mode, FDCAN_RX pin is disconnected from the FDCAN and
FDCAN_TX pin is held recessive. Figure 886 shows the connection of FDCAN_TX and
FDCAN_RX pins to the FDCAN in case of internal loop-back mode.

<!-- pagebreak -->

