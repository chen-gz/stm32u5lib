RM0456 Rev 6

RM0456

Transmission of commands in low-power mode
DSI Host can be configured to send the low-power commands during the high-speed video
mode transmission.
To enable this feature, set the Low Power command enable (LPCE) bit of the DSI Host
video mode configuration register (DSI_VMCR) to 1. In this case, it is necessary to calculate
the time available, in bytes, to transmit a command in low-power mode to horizontal frontporch (HFP), vertical sync active (VSA), vertical back-porch (VBP), and vertical front-porch
(VFP) regions.
Bits 8 to 13 of the video mode configuration register (DSI_VMCR) indicate if DSI Host can
go to LP when idle. If the low-power command enable (LPCE) bit is set and non-video
packets are in queue, DSI Host ignores the low-power configuration and transmits
low-power commands, even if it is not allowed to enter low-power mode in a specific region.
After the low-power commands transmission, DSI Host remains in low-power until a sync
event occurs.
For example, consider that the VFP is selected as high-speed region (LPVFPE = 1'b0) with
LPCE set as a command to transmit in low-power in the VPF region. This command is
transmitted in low-power, and the line stays in low-power mode until a new HSS arrives.

Calculating the time to transmit commands in LP mode in the VSA, VBP, and
VFP regions
The largest packet size (LPSIZE) field of the DSI Host low-power mode configuration
register (DSI_LPMCR) indicates the time available (in bytes) to transmit a command in lowpower mode (based on the escape clock) on a line during the VSA, VBP, and the VFP
regions.
Calculation of largest packet size (LPSIZE) depends on the used video mode.
Figure 429 illustrates the timing intervals for the video mode in non-burst with sync pulses,
while Figure 430 refers to video mode in burst and non-burst with sync events.
Figure 429. LPSIZE for non-burst with sync pulses
tL
tLPS -> HS

2 tESCCLK

tLPDT

EscExit

LPDT
command

HSÆLP

tLPDT

EscEntry

HSA

tHS -> LP

HSE

tH1

outvact_lpcmd_time

LPÆHS

MSv35870V1

Figure 430. LPSIZE for burst or non-burst with sync events
tL

outvact_lpcmd_time

tLPS -> HS

2 tESCCLK

tLPDT

EscExit

LPDT
command

HSÆLP

tLPDT

EscEntry

tH1 tHS -> LP

HSS

HSS

44.9.2

DSI Host (DSI)

LPÆHS

MSv35871V1

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

This time is calculated as follows:

LPSIZE = (tL - (tH1 + tHS->LP + tLPHS + tLPDT + 2 tESCCLK)) / (2 × 8 × tESCCLK), where
•
tL = line time
•
tH1 = time of the HSA pulse for sync pulses mode (Figure 429) or time to send the HSS
packet, including EoTp (Figure 430)

•

tHS->LP = time to enter the low-power mode
tLP->HS = time to leave the low-power mode
tLPDT = D-PHY timing related with escape mode entry, LPDT command, and escape

•

tESCCLK = escape clock period as programmed in the TXECKDIV field of the DSI_CCR

•

tESCCLK = delay imposed by the DSI Host implementation.

•
•

exit. According to the D-PHY specification, this value is always 11 bits in LP (or 22 TX
escape clock cycles)

register

In the above equation, division by eight is done to convert the available time to bytes.
Division by two is done because one bit is transmitted every two escape clock cycles. The
largest packet size (LPSIZE) field can be compared directly with the size of the command to
be transmitted to determine if there is enough time to transmit the command. The maximum
size of a command that can be transmitted in low-power mode is limited to 255 bytes by this
field. Program this register to a value greater than or equal to 4 bytes for the transmission of
the DCTRL commands, such as shutdown and color in low-power mode.
Consider an example of a frame with 12.4 μs per line and assume an escape clock
frequency of 20 MHz and a lane bit rate of 800 Mbit/s. In this case, it is possible to send 124
bits in escape mode (that is, 124 bit = 12.4 μs * 20 MHz / 2). Still, you need to consider the
D-PHY protocol and PHY timings.
The following assumptions are made:
•

lane byte clock period is 10 ns (800 Mbit/s per lane)

•

escape clock period is 50 ns (DSI_CCR.TXECKDIV = 5)

•

video is transmitted in non-burst mode with sync pulses bounded by HSS and HSE
packets

•

DSI is configured for two lanes

•

D-PHY takes 180 ns to transit from low-power to high-speed mode
(DSI_DLTCR.LS2HS_TIME = 18)

•

D-PHY takes 200 ns to transit from high-speed to low-power mode
(DSI_DLTCR.HS2LP_TIME = 20)

•

tHSA = 420 ns.

In this example, a 13-byte command can be transmitted as follows:
LPSIZE = (12.4 μs - (420 ns + 180 ns +200 ns + (22 × 50 ns + 2 × 50 ns))) / (2 × 8 × 50 ns)
= 13 bytes.

Calculating the time to transmit commands in low-power mode in HFP region
The VACT largest packet size (VLPSIZE) field of the DSI Host low-power mode
configuration register (DSI_LPMCR) indicates the time available (in bytes) to transmit a
command in low-power mode (based on the escape clock) in the vertical active (VACT)
region.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

DSI Host (DSI)
To calculate the value of VACT largest packet size (VLPSIZE), consider the video mode
being used. Figure 431 shows the timing intervals for video mode in non-burst with sync
pulses, Figure 432 those for video mode in non-burst with sync events, and Figure 433
refers to the burst video mode.
Figure 431. VLPSIZE for non-burst with sync pulses
tL
tLPDT

invact_lpcmd_time

tLPS -> HS

2 tESCCLK

HACT with
HSÆLP
Blanking Non-Burst

tLPDT

EscExit

tHS -> LP

LPDT
command

HBP

tHACT

EscEntry

HSA

tHBP

HSE

HSS

tHSA

LPÆHS

MSv35872V1

Figure 432. VLPSIZE for non-burst with sync events

HACT with
HSÆLP
Blanking Non-Burst

tLPDT

tLPDT

invact_lpcmd_time

tLPS -> HS

2 tESCCLK

HBP

tHS -> LP

EscExit

HSA

tHACT

LPDT
command

tHBP

EscEntry

HSS

tL
tHSA

LPÆHS

MSv35890V1

Figure 433. VLPSIZE for burst mode

HSA

HBP

HACT Burst

HSÆLP

tLPDT

tLPDT

invact_lpcmd_time

tLPS -> HS

2 tESCCLK

tHS -> LP

EscExit

tHACT

LPDT
command

tHBP

EscEntry

HSS

tL
tHSA

LPÆHS

MSv35873V1

This time is calculated as follows:
VLPSIZE = (tL - (tHSA + tHBP + tHACT + tHS->LP + tLP->HS + tLPDT + 2 tESCCLK)) /
(2 × 8 × tESCCLK)
where
•
tL = line time
•
tHSA = time of the HSA pulse (DSI_VHSACR.HSA)
•
tHBP = time of horizontal back-porch (DSI_VHBPCR.HBP)
•
tHACT = time of video active. For burst mode, the video active is time compressed and
is calculated as tHACT = VPSIZE * Bytes_per_Pixel /Number_Lanes * tLane_byte_clk
•
tESCCLK = escape clock period as programmed in TXECKDIV field of the DSI_CCR
register.

RM0456 Rev 6

<!-- pagebreak -->

1861

DSI Host (DSI)

RM0456

The VLPSIZE field can be compared directly with the size of the command to be transmitted
to determine if there is time to transmit the command.
Consider an example of a frame with 16.4 μs per line and assume an escape clock
frequency of 20 MHz and a lane bit rate of 800 Mbits/s. In this case, it is possible to send
420 bits in escape mode (that is, 164 bits = 16.4 μs * 20 MHz / 2). Still, since it is the vertical
active region of the frame, consider the HSA, HBP, and HACT timings apart from the D-PHY
protocol and PHY timings. The following assumptions are made:
•

number of active lanes is four

•

Lane byte clock period (lanebyteclkperiod) is 10 ns (800 Mbit/s per lane)

•

escape clock period is 50 ns (DSI_CCR.TXECKDIV = 5)

•

D-PHY takes 180 ns to pass from low-power to high-speed mode
(DSI_DLTCR.LP2HS_TIME = 18)

•

D-PHY takes 200 ns to pass from high-speed to low-power mode
(DSI_DLTCR.HS2LP_TIME = 20)

•

tHSA = 420 ns

•

tHBP = 800 ns

•

tHACT = 12800 ns to send 1280 pixel at 24 bpp

•

video is transmitted in non-burst mode

•

DSI is configured for four lanes.

In this example, consider that video is sent in non-burst mode. The VLPSIZE is calculated
as follows:
VLPSIZE = (16.4 µs -(420 ns + 800 ns + 12.8 µs + 180 ns +200 ns +
(22 × 50 ns + 2 × 50 ns)) / (2 × 8 × 50 ns) = 1 byte
Only one byte can be transmitted in this period. A short packet (for example, generic short
write) requires a minimum of four bytes. Therefore, in this example, commands are not sent
in the VACT region.
If burst mode is enabled, more time is available to transmit the commands in the VACT
region, because HACT is time compressed.
VLPSIZE = (16.4 µs - (420 ns + 800 ns + (1280 × 3 / 4 × 10 ns) + 180 ns + 200 ns +
(22 × 50 ns + 2 × 50 ns) / (2 × 8 × 50 ns) = 5 bytes
For burst mode, the VLPSIZE is 5 bytes and then a 4-byte short packet can be sent.

Transmission of commands in different periods
The LPSIZE and VLPSIZE fields allow a simple comparison to determine if a command can
be transmitted in any of the BLLP periods.
Figure 434 illustrates the meaning of VLPSIZE and LPSIZE, matching them with the shaded
areas and the VACT region.

<!-- pagebreak -->

