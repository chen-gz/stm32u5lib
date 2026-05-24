191

System security

RM0456

Software filtering mechanism
Each tamper source can be configured not to launch an immediate erase, by setting
the corresponding TAMPxNOER bit in TAMP_CR2 (for external tamper pin) or TAMP_CR3
(for internal tamper).
In such situation, when the tamper flag is raised, the access to below secrets is blocked until
all tamper flags are cleared:
•

DHUK in SAES: fixed to a dummy value

•

Backup registers, backup SRAM, SRAM2: read-as-zero, write-ignored

•

AES, SAES, and HASH peripherals: automatically reset by RCC

•

PKA peripheral: reset, with memory use blocked (meaning PKA not usable)

Once the application, notified by the tamper event, analyzes the situation, there are two
possible cases:

Note:

•

The application launches secrets erase with a software command (confirmed tamper).

•

The application just clears the flags to release secrets blocking (false tamper).

If the tamper software fails to react to such a tamper flag, an IWDG reset triggers
automatically the erase of secrets.

Tamper detection and low-power modes
The effect of low-power modes on a tamper detection are summarized on the table below.
Table 18. Effect of low-power modes on TAMP
Mode

Description

Sleep

No effect on tamper detection features
TAMP interrupts cause the device to exit Sleep mode.

Stop

No effect on tamper detection features, except for level detection with filtering and active tamper modes
that remain active only when the clock source is LSE or LSI
Tamper events cause the device to exit Stop mode.

Standby

No effect on tamper detection features, except for level detection with filtering and active tamper
modes, which remain active only when the clock source is LSE or LSI
Tamper events cause the device to exit Standby mode.

No effect on tamper detection features, except for level detection with filtering and active tamper
Shutdown modes, which remain active only when the clock source is LSE
Tamper events cause the device to exit Shutdown mode.

3.8

Secure storage
A critical feature of any security system is how long-term keys are stored, protected, and
provisioned. Such keys are typically used for loading a boot image, or handling of critical
user data.
Figure 13 shows how the key management service application can use the AES engine, for
example, to compute external image decryption keys. A nonvolatile key can be stored in the
embedded secure HDP area (see Section 3.6.1), while volatile key storage consists in the
battery-powered, tamper-protected SRAM or registers in TrustZone-aware TAMP.

<!-- pagebreak -->

