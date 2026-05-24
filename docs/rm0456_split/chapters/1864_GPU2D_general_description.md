1864

Neo-Chrom graphic processor (GPU2D)

RM0456

45.4

GPU2D general description

45.4.1

GPU2D block diagram
Figure 449. GPU2D block diagram
32-bit
AHB bus

Bus interface unit
32-bit
AHB bus

Command
list
processor

gpu2d_hclk
gpu2d_irq

Texture map unit
Render
output
unit

gpu2d_irqsys
gpu2d_flag[3:0]

Rasterizer

Register
file

32-bit
AHB bus

Fragment
processing
core

Graphic pipeline
MSv65699V1

45.4.2

GPU2D pins and internal signals
The internal signals of the GPU2D peripheral are given in the following table.
Table 450. GPU2D internal input/output signals
Internal signal name

Signal type

Description

gpu2d_hclk

Input

GPU2D AHB clock

gpu2d_irq

Output

GPU2D interrupt request

gpu2d_irqsys

Output

GPU2D system interrupt request

gpu2d_flag[3:0]

Output

GPU2D general purpose flags

Table 451. GPU2D trigger connections

<!-- pagebreak -->

