Signal name

Signal type

Description

sdmmc_ker_ck

Digital input

SDMMC kernel clock

sdmmc_hclk

Digital input

AHB clock

sdmmc_it

Digital output

SDMMC global interrupt

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
Table 275. SDMMC internal input/output signals (continued)
Signal name

Signal type

Description

sdmmc_io_in_ck

Digital input

SD/SDIO/e•MMC card feedback clock. This signal is
internally connected to the SDMMC_CK pin (for DS
and HS modes).

sdmmc_fb_ck

Digital input

SD/SDIO/e•MMC card tuned feedback clock after
DLYB delay block (for SDR50, DDR50, SDR104,
HS200)

Table 276. SDMMC pins
Pin name

Pin type

SDMMC_CK

Digital output Clock to SD/SDIO/e•MMC card

SDMMC_CKIN

Digital input

Clock feedback from an external driver for SD/SDIO/e•MMC
card. (for SDR12, SDR25, SDR50, DDR50)

SDMMC_CMD

Digital
input/output

SD/SDIO/e•MMC card bidirectional command/response signal.

SDMMC_CDIR

Digital output

SD/SDIO/e•MMC card I/O direction indication for the
SDMMC_CMD signal.

SDMMC_D[7:0]

Digital
input/output

SD/SDIO/e•MMC card bidirectional data lines.

SDMMC_D0DIR

Digital output

SD/SDIO/e•MMC card I/O direction indication for the
SDMMC_D0 data line.

SDMMC_D123DIR Digital output

31.5.3

Description

SD/SDIO/e•MMC card I/O direction indication for the data lines
SDMMC_D[3:1].

General description
The SDMMC_D[7:0] lines have different operating modes:
•

By default, SDMMC_D0 line is used for data transfer. After initialization, the host can
change the databus width.

•

For an e•MMC, 1-bit (SDMMC_D0), 4-bit (SDMMC_D[3:0]) or 8-bit (SDMMC_D[7:0])
data bus widths can be used.

•

For an SD or an SDIO card, 1-bit (SDMMC_D0) or 4-bit (SDMMC_D[3:0]) can be used.
All data lines operate in push-pull mode.

To allow the connection of an external driver (a voltage switch transceiver), the direction of
data flow on the data lines is indicated with I/O direction signals. The SDMMC_D0DIR signal
indicates the I/O direction for the SDMMC_D0 data line, the SDMMC_D123DIR for the
SDMMC_D[3:1] data lines.
SDMMC_CMD only operates in push-pull mode:
To allow the connection of an external driver (a voltage switch transceiver), the direction of
data flow on the SDMMC_CMD line is indicated with the I/O direction signal SDMMC_CDIR.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

SDMMC_CK clock to the card originates from sdmmc_ker_ck:
•

When the sdmmc_ker_ck clock has 50 % duty cycle, it can be used even in bypass
mode (CLKDIV = 0).

•

When the sdmmc_ker_ck duty cycle is not 50 %, the CLKDIV must be used to divide it
by 2 or more (CLKDIV > 0).

•

The phase relation between the SDMMC_CMD / SDMMC_D[7:0] outputs and the
SDMMC_CK can be selected through the NEGEDGE bit. The phase relation depends
on the CLKDIV, NEGEDGE, and DDR settings. See Figure 192.
Figure 192. SDMMC Command and data phase relation

CLKDIV = 0

CLKDIV > 0
DDR = 0
NEGEDGE = 0

CLKDIV > 0
DDR = 0
NEGEDGE = 1

CLKDIV > 0
DDR = 1
NEGEDGE = 0

CLKDIV > 0
DDR = 1
NEGEDGE = 1

SDMMC_CMD
SDMMC_Dn
SDMMC_CK
sdmmc_ker_ck
MSv40159V2

CLKDIV

DDR

NEGEDGE

Table 277. SDMMC Command and data phase selection

SDMMC_CK

0

x

x

=
sdmmc_ker_ck

Data out

Generated on sdmmc_ker_ck falling edge

0

Generated on sdmmc_ker_ck falling edge succeeding the SDMMC_CK rising
edge.

1

Generated on the same sdmmc_ker_ck rising edge that generates the
SDMMC_CK falling edge.

0

>0

Command out

0
1
1

Generated on
sdmmc_ker_ck
rising edge

Generated on sdmmc_ker_ck falling edge
succeeding the SDMMC_CK rising edge.
Generated on the same sdmmc_ker_ck rising
edge that generates the SDMMC_CK falling
edge.

Generated on
sdmmc_ker_ck falling edge
succeeding a SDMMC_CK
edge.

By default, the sdmmc_io_in_ck feedback clock input is selected for sampling incoming
data in the SDMMC receive path. It is derived from the SDMMC_CK pin.
For tuning the phase of the sampling clock to accommodate the receive data timing, the
DLYB delay block available on the device can be connected between sdmmc_io_in_ck
signal (DLYB input dlyb_in_ck) and sdmmc_fb_ck clock input of SDMMC (DLYB output
dlyb_out_ck). Selecting the sdmmc_fb_ck clock input in the receive path then enables
using the phase-tuned sampling clock for the incoming data. This is required for SDMMC to
support the SDR104 and HS200 operating mode and optional for SDR50 and DDR50
modes.

<!-- pagebreak -->

