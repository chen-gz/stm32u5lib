1696

Digital camera interface (DCMI)

41.3.1

RM0456

DCMI block diagram
Figure 392 shows the DCMI block diagram.
Figure 392. DCMI block diagram
DCMI

DMA
interface

32-bit AHB Bus

dcmi_dma

Control/Status register

FIFO/Data
formatter

Data
extraction

Synchronizer

DCMI_PIXCLK
DCMI_D[13:0]
DCMI_HSYNC
DCMI_VSYNC

dcmi_it
dcmi_hclk

MSv43767V2

41.3.2

DCMI pins and internal signals
The following table shows DCMI pins.
Table 410. DCMI input/output pins
Mode

Pin name

Signal type

Description

8 bits
10 bits
12 bits
14 bits

DCMI_D[7:0]
DCMI_D[9:0]
DCMI_D[11:0]
DCMI_D[13:0]

Inputs

DCMI data

DCMI_PIXCLK

Input

Pixel clock

DCMI_HSYNC

Input

Horizontal synchronization / Data valid

DCMI_VSYNC

Input

Vertical synchronization

The following table shows DCMI internal signals.
Table 411. DCMI internal input/output signals

<!-- pagebreak -->

