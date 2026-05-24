2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Bit 28 M1: Word length
This bit must be used in conjunction with bit 12 (M0) to determine the word length. It is set or
cleared by software.
M[1:0] = 00: 1 Start bit, 8 Data bits, n stop bits
M[1:0] = 01: 1 Start bit, 9 Data bits, n stop bits
M[1:0] = ‘10: 1 Start bit, 7 Data bits, n stop bits
This bit can only be written when the LPUART is disabled (UE = 0).
Note: In 7-bit data length mode, the smartcard mode, LIN master mode and auto baud rate
(0x7F and 0x55 frames detection) are not supported.
Bits 27:26 Reserved, must be kept at reset value.
Bits 25:21 DEAT[4:0]: Driver Enable assertion time
This 5-bit value defines the time between the activation of the DE (Driver Enable) signal and
the beginning of the start bit. It is expressed in lpuart_ker_ck clock cycles. For more details,
refer Section 66.5.21: RS232 hardware flow control and RS485 driver enable.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bits 20:16 DEDT[4:0]: Driver Enable deassertion time
This 5-bit value defines the time between the end of the last stop bit, in a transmitted
message, and the de-activation of the DE (Driver Enable) signal.It is expressed in
lpuart_ker_ck clock cycles. For more details, refer Section 67.4.14: RS232 hardware flow
control and RS485 driver enable.
If the LPUART_TDR register is written during the DEDT time, the new data is transmitted
only when the DEDT and DEAT times have both elapsed.
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 15 Reserved, must be kept at reset value.
Bit 14 CMIE: Character match interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated when the CMF bit is set in the LPUART_ISR register.
Bit 13 MME: Mute mode enable
This bit activates the mute mode function of the LPUART. When set, the LPUART can switch
between the active and mute modes, as defined by the WAKE bit. It is set and cleared by
software.
0: Receiver in active mode permanently
1: Receiver can switch between mute mode and active mode.
Bit 12 M0: Word length
This bit is used in conjunction with bit 28 (M1) to determine the word length. It is set or
cleared by software (refer to bit 28 (M1) description).
This bit can only be written when the LPUART is disabled (UE = 0).
Bit 11 WAKE: Receiver wake-up method
This bit determines the LPUART wake-up method from mute mode. It is set or cleared by
software.
0: Idle line
1: Address mark
This bitfield can only be written when the LPUART is disabled (UE = 0).

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

Bit 10 PCE: Parity control enable
This bit selects the hardware parity control (generation and detection). When the parity
control is enabled, the computed parity is inserted at the MSB position (9th bit if M = 1; 8th bit
if M = 0) and parity is checked on the received data. This bit is set and cleared by software.
Once it is set, PCE is active after the current byte (in reception and in transmission).
0: Parity control disabled
1: Parity control enabled
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 9 PS: Parity selection
This bit selects the odd or even parity when the parity generation/detection is enabled (PCE
bit set). It is set and cleared by software. The parity is selected after the current byte.
0: Even parity
1: Odd parity
This bitfield can only be written when the LPUART is disabled (UE = 0).
Bit 8 PEIE: PE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever PE = 1 in the LPUART_ISR register
Bit 7 TXEIE: Transmit data register empty
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated whenever TXE = 1 in the LPUART_ISR register
Bit 6 TCIE: Transmission complete interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever TC = 1 in the LPUART_ISR register
Bit 5 RXNEIE: Receive data register not empty
This bit is set and cleared by software.
0: Interrupt is inhibited
1: A LPUART interrupt is generated whenever ORE = 1 or RXNE = 1 in the LPUART_ISR
register
Bit 4 IDLEIE: IDLE interrupt enable
This bit is set and cleared by software.
0: Interrupt is inhibited
1: An LPUART interrupt is generated whenever IDLE = 1 in the LPUART_ISR register
Bit 3 TE: Transmitter enable
This bit enables the transmitter. When the autonomous mode is disabled, TE bit is set and
cleared by software. When the autonomous mode is enabled, TE bit becomes a status bit,
which is set and cleared by hardware.
0: Transmitter is disabled
1: Transmitter is enabled
Note: When the LPUART acts as a transmitted, a low pulse on the TE bit (0 followed by 1)
sends a preamble (idle line) after the current word, except in smartcard mode. In order
to generate an idle character, the TE must not be immediately written to 1. To ensure
the required duration, the software can poll the TEACK bit in the LPUART_ISR register.
In smartcard mode, when TE is set, there is a 1 bit-time delay before the transmission
starts.

RM0456 Rev 6

<!-- pagebreak -->

