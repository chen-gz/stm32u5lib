2902

Low-power universal asynchronous receiver transmitter (LPUART)

RM0456

Table 686. USART/LPUART features (continued)
Modes/features(1)

Full feature

Basic feature

Low-power feature

Receiver timeout interrupt

X

X

-

Modbus communication

X

X

-

Auto baud rate detection

X

X

-

Driver enable

X

X

X

USART data length

7, 8 and 9 bits

Tx/Rx FIFO

X

X

Wake-up from low-power mode

X

(2)

(2)

X

X(3)

Autonomous mode

X

X

X

Tx/Rx FIFO size

8

1. X = supported.
2. Wake-up supported from Stop 0 and Stop 1 modes.
3. Wake-up supported from Stop 0, Stop 1, and Stop 2 modes.

<!-- pagebreak -->

X

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)

67.4

LPUART functional description

67.4.1

LPUART block diagram
Figure 834. LPUART block diagram

LPUART
lpuart_ker_ck clock domain
lpuart_wkup
lpuart_it
lpuart_tx_dma
lpuart_rx_dma

IRQ Interface

lpuart_pclk
clock domain

lpuart_trg[15:0]

DMA Interface

Hardware
flow control

TxFIFO

LPUART_CR3
LPUART_RQR
LPUART_ICR
LPUART_TDR

RxFIFO

32-bit APB bus

COM Controller
LPUART_CR1
LPUART_ISR
LPUART_CR2

LPUART_RDR
LPUART_
RTOR

CTS
RTS/DE

TX Shift Reg
...

TX

RX Shift Reg
...

RX

Baudrate
generator
lpuart_pclk
lpuart_ker_ck

LPUART_BRR
LPUART_
PRESC

lpuart_ker_ck_pres

MSv40858V6

RM0456 Rev 6

<!-- pagebreak -->

