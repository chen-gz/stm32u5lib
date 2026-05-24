191

System security

RM0456

Details on the password-based regression can be found in the table below.
Table 22. OEM1/2 RDP unlocking methods
OEM1 password options
OEM1
Initial
LOCK RDP level

Regression to level 0
possible only through
OEM1 unlock
sequence (see below)

1
1

Regression to level 0
always granted

0

•

•

•

Note:

RDP regression

OEM2 password options
OEM2
Initial
LOCK RDP level
1

Regression to level 0.5 possible only through
OEM2.1 unlock sequence (see below)

2

Automatic regression to level 1 triggered upon
successful OEM2.2 unlock sequence
(see below)

1

Regression to level 0.5 always granted

2

Regression to level 1 never granted
RDP remains a permanent state.

1

0

RDP regression

OEM1 unlock sequence, starting at RDP level 1:
–

Shift the password key through JTAG/SWD (see the note below).

–

If this key matches the OEM1KEY provisioned in the device, the application can
trigger a regression sequence to level 0. After the regression is completed, the
whole embedded flash memory and device secrets are erased. The OTP area is
not erased.

–

In case of mismatch value, the RDP regression is blocked and RDP level 1
protections are enforced until the next power-on reset.

OEM2.1 unlock sequence, starting at RDP level 1:
–

Shift the password key through JTAG/SWD under reset (see the note below).

–

If this key matches the OEM2KEY provisioned in the device, the application can
trigger a regression sequence to level 0.5. After the regression is completed, the
nonsecure embedded flash memory and device secrets are erased. The OTP
area is not erased.

–

In case of mismatch value, the RDP regression is blocked and RDP level 1
protections are enforced until the next power-on reset.

OEM2.2 unlock sequence, starting at RDP level 2:
–

Shift the password key through JTAG/SWD under reset (see the note below).

–

If this key matches the OEM2KEY provisioned in the device, the device
automatically triggers a regression sequence to level 1. After the regression is
completed, a power-on reset has to be performed by the user.

–

In case of mismatch value, the RDP regression is blocked and RDP level 2
protections are enforced until the next power-on reset.

Unlocking the device with a password is possible only once per power cycle.
Shifting the password key through JTAG/SWD corresponds to writing two 32-bit key words,
AUTH_KEY[31:0], then AUTH_KEY[63:32], in the DBGMCU_DBG_AUTH_HOST register.

<!-- pagebreak -->

