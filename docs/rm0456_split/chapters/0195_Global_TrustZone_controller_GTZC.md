RM0456 Rev 6

RM0456

Global TrustZone controller (GTZC)

5

Global TrustZone controller (GTZC)

5.1

GTZC introduction
This section describes the global TrustZone controller (GTZC) block that contains
the following subblocks:
•

TZSC: TrustZone security controller
This subblock defines the secure/privileged state of slave peripherals. It also controls
the sub-region area size and properties for the watermark memory peripheral controller
(MPCWM). The TZSC informs some peripherals (such as RCC or GPIOs) about the
secure status of each securable peripheral, by sharing with RCC and I/O logic.

•

MPCBB: memory protection controller - block based
This subblock configures the internal RAM in a TrustZone-system product having
segmented SRAM (pages of 512 bytes) with programmable-security and privileged
attributes.

•

TZIC: TrustZone illegal access controller
This subblock gathers all illegal access events in the system and generates a secure
interrupt towards NVIC.

These subblocks are used to configure TrustZone system security in a product having bus
agents with programmable-security and privileged attributes such as:

5.2

•

on-chip RAM with programmable secure and/or privileged blocks (pages)

•

AHB and APB peripherals with programmable security and/or privileged access

•

off-chip memories with secure and/or privileged areas

GTZC main features
The GTZC main features are listed below:
•

3 independent 32-bit AHB interface for TZSC, TZIC and MPCBB

•

TZIC accessible only with secure transactions

•

Secure and nonsecure access supported for privileged and unprivileged part of TZSC
and MPCBB

•

Set of registers to define product security settings:
–

Secure/privileged blocks for internal SRAMs

–

Secure/privileged regions for external memories and internal backup SRAM

–

Secure/privileged access mode for securable peripherals

–

Secure/privileged access mode for securable masters

GTZC TrustZone system architecture
The Armv8-M supports security per TrustZone-M model with isolation between:
•

a secure world, where usually security sensitive applications are run and critical
resources are located

•

a nonsecure or public world (such as usual nonsecure operating system and
user space)

RM0456 Rev 6

<!-- pagebreak -->

275

Global TrustZone controller (GTZC)

RM0456

The TrustZone architecture is extended beyond AHB and Armv8-M with:
•

AHB/APB bridge used as secure gate to block or propagate secure/nonsecure and
privileged/unprivileged transaction towards APB agents

•

PPC (peripheral protection controller) used as secure gate to block or propagate
secure/nonsecure and privileged/unprivileged transaction towards AHB agents

•

TrustZone block-based MPC firewalls used as secure gate to filter secure/nonsecure,
privileged/unprivileged access towards internal SRAMs

•

TrustZone watermark MPC firewalls used as secure gate to filter secure/nonsecure,
privileged/unprivileged access towards external memories

AHB and APB Peripherals can be categorized as:
•

privileged: peripherals protected by AHB/APB firewall stub that is controlled from
TZSC to define privilege properties

•

secure: peripherals always protected by an AHB/APB firewall stub. These peripherals
are always secure (such as TZIC)

•

securable: peripherals protected by an AHB/APB firewall stub that is controlled from
TZSC to define security properties (optional)

•

nonsecure and unprivileged: peripherals connected directly to AHB/APB
interconnect without any secure gate

•

TrustZone-aware: peripherals connected directly to AHB or APB bus and
implementing a specific TrustZone behavior (such as a subset of registers being
secure). TrustZone-aware AHB masters always drive HNONSEC signal according to
their security mode (such as Armv8-M core or DMA)

AHB securable masters can be configured in the TZSC to be secure/nonsecure and/or
privileged/unprivileged.

Application information
The TZSC, MPCBB and TZIC can be used in one of the following ways:

<!-- pagebreak -->

