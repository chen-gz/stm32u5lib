RM0456 Rev 6

RM0456

JPEG codec (JPEG)
When decoding two images successively, the START bit of the JPEG_CONFR0 register
must be set again (even if already 1) after the header processing of the second image is
completed.

DMA generation for output FIFO
A DMA request is generated when the 32-byte output FIFO becomes at least half-full, that
is, when there are at least 16 bytes of data.
A burst transfer is launched by the DMA to read 16 bytes of data.
Reads return 0 if the output FIFO is empty.
Once the decoding process is done, no extra bytes must remain in the output FIFO and no
DMA request must be pending as the JPEG decoding generates blocks of 64 bytes.
In case of abort of the JPEG codec operations by resetting the START bit of the
JPEG_CONFR0 register, the output FIFO can be flushed by setting the OFF bit (input FIFO
flush) of the JPEG_CR register.
Prior to flushing the FIFO, the DMA for the output FIFO must be disabled to prevent an
unwanted DMA request upon flushing the FIFO.

Interrupt or DMA trigger generation for output FIFO
The output FIFO can be managed using interrupts or DMA triggers through two flags
according to the FIFO state:
•

Output FIFO not empty flag: a 32-bit value can be read out.

•

Output FIFO threshold flag: 8 words (32 bytes) can be read out.

Reads return 0 if the output FIFO is empty.
In case of abort of the JPEG codec operations by resetting the START bit of the
JPEG_CONFR0 register, the output FIFO can be flushed. If the FIFO needs to be flushed, it
must be done by software setting the FF bit (FIFO flush) of the JPEG_CR register.
Prior to flushing the FIFO:
•

The interrupts for the output FIFO must be disabled to prevent unwanted interrupt
request upon flushing the FIFO.

•

The DMA channel must be stopped to prevent an unwanted DMA trigger.

The output FIFO must be flushed at the end of processing before any JPEG configuration
change.

46.3.4

JPEG encoding procedure
The JPEG codec can encode a JPEG stream as defined in the ISO/IEC 10918-1
specification.
It can optionally generate the JPEG header.
The JPEG codec is configured in encode mode resetting the DE bit (decode enable) of the
JPEG_CONFR1 register.
The configuration used for encoding the JPEG must be loaded in the JPEG codec:
•

JPEG codec configuration registers

•

Quantization tables

•

Huffman tables
RM0456 Rev 6

<!-- pagebreak -->

1886

JPEG codec (JPEG)

RM0456

The JPEG codec is started setting the START bit of the JPEG_CONFR0 register.
Once the JPEG codec has been started, it requests data for its input FIFO generating one
of:
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
At the end of the encoding process, extra bytes may remain in the input FIFO and/or a DMA
request may be pending. The FIFO can be flushed by setting the IFF bit (input FIFO flush) of
the JPEG_CR register.
Prior to flushing the FIFO, the DMA for the input FIFO must be disabled to prevent
unwanted DMA request upon flushing the FIFO.
The consequence of not flushing the FIFO at the end of the encoding process is that any
remaining data is taken into the next JPEG encoding.
The DMA requests are no more generated once the EOCF flag of the JPEG_SR register is
set.

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
At the end of the encoding process, extra bytes may remain in the input FIFO and/or an
interrupt request / DMA trigger may be pending. The FIFO can be flushed by setting the IFF
bit (input FIFO flush) of the JPEG_CR register.
Prior to flushing the FIFO:

<!-- pagebreak -->

•

The interrupts for the input FIFO must be disabled to prevent an unwanted interrupt
request upon flushing the FIFO.

•

The DMA channel must be stopped to prevent an unwanted DMA trigger.

RM0456 Rev 6

RM0456

JPEG codec (JPEG)
The consequence of not flushing the FIFO at the end of the encoding process is that any
remaining data is taken into the next JPEG encoding.

JPEG encoding
Once the JPEG header is generated, the incoming MCUs are encoded and the resulting
data stream sent to the output FIFO.

DMA generation for output FIFO
A DMA request is generated when the 32-byte output FIFO becomes at least half-full, that
is, when there are at least 16 bytes of data.
A burst transfer is launched by the DMA to read 16 bytes of data.
Read returns 0 if the output FIFO is empty.
At the end of the encoding process, the last bytes may remain in the output FIFO as the
stream padding may not be on 16 bytes.
These additional bytes must be managed by the CPU using the output FIFO not empty flag.
In case of abort of the JPEG codec operations by resetting the START bit of the
JPEG_CONFR0 register, the output FIFO can be flushed. The FIFO can be flushed by
setting the OFF bit (output FIFO flush) of the JPEG_CR register.
Prior to flushing the FIFO, the DMA for the input FIFO must be disabled to prevent an
unwanted DMA request upon flushing the FIFO.

Interrupt or DMA trigger generation for output FIFO
Output FIFO can be managed using interrupts or DMA triggers through two flags according
to the FIFO state:
•

Output FIFO not empty flag: a 32-bit value can be read out.

•

Output FIFO threshold flag: 8 words (32 bytes) can be read out.

Reads return 0 if the output FIFO is empty.
In case of abort of the JPEG codec operations by resetting the START bit of the
JPEG_CONFR0 register, the output FIFO can be flushed. The FIFO can be flushed by
setting the FF bit (FIFO flush) of the JPEG_CR register.
Prior to flushing the FIFO:
•

The interrupts for the output FIFO must be disabled to prevent an unwanted interrupt
request upon flushing the FIFO.

•

The DMA channel must be stopped to prevent an unwanted DMA trigger.

The output FIFO must be flushed at the end of processing before any JPEG configuration
change.
The EOCF bit (end of conversion flag) of the JPEG_SR register can only be cleared when
the output FIFO is empty.
Clearing either of the HDR bit (header processing) of the JPEG_CONFR1 register and the
JCEN bit (JPEG codec enable) of the JPEG_CR register is allowed only when the EOCF bit
of the JPEG_SR register is cleared.

RM0456 Rev 6

<!-- pagebreak -->

