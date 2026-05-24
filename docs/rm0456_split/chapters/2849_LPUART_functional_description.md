3020

Serial audio interface (SAI)
•

•

69.3

RM0456

Interrupt sources when enabled:
–

Errors

–

FIFO requests

2-channel DMA interface

SAI implementation
Table 704. SAI features(1)
SAI1

SAI2(2)

X

X

8 words

8 words

SPDIF

X

X

PDM

X(3)

-

SAI features
I2S, LSB- or MSB-justified, PCM/DSP, TDM, AC’97
FIFO size

1. ‘X’ = supported, ‘-’ = not supported.
2. This instance is not available in STM32U535/545 devices.
3. Only signals D[3:1], and CK[2:1] are available.

<!-- pagebreak -->

