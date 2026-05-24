•

resource isolation using TrustZone (see Section 3.5)

•

temporal isolation using hide protection (see Section 3.6.1)

•

secure execution (see Section 3.7)

•

secure install and update (see Section 3.2 and Section 3.4)

RM0456 Rev 6

RM0456

System security
•

secure storage, with associated cryptographic engines if available (see Section 3.8 and
Section 3.9)

This section describes the features specifically designed for secure boot.

3.3.1

Unique boot entry and BOOT_LOCK
When TrustZone is activated (TZEN = 1) and the BOOT_LOCK secure option bit is cleared,
the application selects a boot entry point located either in the system flash memory (see the
next section), or in the secure user flash memory, at the address defined by
SECBOOTADD0 option bytes.
When TrustZone is activated (TZEN = 1) and the BOOT_LOCK secure option bit is set, the
device unique boot entry is the unmodifiable secure address defined by SECBOOTADD0
option bytes. All these option bytes cannot be modified by the application anymore when
BOOT_LOCK is set.

Note:

As long as it is cleared, the BOOT_LOCK option bit can be set without any constraint. But
once set, the BOOT_LOCK option bit cannot be cleared when RDP level > 0.
For more information on the boot mechanisms, refer to Section 4: Boot modes.

3.3.2

Immutable root of trust in system flash memory
The immutable root-of-trust code stored in the system flash memory is first used to initiate
the SFI, allowing secure and counted installation of OEM firmware in the untrusted
production environment (such as the OEM contract manufacturer).
The STMicroelectronics immutable code also includes secure runtime services that can be
called at runtime when a secure application sets the SYSCFG_RSSCMDR register to a
non-null value before triggering a system reset. This runtime feature is deactivated when the
BOOT_LOCK secure option bit is set, and the secure address defined by SECBOOTADD0
is set on the secure user flash memory.

3.4

Secure update
The secure firmware update is a secure service that runs after a secure boot. Its actual
functions depend on the availability of the TrustZone features, and on the firmware stored in
the device.
The device trusted firmware-M (TFM) application, supported by the STM32 ecosystem,
allows the update of the microcontroller built-in program with new firmware versions, adding
new features and correcting potential issues. The update process is performed in a secure
way to prevent unauthorized updates and access to confidential on-device data.
A firmware update can be done either on a single firmware image including both secure and
nonsecure parts, or on the secure (respectively nonsecure) part of the firmware image,
independently.
In the devices, the secure update application leverages the same hardware security as the
firmware install described in Section 3.2. For more information, refer to the user manual:
Getting started with STM32CubeU5 TFM application (UM2851).

RM0456 Rev 6

<!-- pagebreak -->

