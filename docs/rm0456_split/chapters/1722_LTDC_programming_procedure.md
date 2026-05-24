1741

LCD-TFT display controller (LTDC)

RM0456

These interrupt events are connected to the NVIC controller as described in the figure
below.
Figure 411. Interrupt events
Line
LTDC global interrupt

Register reload
FIFO underrun

LTDC global error interrupt

Transfer error

MS19678V1

Table 433. LTDC interrupt requests
Interrupt event

43.6

Event flag

Enable control bit

Line

LIF

LIE

Register reload

RRIF

RRIEN

FIFO underrun

FUDERRIF

FUDERRIE

Transfer error

TERRIF

TERRIE

LTDC programming procedure
The steps listed below are needed to program the LTDC:

<!-- pagebreak -->

