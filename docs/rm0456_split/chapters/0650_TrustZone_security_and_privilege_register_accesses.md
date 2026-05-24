667

System configuration controller (SYSCFG)

RM0456

The table below shows the register security overview.
Table 131. TrustZone security and privilege register accesses
SYSCFG register name

Privileged
/unprivileged
access

Read/write access

TrustZone configuration(1)

TZEN = 1

TZEN = 0

Not applicable

SYSCFG_SECCFGR

Read: no restriction
Write: secure access only
Nonsecure write is WI and generates an illegal
access event.

RAZ/WI

Read: no restriction,
FPUSEC privileged
write only
Other bits write: no
restriction

SYSCFG_CSLCKR

Read/Write: secure access only
Nonsecure access is RAZ/WI and generates and
an illegal access event.

RAZ/WI

Privileged only
Unprivileged:
RAZ/WI

SYSCFG_FPUIMR

– If FPUSEC = 1:
Read/Write: secure access only
Nonsecure access is RAZ/WI and generates an
illegal access event.
– If FPUSEC = 0: Read/Write: no restriction

Privileged only
No
restriction Unprivileged:
RAZ/WI

SYSCFG_CNSLCKR

Read/write: no restriction

Privileged only
No
restriction Unprivileged:
RAZ/WI

SYSCFG_CFGR1

Read/Write: secure access only for secure bits
depending on peripheral security bits in GTZC
Nonsecure access only for nonsecure bits,
otherwise RAZ/WI

No
No restriction
restriction

SYSCFG_CFGR2

– If CLASSBSEC = 1:
Read/Write: secure access only
Nonsecure access is RAZ/WI and generates
an illegal access event.
– If CLASSBSEC = 0:Read/Write: no restriction

No
No restriction
restriction

SYSCFG_MESR

– If SYSCFGSEC = 1:
Read/Write: secure access only
Nonsecure access is RAZ/WI and generates
an illegal access event.
– If SYSCFGSEC = 0:Read/Write: no restriction

No
No restriction
restriction

SYSCFG_CCCSR
SYSCFG_CCVR
SYSCFG_CCCR

– If SYSCFGSEC = 1:
Read/Write: secure access only
Nonsecure access is RAZ/WI and generates
an illegal access event
– If SYSCFGSEC = 0:Read/Write: no restriction

No
No restriction
restriction

<!-- pagebreak -->

RM0456 Rev 6

RM0456

System configuration controller (SYSCFG)
Table 131. TrustZone security and privilege register accesses (continued)

SYSCFG register name
TrustZone configuration(1)
SYSCFG_RSSCMDR

Privileged
/unprivileged
access

Read/write access
TZEN = 1
RAZ/WI if register access is not allowed(2)

– If OTGSEC = 1:
SYSCFG_OTGHSPHYCR
Read/Write: secure access only
SYSCFG_OTGHSPHYTUNE Nonsecure access is RAZ/WI and generates an
R2
illegal access event.
– If OTGSEC = 0: Read/write, no restriction

TZEN = 0
RAZ/WI

Not applicable
No restriction

No restriction

1. TrustZone security is activated by the TZEN option bit in the FLASH_OPTR register.
2. Refer to register description for register access.

15.2.3

Configuring the OTG_HS PHY
(only for STM32U59x/5Ax/5Fx/5Gx)
In order to use the OTG_HS PHY, the following configuration steps are required before the
configuration of the OTG_HS:

15.2.4

1.

Activate clocks in RCC clock gating registers for SYSCFG, OTG_HS, and
OTG_HS PHY.

2.

Configure desired clock settings for OTG_HS PHY using CLKSEL bitfield
in SYSCFG_OTGHSPHYCR.

3.

Adjust the disconnect threshold by writing 0b010 to COMPDISTUNE bitfield and the
squelch threshold by writing 0b000 to SQRXTUNE bitfield in
SYSCFG_OTGHSPHYTUNER2.

4.

Enable the OTG_HS PHY by setting EN in SYSCFG_OTGHSPHYCR.

Adjusting HSPI supply capacitance
(only for STM32U59x/5Ax/5Fx/5Gx)
The HSPI supply capacitance can be adjusted using ENDCAP[1:0] in SYSCFG_CFGR1.
If the HSPI alternate functions are not used, ENDCAP[1:0] must be left at its reset value.

15.2.5

Internal SRAMs cacheability by DCACHE2
(only for STM32U59x/5Ax/5Fx/5Gx)
Since DCACHE2 is only addressed by the GPU2D M0 port, and because vector graphic
algorithms can manipulate data on the M1 port, it is recommended to clear SRAMCACHED
in SYSCFG_CFGR1 before activating the GPU2D, to avoid any cache coherency issues.
Also, since internal SRAMs are accessible in zero wait state through the bus matrix,
no performance degradation is expected.

RM0456 Rev 6

<!-- pagebreak -->

