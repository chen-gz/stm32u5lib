Signal name

Signal type

Description

jpeg_hclk

Digital input

JPEG kernel and register interface clock

jpeg_it

Digital output

JPEG global interrupt

jpeg_ift_trg

Digital output

JPEG input FIFO threshold for DMA trigger

jpeg_ifnf_trg

Digital output

JPEG input FIFO not full for DMA trigger

jpeg_oft_trg

Digital output

JPEG output FIFO threshold for DMA trigger

RM0456 Rev 6

RM0456

JPEG codec (JPEG)
Table 452. JPEG internal signals (continued)
Signal name

Signal type

Description

jpeg_ofne_trg

Digital output

JPEG output FIFO not empty for DMA trigger

jpeg_eoc_trg

Digital output

JPEG end of conversion for DMA trigger

Table 453. JPEG trigger connections

46.3.3

Trigger name

Direction

Trigger source/destination

jpeg_eoc_trg

Output

gpdma_trigsel[63]

jpeg_ifnf_trg

Output

gpdma_trigsel[64]

jpeg_ift_trg

Output

gpdma_trigsel[65]

jpeg_ofne_trg

Output

gpdma_trigsel[66]

jpeg_oft_trg

Output

gpdma_trigsel[67]

JPEG decoding procedure
The JPEG codec can decode a JPEG stream as defined in the ISO/IEC 10918-1
specification.
It can optionally parse the JPEG header and accordingly update the JPEG codec registers,
the quantization tables, and the Huffman tables.
The JPEG codec is configured in decode mode setting the DE bit (decode enable) of the
JPEG_CONFR1 register.
The JPEG decode starts by setting the START bit of the JPEG_CONFR0 register.
The JPEG codec requests data for its input FIFO through generating one of:
•

DMA request

•

DMA trigger

•

Interrupts

DMA generation for input FIFO
A DMA request is generated when the 32-byte input FIFO becomes at least half-empty, that
is, when there is free room for writing 16 bytes of data.
The DMA request generation is independent of the START bit of the JPEG_CONFR0
register. If the input FIFO can accept 16 bytes and the DMA for the input FIFO is enabled
(setting the IDMAEN bit of the JPEG_CR register), a DMA request is generated regardless
of the state of the JPEG codec kernel.
A burst transfer is launched by the DMA to write 16 bytes of data.
Writes are ignored if the input FIFO is full.
At the end of the decoding process, extra bytes may remain in the input FIFO and/or a DMA
request may be pending. The FIFO can be flushed by setting the IFF bit (input FIFO flush) of
the JPEG_CR register.
Prior to flushing the FIFO, the DMA for the input FIFO must be disabled to prevent
unwanted DMA request upon flushing the FIFO.

RM0456 Rev 6

<!-- pagebreak -->

1886

JPEG codec (JPEG)

RM0456

The consequence of not flushing the FIFO at the end of the decoding process is that any
remaining data is taken into the next JPEG decoding.
DMA requests are no more generated once the EOCF flag of the JPEG_SR register is set.

Interrupt or DMA trigger generation for input FIFO
Input FIFO can be managed using interrupts or DMA triggers through two flags according to
the FIFO state:
•

Input FIFO not full flag: a 32-bit value can be written in.

•

Input FIFO threshold flag: 8 words (32 bytes) can be written in.

The interrupt or DMA trigger generation is independent of the START bit of the
JPEG_CONFR0 register. The input FIFO flags are generated regardless of the state of the
JPEG codec kernel.
Writes are ignored if the input FIFO is full.
At the end of the decoding process, extra bytes may remain in the input FIFO and/or an
interrupt request / DMA trigger may be pending. The FIFO can be flushed by setting the IFF
bit (Input FIFO Flush) of the JPEG_CR register.
Prior to flushing the FIFO:
•

The interrupts for the input FIFO must be disabled to prevent an unwanted interrupt
request upon flushing the FIFO.

•

The DMA channel must be stopped to prevent an unwanted DMA trigger.

The consequence of not flushing the FIFO at the end of the decoding process is that any
remaining data is taken into the next JPEG decoding.

Header parsing
The header parsing can be activated by setting the HDR bit of the JPEG_CONFR1 register.
The JPEG header parser supports all markers relevant to the JPEG baseline algorithm
indicated in Annex B of the ISO/IEC 10918-1.
When parsing a supported marker, the JPEG header parser extracts the required
parameters and stores them in shadow registers. At the end of the parsing the JPEG codec
registers are updated.
If a DQT marker segment is located, quantization data associated with it is written into the
quantization table memory.
If a DHT marker segment is located, the Huffman table data associated with it is converted
into three different table formats (HuffMin, HuffBase, and HuffSymb) and stored in their
respective memories.
Once the parsing operation is completed, the HPDF (header parsing done flag) bit of the
JPEG_SR register is set. An interrupt is generated if the EHPIE (end of header parsing
interrupt enable) bit of the JPEG_CR register is set.

JPEG decoding
Once the JPEG header is parsed or the JPEG codec registers and memories are properly
programmed, the incoming data stream is decoded and the resulting MCUs are sent to the
output FIFO.

<!-- pagebreak -->

