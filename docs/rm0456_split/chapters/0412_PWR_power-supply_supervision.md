482

Power control (PWR)

RM0456

10.6

PWR power-supply supervision

10.6.1

Brownout reset (BOR)
The device has an integrated brownout reset (BOR) circuitry. The BOR is active in all power
modes except Shutdown mode, and cannot be disabled.
Five BOR thresholds can be selected through option bytes. BOR0 provides the always
enabled power-on/power-down functionality, independent from any other higher BOR level
selection.
During power-on, the BOR keeps the device under reset until the supply voltage VDD
reaches the specified VBORx threshold. When VDD drops below the selected threshold, a
device reset is generated. When VDD is above the VBORx upper limit, the device reset is
released and the system can start.
For more details on the brownout reset thresholds, refer to the electrical characteristics
section in the datasheet.
During Standby mode, and if BOR level 0 is selected, it is possible to set the BOR in
ultra-low-power mode to further reduce the current consumption by setting ULPMEN in
PWR_CR1.
Figure 33. Brownout reset waveform
VDD
VBOR0 (rising edge)
hysteresis
VBOR0 (falling edge)
Temporization
tRSTTEMPO

Reset
MS31444V5

1. The reset temporization tRSTTEMPO is present only for the BOR lowest threshold (VBOR0).

Note:

BOR is not functional in Shutdown mode.

10.6.2

Programmable voltage detector (PVD)
The PVD can be used to monitor the VDD power supply by comparing it to a threshold
selected by PVDLS[2:0] in PWR_SVMCR.
The PVD is enabled by setting PVDE bit.
A PVDO flag is available in PWR_SVMCR to indicate if VDD is higher or lower than
the PVD threshold. This event is internally connected to the EXTI, and can generate an
interrupt if enabled through the EXTI registers (refer to Table 110).
The rising/falling edge sensitivity of the EXTI Line must be configured according to PVD
output behavior. For example, if the EXTI line is configured to rising edge sensitivity, the

<!-- pagebreak -->

