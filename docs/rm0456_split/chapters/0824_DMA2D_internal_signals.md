856

Chrom-ART Accelerator controller (DMA2D)

RM0456

The block diagram of the DMA2D is shown in the figure below.
Figure 90. DMA2D block diagram
Output PFC

color mode
16/24/32

FIFO

32-bit
AHB bus

32

Converter

RGB

AHB
Master
Foreground PFC

mode

color mode

dma2d_hclk
FIFO

Expander
256x32-bit
CLUT

32

8

X

32

dma2d_it
dma2d_tc

8-bit

ɲ

8

RGB

24

32

RGB
32

Blender

dma2d_ctc
Background PFC

dma2d_tw

mode

color mode

32-bit
AHB bus

AHB
Slave

FIFO

Expander
256x32-bit
CLUT

19.3.2

8-bit

32

32
X

32
ɲ

8

RGB

24

8

32

RGB

DMA2D internal signals
The internal signals of the DMA2D are given in the table below.
Table 157. DMA2D internal signals
Names

Signal type

Description

dma2d_hclk

Input

DMA2D AHB clock

dma2d_it

Output

DMA2D global interrupt request

dma2d_tc

Output

DMA2D transfer complete trigger

dma2d_ctc

Output

DMA2D CLUT transfer complete trigger

dma2d_tw

Output

DMA2D transfer watermark trigger

The table below shows the way the flags of the DMA2D are connected.
Table 158. DMA2D trigger interconnection(1)
Trigger name

Direction

Trigger source/destination

dma2d_tc

Output

gpdma_trigsel[50]

dma2d_ctc

Output

gpdma_trigsel[51]

dma2d_tw

Output

gpdma_trigsel[52]

1. Only available on STM32U59x/5Ax products.

<!-- pagebreak -->

