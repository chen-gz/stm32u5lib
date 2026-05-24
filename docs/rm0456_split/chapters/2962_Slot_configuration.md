RM0456 Rev 6

RM0456

FD controller area network (FDCAN)

Bit 13 TSW: Timestamp wraparound
0: No timestamp counter wrap-around
1: Timestamp counter wrapped around
Bit 12 TEFL: Tx event FIFO element lost
0: No Tx event FIFO element lost
1: Tx event FIFO element lost
Bit 11 TEFF: Tx event FIFO full
0: Tx event FIFO Not full
1: Tx event FIFO full
Bit 10 TEFN: Tx event FIFO new entry
0: Tx event FIFO unchanged
1: Tx handler wrote Tx event FIFO element.
Bit 9 TFE: Tx FIFO empty
0: Tx FIFO non-empty
1: Tx FIFO empty
Bit 8 TCF: Transmission cancellation finished
0: No transmission cancellation finished
1: Transmission cancellation finished
Bit 7 TC: Transmission completed
0: No transmission completed
1: Transmission completed
Bit 6 HPM: High-priority message
0: No high-priority message received
1: High-priority message received
Bit 5 RF1L: Rx FIFO 1 message lost
0: No Rx FIFO 1 message lost
1: Rx FIFO 1 message lost
Bit 4 RF1F: Rx FIFO 1 full
0: Rx FIFO 1 not full
1: Rx FIFO 1 full
Bit 3 RF1N: Rx FIFO 1 new message
0: No new message written to Rx FIFO 1
1: New message written to Rx FIFO 1
Bit 2 RF0L: Rx FIFO 0 message lost
0: No Rx FIFO 0 message lost
1: Rx FIFO 0 message lost
Bit 1 RF0F: Rx FIFO 0 full
0: Rx FIFO 0 not full
1: Rx FIFO 0 full
Bit 0 RF0N: Rx FIFO 0 new message
0: No new message written to Rx FIFO 0
1: New message written to Rx FIFO 0

RM0456 Rev 6

<!-- pagebreak -->

3086

FD controller area network (FDCAN)

70.4.16

RM0456

FDCAN interrupt enable register (FDCAN_IE)
The settings in the interrupt enable register determine which status changes in the interrupt
register are signaled on an interrupt line.
Address offset: 0x0054
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

ARAE

PEDE

PEAE

WDIE

BOE

EWE

EPE

ELOE

rw

rw

rw

rw

rw

rw

rw

rw

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

TFEE

TCFE

TCE

HPME

rw

rw

rw

rw

TOOE MRAFE TSWE
rw

rw

rw

TEFLE TEFFE TEFNE
rw

rw

rw

RF1LE RF1FE RF1NE RF0LE RF0FE RF0NE
rw

Bits 31:24 Reserved, must be kept at reset value.
Bit 23 ARAE: Access to reserved address enable
Bit 22 PEDE: Protocol error in data phase enable
Bit 21 PEAE: Protocol error in arbitration phase enable
Bit 20 WDIE: Watchdog interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 19 BOE: Bus-off status
0: Interrupt disabled
1: Interrupt enabled
Bit 18 EWE: Warning status interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 17 EPE: Error passive interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 16 ELOE: Error logging overflow interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 15 TOOE: Timeout occurred interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 14 MRAFE: Message RAM access failure interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 13 TSWE: Timestamp wraparound interrupt enable
0: Interrupt disabled
1: Interrupt enabled
Bit 12 TEFLE: Tx event FIFO element lost interrupt enable
0: Interrupt disabled
1: Interrupt enabled

<!-- pagebreak -->

