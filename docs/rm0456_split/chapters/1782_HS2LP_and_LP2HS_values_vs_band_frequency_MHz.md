1861

DSI Host (DSI)

RM0456
Table 445. HS2LP and LP2HS values vs. band frequency (MHz)

80
to 100

100
to 120

120
to 160

160
to 200

200
to 240

240
to 320

320
to 390

390
to 450

450
to 510

Clock
4 + LPXO 4 + LPXO 5 + LPXO 6 + LPXO 7 + LPXO 7 + LPXO 9 + LPXO 9 + LPXO 11 + LPXO
HS2LP
Clock
8+
LP2HS 3 * LPXO
Data
HS2LP

5

Data
8+
LP2HS 2 * LPXO

44.12.3

8+
3 * LPXO

14 +
3 * LPXO

16 +
3 * LPXO

21 +
3 * LPXO

25 +
3 * LPXO

32 +
3 * LPXO

33 +
3 * LPXO

40 +
3 * LPXO

5

6

7

8

8

10

10

12

9+
2 * LPXO

11 +
2 * LPXO

14 +
2 * LPXO

15 +
2 * LPXO

17 +
2 * LPXO

19 +
2 * LPXO

20 +
2 * LPXO

23 +
2 * LPXO

Special D-PHY operations
The DSI Wrapper features some control bits to force the D-PHY in some particular state
and/or behavior.

Forcing lane state
It is possible to force the data lane and/or the clock lane in TX stop mode through the bits
FTXSMDL and FTXSMCL of the DSI_WPCR0 register. Setting these bits causes the
respective lane module to immediately jump in transmit control mode and to begin
transmitting a Stop state (LP-11).
This feature can be used to go back in TX mode after a wrong BTA sequence.

44.12.4

DSI PLL control
The dedicated DSI PLL is controlled through the DSI Wrapper, as shown in Figure 444.
Figure 444. PLL block diagram
Ring oscillator
FVCO
500 to 1000 MHz

FPFD
2 to 100 MHz
FCLKIN
4 to 100 MHz

9-bit
input divider
1/IDF

PFD

9-bit
output divider
1/ODF

Filter

9-bit
feedback divider
1/DIVN

PHI1

1/2
MS53570V1

The PLL output frequency is configured through the DSI_WRPCR register fields. The VCO
frequency and the PLL output frequency are calculated as follows:
FVCO = (CLKIN / IDF) * 2 * NDIV,
PHI = FVCO / ODF

<!-- pagebreak -->

