363

Embedded flash memory (FLASH)

RM0456

Table 74. Flash mass erase versus privilege mode(1)
Unprivileged flash
memory

Access type
Mass erase

Privileged

Mass erase

Unprivileged

Privileged flash
memory

Mix unprivileged and
privileged flash memory

Ok
Ok

WI, secure or nonsecure WRPERR flag set

1. When TZEN = 1, access must be granted by security firewall before privilege is considered.

Table 75. SECyBBRx registers access when TrustZone is active (TZEN = 1)
Access type

Bit i in
PRIVyBBRx

Bit i in SECyBBRx

Fetch

Secure/nonsecure

Privileged/unprivileged

-

Bus error

Read

Secure/nonsecure

Privileged/unprivileged

-

Ok

Privileged

-

Ok

Unprivileged

0

Ok for bit i

Unprivileged

1

WI for bit i

Privileged/unprivileged

-

WI and a flash memory register illegal
access event

Secure
Write
Nonsecure

Table 76. PRIVyBBRx registers access when TrustZone is active (TZEN = 1)
Access type

Page secure state
(watermark or blocked
based)

Bit i in PRIVyBBRx

Fetch

Privileged/unprivileged

Secure/nonsecure

-

Bus error

Read

Privileged/unprivileged

Secure/nonsecure

-

Ok for all bits

Secured

-

Ok for all bits

Nonsecure

Nonsecure

Ok for bit i

Nonsecure

Secure

WI for bit i

Secure/nonsecure

-

WI for all bits

Privileged

Write

Unprivileged

Table 77. PRIVyBBRx registers access when TrustZone is disabled (TZEN = 0)
Access type

PRIVyBBRx

Fetch

Privileged/unprivileged

Bus error

Read

Privileged/unprivileged

Ok

Privileged

Ok

Unprivileged

WI

Write

<!-- pagebreak -->

