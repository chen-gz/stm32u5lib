2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

RM0456

Bit 17 TRIGEN: Trigger enable bit
0: Trigger disabled
1: Trigger enabled
Note: This bitfield can be written only when the UE bit of USART_CR1 register is cleared.
When a trigger is detected, TE is set to 1 in USART_CR1 and the data transfer is
launched.
Bit 16 TRIGPOL: Trigger polarity bit
This bitfield can be written only when the UE bit is cleared in USART_CR1 register.
0: Trigger active on rising edge
1: Trigger active on falling edge
Bits 15:0 TDN[15:0]: TDN transmission data number
This bitfield enables the programming of the number of data to be transmitted. It can be
written only when UE is cleared in USART_CR1.

66.8.17

USART register map

2

1

0

UESM

UE

UESM

UE
SLVEN

3

RE
RE

Res.

4

TE
TE

Res.

5

IDLEIE
IDLEIE

0

0

0

0

0

0

IRLP

IREN

EIE

0

0

0

0

0

0

0

0

0

CTSE

RTSE

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

0

BRR[15:0]
0

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EIE

HDSEL

0

IREN

NACK

0

IRLP

SCEN

0

HDSEL

DMAR

0

NACK

DMAT

0

SCEN

RTSE

0

DMAR

CTSE

0

0

0

0

0

0

0

GT[7:0]
0

RM0456 Rev 6

0

CTSIE

0

0

CTSIE

0

0

ONEBIT

0

0

ONEBIT

0

0

OVRDIS

0

OVRDIS

0

DDRE

0

DDRE

0

DIS_NSS

6

RXFNEIE
RXNEIE

0

ADDM7

7

TCIE
TCIE

0

LBDL

8

TXFNFIE
TXEIE

0

LBDIE

9
PS

PEIE

PS

PEIE

Res.

11

10
PCE
PCE

0

LBCL

12

WAKE
WAKE

0

CPHA

13

M0
M0

0

CPOL

14

MME
MME

0

CLKEN

15

CMIE

0

LINEN

CMIE

16

OVER8
OVER8

17

0

DMAT

Res.

Res.

Res.

0

DEM

Res.
Res.

0

DEM

TCBGTIE
Res.

0

SWAP

Res.
Res.

0

DEP

Res.
Res.

0

0

Reset value

<!-- pagebreak -->

