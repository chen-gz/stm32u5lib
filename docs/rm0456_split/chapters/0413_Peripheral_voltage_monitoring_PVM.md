RM0456 Rev 6

RM0456

Power control (PWR)
interrupt is generated when VDD drops below the PVD threshold. As an example, the
service routine can perform emergency shutdown tasks.
The PVD can remain active in Stop 0, Stop 1, and Stop 2 modes, and the PVM interrupt can
wake up from Stop mode. The PVD is not functional in Stop 3 mode.
Figure 34. PVD thresholds
V DD

V PVD threshold

100 mV
hysteresis

PVD output
MSv66959V1

Note:

PVD is not functional in Shutdown mode.

10.6.3

Peripheral voltage monitoring (PVM)
Only VDD is monitored by default, as it is the only supply required for all system-related
functions. The other supplies (VDDA, VDDIO2, and VDDUSB) can be independent from VDD
and can be monitored with four peripheral voltage monitoring (PVM):
•

The UVM monitors the USB supply VDDUSB. VDDUSBRDY indicates if the VDDUSB
independent power supply is higher or lower than the VUVM threshold.

•

The IO2VM monitors the PG[15:2] supply VDDIO2. VDDIO2RDY indicates if the VDDIO2
independent power supply is higher or lower than the VIO2VM threshold.

•

The AVM1 monitors the analog supply VDDA. VDDA1RDY indicates if the VDDA
independent power supply is higher or lower than the VAVM1 threshold.

•

The AVM2 monitors the analog supply VDDA. VDDA2RDY indicates if the VDDA
independent power supply is higher or lower than the VAVM2 threshold.

Each PVM output is connected to an EXTI line and can generate an interrupt if enabled
through the EXTI registers. The PVMx output interrupt is generated when the independent
power supply drops below the PVM threshold and/or when it rises above the PVM threshold,
depending on EXTI line rising/falling edge configuration (refer to Table 110).
Each PVM can remain active in Stop 0, Stop 1, and Stop 2 modes, and the PVM interrupt
can wake up from the Stop mode. The PVM is not functional in Stop 3 mode.
Table 98. PVM features
PVM

Power supply

PVM threshold

UVM

VDDUSB

VUVM (around 1.2 V)

IO2VM

VDDIO2

VIO2VM (around 0.9 V)

AVM1

VDDA

VAVM1 (around 1.6 V)

AVM2

VDDA

VAVM2 (around 1.8 V)

RM0456 Rev 6

<!-- pagebreak -->

482

Power control (PWR)

RM0456

The independent supplies (VDDA, VDDIO2, and VDDUSB) are not considered as present
by default, and a logical and electrical isolation is applied to ignore any information coming
from the peripherals supplied by these dedicated supplies:
•

If these supplies are shorted externally to VDD, the application assumes they are
available without enabling any peripheral voltage monitoring.

•

If these supplies are independent from VDD, the peripheral voltage monitoring (PVM)
can be enabled to confirm whether the supply is present or not.

The following sequence must be done before using the USB/OTG_FS/OTG_HS:
1.

If VDDUSB is independent from VDD:
a)

2.
3.

Enable the UVM by setting UVMEN in PWR_SVMCR.

b)

Wait for the UVM wake-up time.

c)

Wait until VDDUSBRDY is set in PWR_SVMSR.

d)

Disable the UVM for consumption saving (optional).

Set USV in PWR_SVMCR to remove the VDDUSB power isolation.
On STM32U59x/5Ax/5Fx/5Gx devices only:
a)

Make sure the voltage scaling is in range 1 or in range 2 (using VOS[1:0]
in PWR_VOSR).

b)

Make sure the EPOD booster clock is enabled (using PLL1MBOOST[3:0]
in RCC_PLL1CFGR).

c)

Enable the USB internal power by setting USBPWREN and USBBOOSTEN
in PWR_VOSR.

d)

Wait for USBBOOSTRDY to be set in PWR_VOSR.

The following sequence must be done before using any I/O from PG[15:2]:
1.

2.

If VDDIO2 is independent from VDD:
a)

Enable the IO2VM by setting IO2VM in PWR_SVMCR.

b)

Wait for the IO2CVM wake-up time.

c)

Wait until VDDIO2RDY is set in PWR_SVMSR.

d)

Disable the IO2VM for consumption saving (optional).

Set IO2SV in PWR_SVMCR to remove the VDDIO2 power isolation.

The following sequence must be done before using any of these analog peripherals: analog
to digital converters, digital to analog converters, comparators, operational amplifiers,
voltage reference buffer:
1.

2.
Note:

<!-- pagebreak -->

