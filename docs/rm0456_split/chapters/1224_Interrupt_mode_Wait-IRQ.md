1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 212. CMD5 Sleep Awake procedure
VCC

SDMMC_CMD

SDMMC_D0

Off

CMD5
sleep

CMD5
awake

RESP

RESP

BUSY

BUSY

Transistion phase

Transistion phase

BUSYD0END

Sleep state

Standby state

Standby state
MSv40943V1

31.6.4

Interrupt mode (Wait-IRQ)
The host and card enter and exit interrupt mode (Wait-IRQ) simultaneously. In interrupt
mode there is no data transfer. The only message allowed is an interrupt service request
response from the card or the host. For the interrupt mode to work correctly the
SDMMC_CK frequency must be set in accordance with the achievable SDMMC_CMD data
rate in Open Drain mode, which depend on the capacitive load and pull-up resistor. The
CLKDIV must be set >1, and the SETCLKRX must select either the sdmmc_io_in_ck or
SDMMC_CLKin source.
The host must ensure that the card is in Standby state before issuing the CMD40
(GO_IRQ_STATE). While waiting for an interrupt response the SDMMC_CK clock signal
must be kept active.
A card in interrupt mode (IRQ state):
•

is waiting for an internal card interrupt event. Once the event occurs, the card starts to
send the interrupt service request response. The response is sent in open-drain mode.

•

while waiting for the internal card interrupt event, the card also monitors the
SDMMC_CMD line for a start bit. Upon detection of a start bit the card aborts the
interrupt mode and switch to Standby state.

The host in interrupt mode (CPSM Wait state waiting for interrupt):
•

is waiting for a card interrupt service request response (start bit).

•

while waiting for a card interrupt service request response the host may abort the
interrupt mode (by clearing the WAITINT register bit), which causes the host to send a
interrupt service request response R5 with RCA = 0x0000 in open-drain mode.

When sending the interrupt service request response, the sender bit-wise monitors the
SDMMC_CMD bit stream. The sender whose interrupt service request response bit does
not correspond to the bit on the SDMMC_CMD line stops sending. In the case of multiple
senders only one successfully sends its full interrupt service request response. If the host
sends simultaneously, it loses sending after the transmission bit.
To handle the interrupt mode, the following procedure applies:

<!-- pagebreak -->

