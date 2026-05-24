1674

Audio digital filter (ADF)

RM0456

Table 402. Register protection summary (continued)
Unprotected
fields

Write-protected fields

Write-protection
condition

ADF DFLT0 interrupt enable register
(ADF_DFLT0IER)

All fields

-

-

ADF DFLT0 interrupt status register 0
(ADF_DFLT0ISR)

All fields

-

-

ADF SAD control register (ADF_SADCR)

SADEN

FRSIZE[2:0], DETCFG,
SADMOD[1:0], HYSTEN,
DATCAP[1:0]

SADACTIVE = 1

-

ANMIN[12:0], SNTHR[3:0],
HGOVR[3:0], LFRNB[2:0],
ANSLP[2:0]

SADACTIVE = 1

Registers

ADF SAD configuration register
(ADF_SADCFGR)

All the ADF processing is performed in the adf_proc_ck clock domain. For that reason,
enabling or disabling an ADF sub-block may take some time due to the re-synchronization
between the AHB clock domain and the adf_proc_ck clock domain. XXXACTIVE flags are
available to allow the application to check that the synchronization between the two clock
domains is completed.
To change a write-protected bitfield, the application must follow this sequence:
1.

Set the enable bit of the sub-block to 0.

2.

Wait for corresponding flag XXXACTIVE = 0.

3.

Modify the wanted fields.

4.

Set the enable bit of the sub-block to 1.

Refer to the description of each sub-block for details.

40.5

ADF low-power modes
Table 403. Effect of low-power modes on ADF

Mode

Description

Sleep

No effect. ADF interrupts cause the device to exit the Sleep mode.

Stop(1)

The ADF registers content is kept.
Thanks to the kernel clock request feature, if the ADF is clocked by an internal oscillator available in Stop
mode, the ADF remains active. The interrupts cause the device to exit Stop mode.

Standby The ADF is powered down and must be reinitialized after exiting Standby mode.
1. Refer to Section 40.3: ADF implementation for details about Stop modes supported by the ADF.

40.6

ADF interrupts
To increase the CPU performance, the ADF offers an interrupt line (adf_flt0_it), sensitive to
several events.

Note:

<!-- pagebreak -->

