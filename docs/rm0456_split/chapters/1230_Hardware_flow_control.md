1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

Figure 217. Card cycle power / power up diagram
Power stable

Card Vcc
Vcc min

SDMMC_CK

SDMMC_CMD

SDMMC_Dn

SDMMC state

Driven ‘0’
HiZ

CMD

Driven ‘0’
HiZ
Driven ‘0’

Reset

Power-cycle
1 ms min

0.1 ms min
Power ramp up

31.7

Driven ‘1’

Power-off

Power-on

1 ms min
74
SDMMC_CK
clocks

MSv39275V1

Hardware flow control
The hardware flow control during data transfer functionality is used to avoid FIFO underrun
(TX mode) and overrun (RX mode) errors.
The behavior is to stop SDMMC_CK during data transfer and freeze the SDMMC state
machines. The data transfer is stalled when the FIFO is unable to transmit or receive data.
The data transfer remains stalled until the transmit FIFO is half full or all data according
DATALENGHT has been stored, or until the receive FIFO is half empty. Only state machines
clocked by SDMMC_CK are frozen, the AHB interfaces are still alive. The FIFO can thus be
filled or emptied even if flow control is activated.
On an IDMA linked list transfer error, the hardware flow control is disabled. As a
consequence, depending on when the IDMA linked list transfer error occurs, an underrun or
overrun error may also occur (see Section : IDMA transfer error management).
To enable hardware flow control during data transfer, the HWFC_EN register bit must be set
to 1. After reset hardware flow control is disabled.
Hardware flow control must only be used when the SDMMC_Dn data is cycle-aligned with
the SDMMC_CK. Whenever the sdmmc_fb_ck from the DLYB delay block is used, i.e in the
case of SDR104 mode with a tOP and DtOP delay > 1 cycle, hardware flow control can not
be used.

31.8

Ultra-high-speed phase I (UHS-I) voltage switch
UHS-I mode (SDR12, SDR25, SDR50, SDR104, and DDR50) requires the support for 1.8 V
signaling. After power up the card starts in 3.3V mode. CMD11 invokes the voltage switch

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Secure digital input/output MultiMediaCard interface (SDMMC)
sequence to the 1.8V mode. When the voltage sequence is completed successfully the card
enters UHS-I mode with default SDR12 and card input and output timings are changed.
Figure 218. CMD11 signal voltage switch sequence
3.3 V
SDMMC_CK
0V

Provide SD clock at 3.3 V

Provide SD clock at 1.8 V

1.8 V

5 ms min.

3.3 V
SDMMC_CMD
0V

CMD11

1.8 V

R1
0 min.
1 ms max.

3.3 V
SDMMC_D[3:0]
0V

1.8 V

MSv40950V1

To perform the signal voltage switch sequence the following steps are needed:
1.

Before starting the Voltage Switch procedure, the SDMMC_CK frequency must be set
in the range 100 kHz - 400 kHz.

2.

The host starts the Voltage Switch procedure by setting the VSWITCHEN bit before
sending the CMD11.

3.

The card returns an R1 response.
–

if the response CRC is pass, the Voltage Switch procedure continues the host
does no longer drive the CMD and SDMMC_D[3:0] signals until completion of the
voltage switch sequence. Some cycles after the response the SDMMC_CK is
stopped and the CKSTOP flag is set.

–

if the response CRC is fail (CCRCFAIL flag) or no response is received before the
timeout (CTIMEOUT flag), the Voltage Switch procedure is stopped.

4.

The card drives CMD and SDMMC_D[3:0] to low at the next clock after the R1
response.

5.

The host, after having received the R1 response, may monitor the SDMMC_D0 line
using the BUSYD0 register bit. The SDMMC_D0 line is sampled two SDMMC_CK
clock cycles after the Response. The Firmware may read the BUSYD0 register bit
following the CKSTOP flag.

6.

–

When the BUSYD0 is detected low the host firmware switches the Voltage
regulator to 1.8V, after which it instructs the SDMMC to start the timing critical
section of the Voltage Switch sequence by setting register bit VSWITCH. The
hardware continues to stop the SDMMC_CK by holding it low for at least 5 ms.

–

When the BUSYD0 is detected high the host aborts the Voltage Switch sequence
and cycle power the card.

The card after detecting SDMMC_CK low begins switching signaling voltage to 1.8 V.

7.

The host SDMMC hardware after at least 5 ms restarts the SDMMC_CK.

8.

The card within 1 ms from detecting SDMMC_CK transition drives CMD and DAT[3:0]
high for at least 1 SDMMC_CK cycle and then stop driving CMD and DAT[3:0].

9.

The host SDMMC hardware, 1 ms after the SDMMC_CK has been restarted, the
SDMMC_D0 is sampled into BUSYD0 and the VSWEND flag is set.

RM0456 Rev 6

<!-- pagebreak -->

1259

Secure digital input/output MultiMediaCard interface (SDMMC)

RM0456

10. The host, on the VSWEND flag, checks SDMMC_D0 line using the BUSYD0 register
bit, to confirm completion of voltage switch sequence:
–

When BUSYD0 is detected high, Voltage Switch has been completed successfully.

–

When BUSYD0 is detected low, Voltage Switch has failed, the host cycles the card
power.

The minimum 5 ms time to stop the SDMMC_CK is derived from the internal un-gated
SDMMC_CK clock, which has a maximum frequency of 25 MHz (SD mode), as set by the
clock divider CLKDIV. The >5 ms time is counted by 212 cycles (10.24 ms @ 400 kHz). If a
lower SDMMC_CK frequency is selected by the clock divider CLKDIV the time for the
SDMMC_CK clock to be stopped is longer.
The maximum 1 ms time for the card to drive the SDMMC_Dn and SDMMC_CMD lines high
is derived from the internal ungated SDMMC_CK which has a maximum frequency of
25 MHz (SD mode), as set by the clock divider CLKDIV. The SDMMC checks the lines after
>1 ms time which is counted by 29 cycles (1.28 ms @ 25 MHz). If a lower SDMMC_CK
frequency is selected by the clock divider CLKDIV the time to check the lines is longer.
The signal voltage level is supported through an external voltage translation transceiver like
STMicroelectronics ST6G3244ME.

<!-- pagebreak -->

