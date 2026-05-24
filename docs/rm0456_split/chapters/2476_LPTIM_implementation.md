When writing to the WWDG_CR register, always write 1 in the
T6 bit to avoid generating an immediate reset.

RM0456 Rev 6

RM0456

System window watchdog (WWDG)
Figure 772. Window watchdog timing diagram

CNT down-counter
Refresh not allowed

Refresh allowed

T[6:0]

W[6:0]

0x3F

Time
Tpclk x 4096 x 2WDGTB
0x41
0x40
0x3F

wwdg_ewit
EWIF = 0
wwdg_rst

T6 bit
MS47266V1

The formula to calculate the timeout value is given by:
WDGTB[2:0]
t WWDG = t PCLK × 4096 × 2
× ( T [ 5:0 ] + 1 )

( ms )

where:
•

tWWDG: WWDG timeout

•

tPCLK: APB clock period measured in ms

•

4096: value corresponding to internal divider

As an example, if APB frequency is 48 MHz, WDGTB[2:0] is set to 3, and T[5:0] is set to 63:
3
t WWDG = ( 1 ⁄ 48000 ) × 4096 × 2 × ( 63 + 1 ) = 43.69ms
Refer to the datasheet for the minimum and maximum values of tWWDG.

62.4.6

Debug mode
When the device enters debug mode (processor halted), the WWDG counter either
continues to work normally or stops, depending on the configuration bit in DBG module. For
more details, refer to Section 75: Debug support (DBG).

RM0456 Rev 6

<!-- pagebreak -->

