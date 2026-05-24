RM0456 Rev 6

RM0456

Real-time clock (RTC)
Table 634. RTC interconnection
Signal name

Source/destination

rtc_its

From power controller (PWR): main power loss/switch to VBAT detection
output

rtc_tamp_evt

From TAMP peripheral: tamp_evt

rtc_tzen

From FLASH option bytes: TZEN

rtc_calovf

To TAMP peripheral: tamp_itamp5

The TZEN option bit is used to activate TrustZone in the device.
TZEN = 1: TrustZone activated.
TZEN = 0: TrustZone disabled.
When TrustZone is disabled, the APB access to the RTC registers are nonsecure.
The triggers outputs can be used as triggers for other peripherals.

63.3.3

GPIOs controlled by the RTC and TAMP
The GPIOs included in the Battery Backup Domain (VBAT) are directly controlled by the
peripherals providing functions on these I/Os, whatever the GPIO configuration.
RTC_OUT1, RTC_TS, TAMP_IN1 and TAMP_OUT2 are mapped on the same pin (PC13).
The RTC and TAMP functions mapped on PC13 are available in all low-power modes and in
VBAT mode.
The output mechanism follows the priority order shown in Table 635.

RM0456 Rev 6

<!-- pagebreak -->

2687

Real-time clock (RTC)

RM0456

OUT2EN

TAMPALRM_TYPE

TAMPALRM_PU

TAMP2E=TAMP2AM=1 with
ATOSHARE=0, or TAMPxE=TAMPxAM=1
with ATOSHARE=1 and ATOSELx=1

TAMP1E
(TAMP_IN1 input enable)

TSE
(RTC_TS input enable)

Don’t
care

0

0

Don’t
care

Don’t
care

Don’t
care

Don’t
care

Don’t
care

1

0

Don’t
care

Don’t
care

Don’t
care

Don’t
care

Don’t
care

1

1

Don’t
care

Don’t
care

Don’t
care

0

1

0

Don’t
care

Don’t
care

Don’t
care

Don’t
care

Don’t
care

00

0

0

0

Don’t
care

Don’t
care

1

Don’t
care

Don’t
care

00

0

0

Don’t
care

00

0

1

Don’t
care

Don’t
care

0

1

0

TAMPOE
(TAMPER output enable)

Don’t
care

OSEL[1:0]
(ALARM output enable)

COE
(CALIB output enable)

Table 635. RTC pin PC13 configuration(1)

01 or
10 or
11

0

00

1

01 or
10 or
11

1

01 or
10 or
11

0

00

1

01 or
10 or
11

1

01 or
10 or
11

0

00

1

01 or
10 or
11

1

CALIB output PP

00

TAMP_OUT2 output PP

PC13 pin function

TAMPALRM output
Push-Pull

No pull

TAMPALRM
output
Open-Drain(2)
Internal
pull-up

TAMP_IN1 input floating

Don’t
care

<!-- pagebreak -->

