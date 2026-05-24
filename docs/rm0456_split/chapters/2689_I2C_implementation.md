2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

(0x0). With this value, an interrupt is generated after the 4th received character. The
software must read the LEN field (third byte), its value must be read from the receive buffer.
In interrupt driven receive mode, the length of the block may be checked by software or by
programming the BLEN value. However, before the start of the block, the maximum value of
BLEN (0xFF) may be programmed. The real value is programmed after the reception of the
third character.
If the block is using the LRC longitudinal redundancy check (1 epilogue byte), the
BLEN=LEN. If the block is using the CRC mechanism (2 epilog bytes), BLEN=LEN+1 must
be programmed. The total block length (including prologue, epilogue and information fields)
equals BLEN+4. The end of the block is signaled to the software through the EOBF flag and
interrupt (when EOBIE bit is set).
In case of an error in the block length, the end of the block is signaled by the RTO interrupt
(Character Wait Time overflow).
Note:

The error checking code (LRC/CRC) must be computed/verified by software.

Direct and inverse convention
The smartcard protocol defines two conventions: direct and inverse.
The direct convention is defined as: LSB first, logical bit value of 1 corresponds to a H state
of the line and parity is even. In order to use this convention, the following control bits must
be programmed: MSBFIRST = 0, DATAINV = 0 (default values).
The inverse convention is defined as: MSB first, logical bit value 1 corresponds to an L state
on the signal line and parity is even. In order to use this convention, the following control bits
must be programmed: MSBFIRST = 1, DATAINV =1.
Note:

When logical data values are inverted (0 = H, 1 = L), the parity bit is also inverted in the
same way.
In order to recognize the card convention, the card sends the initial character, TS, as the
first character of the ATR (Answer To Reset) frame. The two possible patterns for the TS
are: LHHL LLL LLH and LHHL HHH LLH.
•

(H) LHHL LLL LLH sets up the inverse convention: state L encodes value 1 and
moment 2 conveys the most significant bit (MSB first). When decoded by inverse
convention, the conveyed byte is equal to '3F'.

•

(H) LHHL HHH LLH sets up the direct convention: state H encodes value 1 and
moment 2 conveys the least significant bit (LSB first). When decoded by direct
convention, the conveyed byte is equal to '3B'.

Character parity is correct when there is an even number of bits set to 1 in the nine
moments 2 to 10.
As the USART does not know which convention is used by the card, it needs to be able to
recognize either pattern and act accordingly. The pattern recognition is not done in
hardware, but through a software sequence. Moreover, supposing that the USART is
configured in direct convention (default) and the card answers with the inverse convention,
TS = LHHL LLL LLH ≥ the USART received character is equal to 03 and the parity is odd.

<!-- pagebreak -->

