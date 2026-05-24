191

System security

RM0456

The hide protection principle is pictured on the figure below.

Secure area

Figure 12. Flash memory secure HDP area

User
applications

User
applications

Secure
Flash memory
applications

Secure
Flash memory
applications

Secure boot
code and data
(HDP)

Hidden

2) Jump to secure code
and hide area

1) Execute after reset
MSv64452V2

When the HDPxEN and HDPx_ACCDIS bits (x = 1, 2) are set, data read, write, and
instruction fetch on the area defined by SECWMx_PSTRT and HDPx_PEND option bytes,
are denied until the next device reset.
Note:

Bank erase aborts when it contains a write-protected area (WRP or HDP area).
The HDP area can be resized by a secure application if the area is not hidden, and
if RDP level ≠ 2.

3.6.2

RSSLIB functions
The RSS provides runtime services thanks to RSS library. As other microcontroller
peripherals features and mapping, the RSS library functions are exposed to user within the
CMSIS device header file provided by the STM32Cube firmware package. Please refer to
UM2656 to get more details regarding STM32Cube firmware package. RSS library functions
are named RSSLIB functions hereafter.
The user firmware calls RSSLIB functions using RSSLIB_PFUNC C defined macro, that
points to a location within nonsecure system memory. Hence prior calling RSSLIB functions,
the secure user firmware must define a nonsecure region above this location within SAU of
the Cortex®-M33. This nonsecure region starts from
RSSLIB_SYS_FLASH_NS_PFUNC_START up to
RSSLIB_SYS_FLASH_NS_PFUNC_END. These last addresses are provided within the
CMSIS device header file. The user can set this nonsecure region either by using the
CMSIS system partition header file or by implementing its own code for SAU setup. The
CMSIS system partition header file is part of the STM32Cube firmware package.
RSSLIB functions are split between non-secure callable and secure callable function.
The RSS library functions are described within sections hereafter.

<!-- pagebreak -->

