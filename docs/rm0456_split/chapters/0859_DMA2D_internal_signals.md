RM0456 Rev 6

RM0456

Chrom-ART Accelerator controller (DMA2D)
Figure 93. DMA2D block diagram
Output PFC
FIFO

Color mode

16/24/32

32

Converter

aRGB

AHB
Master

32-bit
AHB bus

Foreground PFC

a mode

Color mode

dma2d_it
FIFO

Expander

dma2d_tc
256x32-bit
CLUT

dma2d_ctc

8-bit a

32

8

X

32

ɲ

8

RGB

24

32

aRGB
32
32

dma2d_tw
Blender
Background PFC

dma2d_hclk

a mode

color mode

32-bit
AHB bus

AHB
Slave

FIFO

Expander
256x32-bit
CLUT

8-bit a

32

ɲ
32

32
X

8

32

aRGB

8

RGB 24

MSv71100V1

20.3.2

DMA2D internal signals
The internal signals of the DMA2D are given in the table below:
Table 170. DMA2D internal signals
Names

Signal type

Description

dma2d_hclk

Input

DMA2D AHB clock

dma2d_it

Output

DMA2D global interrupt request

dma2d_ctc

Output

DMA2D CLUT transfer complete trigger

dma2d_tc

Output

DMA2D transfer complete trigger

dma2d_tw

Output

DMA2D transfer watermark trigger

Table 171. DMA2D trigger interconnections
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

RM0456 Rev 6

<!-- pagebreak -->

