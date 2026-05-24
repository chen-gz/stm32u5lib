2902

Universal synchronous/asynchronous receiver transmitter (USART/UART)

66.8.14

RM0456

USART transmit data register (USART_TDR)
Address offset: 0x28
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

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

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

TDR[8:0]
rw

Bits 31:9 Reserved, must be kept at reset value.
Bits 8:0 TDR[8:0]: Transmit data value
Contains the data character to be transmitted.
The USART_TDR register provides the parallel interface between the internal bus and the
output shift register (see Section 66.5.1: USART block diagram).
When transmitting with the parity enabled (PCE bit set to 1 in the USART_CR1 register),
the value written in the MSB (bit 7 or bit 8 depending on the data length) has no effect
because it is replaced by the parity.
Note: This register must be written only when TXE/TXFNF = 1.

66.8.15

USART prescaler register (USART_PRESC)
This register can only be written when the USART is disabled (UE = 0).
Address offset: 0x2C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

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

Res.

3

2

1

0

15

14

13

12

11

10

9

8

7

6

5

4

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

PRESCALER[3:0]
rw

Bits 31:4 Reserved, must be kept at reset value.

<!-- pagebreak -->

