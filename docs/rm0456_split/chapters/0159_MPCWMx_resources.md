RM0456 Rev 6

RM0456

System security

Note:

Like with TrustZone, a peripheral can be made privileged-only with TZSC
(see Section 3.6.3). In this case, if this peripheral is master on the interconnect,
it automatically issues privileged transactions.

Securing memories with TZSC and MPCBB
The TZSC block in GTZC provides the capability to manage the security and privilege for all
securable external memories, programming the MPCWM resources.
The table below shows an implementation example. For all other implementations on the
STM32U5 Series devices, refer to Section 5.3: GTZC implementation.
Table 9. MPCWMx resources
MPC
resource

Memory
OCTOSPI1

MPCWM1

FMC_NOR bank

MPCWM2

FMC_NAND bank

MPCWM3

Backup SRAM (BKPSRAM)

MPCWM4

OCTOSPI2

MPCWM5

HSPI1

MPCWM6

Number of
regions

Type of filtering

Default
security

On-the-fly
decryption(1)
Yes

2
Nonsecure
privileged or
unprivileged region
(watermarks)

2
1

Secure
privileged(2)

1

No

2

Yes

2

No

1.

Using the OTFDEC.

2.

Assuming TrustZone is activated on the device, nonsecure unprivileged otherwise.

The MPCBB resources in GTZC provide the capability to configure the security and
privilege of embedded SRAM blocks.
The table below shows an implementation example. For all other implementations on the
STM32U5 Series devices, refer to Section 5.3: GTZC implementation.
Table 10. MPCBBx resources
Memory

MPC resource

SRAM1

GTZC1_MPCBB1

SRAM2

GTZC1_MPCBB2

Type of filtering

Block based,
managing security
and privilege

Memory size
(Kbytes)

Block size
(Bytes)

Number
of superblocks

768

48

64

4

SRAM3

GTZC1_MPCBB3

832

SRAM4

GTZC2_MPCBB4

SRAM5

GTZC1_MPCBB5

832

52

SRAM6

GTZC1_MPCBB6

512

32

16

512(1)

52
1

Default
security

Secure
privileged(2)

1. Blocks are grouped in super-blocks of 32 consecutive blocks, to manage the configuration locking.
2. Assuming TrustZone is activated on the device, nonsecure unprivileged otherwise.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456

Applying GTZC configurations
The TZSC and MPCBB blocks can be used in one of the following ways:
•

statically programmed during the secure boot, locked and not changed afterwards

•

dynamically reprogrammed using a specific application code or real-time kernel

When the dynamic option is selected and the configuration is not locked:
•

MPCBB secure blocks or MPCWM nonsecure region size can be changed by a secure
software. This software must be privileged for MPCWM, can be unprivileged if the
particular block is not privileged-only.

•

The secure (respectively privilege) state of each peripheral can be changed writing to
GTZC_TZSC_SECCFRGx (respectively GTZC_TZSC_PRIVCFGRx) registers.

Securing peripherals with TZSC
The TZSC block in GTZC provides the capability to manage the security and the privilege
for all securable peripherals. The list of these peripherals can be found in Section 5: Global
TrustZone controller (GTZC).
Note:

When the TrustZone is deactivated, the resource isolation hardware GTZC can still be used
to isolate peripherals to privileged code only (see Section 3.6.3).
When the TrustZone is activated, peripherals are set as nonsecure and unprivileged
after reset.

TrustZone-aware peripherals
The devices include the following TrustZone-aware peripherals:
•

GPIOA to GPIOJ, configured in LPGPIO alternate function or not

•

GTZCx_MPCBB, GTZCx_TZIC and GTZCx_TZSC (GTZC blocks)

•

OTFDEC1/2, writable only in secure if TZEN = 1

•

EXTI

•

Flash memory

•

RCC and PWR

•

GPDMA and LPDMA

•

SYSCFG registers

•

RTC and TAMP

•

MCU debug unit DBGMCU

•

ICACHE and DCACHE1

The way illegal accesses to these peripherals are monitored through the TZIC registers is
described in Section 5: Global TrustZone controller (GTZC).
For more details, refer to Section 3.5.5.

TrustZone illegal access controller (TZIC)
The TZIC block in GTZC gathers all illegal access events originated from sources either
protected by GTZC or TrustZone-aware peripherals, generating one global secure interrupt
towards the NVIC.
TZIC is available only when the system is TrustZone enabled (TZEN = 1). All accesses to
TZIC registers must be secured and privileged.

<!-- pagebreak -->

