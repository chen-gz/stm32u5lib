RM0456 Rev 6

RM0456

System security
More details on the various phases and associated transitions, found either at the vendor or
end-user premises, are summarized in the table below.
Table 20. Main product life-cycle transitions

Transitions

Description

Device
manufacturing

STMicroelectronics creates new STM32 devices, always checking for manufacturing defects.
During this process STM32 is provisioned with ROM firmware, secure firmware install (SFI)
unique key pair, and a public ID.

Vendor
manufacturing

One (or more) vendor is responsible for the platform assembly, initialization, and provisioning
before delivery to the end user. This end user can use the final product (“production” transition)
or he/she can use the platform for software development (“user provisioning” transition).

Production

The end-user gets a product ready for use. All security functions of the platform are enabled, the
debugging/testing features are restricted/disabled, and unique boot entry to immutable code is
enforced.

User provisioning

Platform vendor prepares an individual platform for development, not to be connected to a
production cloud network.

These are one-way transitions, with devices kept in user premises or returned to the
Field return or
manufacturer. In both cases, all data including user data is destroyed, therefore the devices lose
decommissioning
the ability to operate securely (like connecting to a managed IoT network).

The features described hereafter contribute to secure the device life-cycle.

RM0456 Rev 6

<!-- pagebreak -->

