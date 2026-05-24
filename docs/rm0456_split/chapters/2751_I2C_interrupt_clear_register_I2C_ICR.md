2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

7.

Write the data to send in the LPUART_TDR register. Repeat this operation for each
data to be transmitted in case of single buffer.
– When FIFO mode is disabled, writing a data in the LPUART_TDR clears the TXE
flag.
– When FIFO mode is enabled, writing a data in the LPUART_TDR adds one data to
the TXFIFO. Write operations to the LPUART_TDR are performed when the TXFNF
flag is set. This flag remains set until the TXFIFO is full.

8.

When the last data is written to the LPUART_TDR register, wait until TC = 1. This
indicates that the transmission of the last frame has been completed.
– When FIFO mode is disabled, this indicates that the transmission of the last frame
has been completed.
– When FIFO mode is enabled, this indicates that both TXFIFO and shift register are
empty.
This check is required to avoid corrupting the last transmission when the LPUART is
disabled or enters Halt mode.

Single byte communication
• When FIFO mode disabled:
Writing to the transmit data register always clears the TXE bit. The TXE flag is set by
hardware to indicate that:
–

the data have been moved from the LPUART_TDR register to the shift register
and data transmission has started;

–

the LPUART_TDR register is empty;

–

the next data can be written to the LPUART_TDR register without overwriting the
previous data.

The TXE flag generates an interrupt if the TXEIE bit is set.
When a transmission is ongoing, a write instruction to the LPUART_TDR register
stores the data in the TDR register, which is copied to the shift register at the end of the
current transmission.
When no transmission is ongoing, a write instruction to the LPUART_TDR register
places the data in the shift register, the data transmission starts, and the TXE bit is set.
• When FIFO mode is enabled, the TXFNF (TXFIFO not full) flag is set by hardware to
indicate that:
– The TXFIFO is not full;
– The LPUART_TDR register is empty;
– The next data can be written to the LPUART_TDR register without overwriting the
previous data. When a transmission is ongoing, a write operation to the

<!-- pagebreak -->

