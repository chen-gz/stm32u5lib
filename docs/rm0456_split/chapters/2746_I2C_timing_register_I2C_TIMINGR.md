Signal name

Signal type

Description

lpuart_pclk

Input

APB clock

lpuart_ker_ck

Input

LPUART kernel clock

lpuart_wkup

Output

LPUART provides a wake-up interrupt

RM0456 Rev 6

RM0456

Low-power universal asynchronous receiver transmitter (LPUART)
Table 688. LPUART internal input/output signals (continued)
Signal name

Signal type

Description

lpuart_it

Output

lpuart_tx_dma

Input/output

LPUART transmit DMA request

lpuart_rx_dma

Input/output

LPUART receive DMA request

lpuart_trg[15:0]

Input

LPUART global interrupt

LPUART triggers.

Description LPUART interconnections
Table 689. LPUART interconnections (LPUART1)
Signal name

Source

lpuart_trg0

lpdma1_ch0_tc

lpuart_trg1

lpdma1_ch1_tc

lpuart_trg2

lpdma1_ch2_tc

lpuart_trg3

lpdma1_ch3_tc

lpuart_trg4

exti6

lpuart_trg5

exti8

lpuart_trg6

lptim1_ch1

lpuart_trg7

lptim3_ch1

lpuart_trg8

comp1_out

lpuart_trg9

comp2_out

lpuart_trg10

rtc_alra_trg

lpuart_trg11

rtc_wut_trg

lpuart_trg12

-

lpuart_trg13

-

lpuart_trg14

-

lpuart_trg15

-

RM0456 Rev 6

<!-- pagebreak -->

