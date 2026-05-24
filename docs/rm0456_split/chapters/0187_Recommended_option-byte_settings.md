RM0456 Rev 6

RM0456

System security

JTAG 32-bit device specific ID
Unless the JTAG port is deactivated (OEM2LOCK = 0 and RDP level = 2), a 32-bit device
specific quantity can always be read though the JTAG port. This information is stored in
DBGMCU_DBG_AUTH_DEVICE.
The OEM can use this 32-bit information to derive the expected OEM password keys to
unlock this specific device.

3.10.2

Recommended option-byte settings
Most of the time, the user threat model focuses mainly on software attacks. In this case,
it may be sufficient to keep the RDP level 1 as device protection.
For a more aggressive threat model, where the user fears physical attacks on the STM32
device, it is recommended to optimize the level of security by setting the RDP level 2.
The recommended settings are detailed below:
•

•

If TrustZone is disabled (TZEN = 0)
–

RDP level 2

–

nonsecure boot address option bytes set in user flash memory

If TrustZone is enabled (TZEN = 1)
–

RDP level 2

–

BOOT_LOCK = 1

–

secure boot address option bytes set in user secure flash memory

As described in the previous section, the customer can decide to allow any RDP level 2 part
to regress to RDP level 1, provided the OEM Key2 has been successfully provisioned, and
the OEM2LOCK option bit is set.

3.11

Access controlled debug
The device restricts access to embedded debug features, in order to guarantee the
confidentiality of customer assets against unauthorized usage of debug and trace features.

3.11.1

Debug protection with readout protection (RDP)
As described in Section 3.10.1, the hardware RDP mechanism automatically controls the
accesses to the device debug and test. The protection of these debug features is defined in
the table below. Possible password-based regressions are described in Section 3.10.1.
Table 23. Debug protection with RDP

RDP protection level
Level 0

Device open

Debug features protection
Any debug(1)

Level 0.5(2) Device partially closed Secure debug is no longer available.
Level 1

Device memories
protected

Nonsecure debug can no longer debug code and data stored in the
embedded flash memory, the encrypted external flash memory(3), SRAM2,
and backup registers.

Level 2

Device closed

JTAG is physically deactivated, unless it is kept operational only for password
key injection (OEM2LOCK = 1). See Section 3.10.1 for details.

RM0456 Rev 6

<!-- pagebreak -->

191

System security

RM0456

1. Including ST engineering test modes, used for field returns.
2. Only applicable when TrustZone security is activated in the product.
3. External flash memory area decrypted on-the-fly with the OTFDEC.

3.12

Software intellectual property protection and collaborative
development
Thanks to the software intellectual property protection and collaborative model, the devices
allow the design of solutions integrating innovative third-party libraries.
Collaborative development is summarized on the figure below. Starting from a personalized
device sold by STMicroelectronics, a vendor A can integrate a portion of hardware and
software on a platform A, that can then be used by a vendor B, who does the same before
deploying a final product to the end users.

Note:

Each platform vendor can provision individual platforms for development not to be
connected to a production cloud network (“Development Platform X”).

User states
Vendor states

Figure 16. Collaborative development principle

Development platform B

Use

STM32 personalized
device

Development platform B

Platform part

Vendor A manufacturing

r pro

visio

ed

t
gra

e

ning

Platform A

Int

Platform part

Vendor B manufacturing

Use

r pro

visio

ning

ed
rat

eg

Platform B

Int

tion

ctiza

Deployed product

u
Prod

Decommissioning

Decommissioned product
MSv64455V1

The features described hereafter contribute to securing the software intellectual property
within such a collaborative development.

<!-- pagebreak -->

