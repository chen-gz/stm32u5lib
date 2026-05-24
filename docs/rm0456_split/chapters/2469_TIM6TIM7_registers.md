2577

Independent watchdog (IWDG)

61.7.3

RM0456

IWDG reload register (IWDG_RLR)
Address offset: 0x08
Reset value: 0x0000 0FFF

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

RL[11:0]
rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 RL[11:0]: Watchdog counter reload value
These bits are write access protected, see Section 61.4.6. They are written by software to
define the value to be loaded in the watchdog counter each time the value 0xAAAA is written
in the IWDG key register (IWDG_KR). The watchdog counter counts down from this value.
The timeout period is a function of this value and the prescaler.clock. It is not recommended
to set RL[11:0] to a value lower than 2.
The RVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the
reload value.
Note: Reading this register returns the reload value from the VDD voltage domain. This value
may not be up to date/valid if a write operation to this register is ongoing, hence the
value read from this register is valid only when the RVU bit in the IWDG status register
(IWDG_SR) is reset.

61.7.4

IWDG status register (IWDG_SR)
Address offset: 0x0C
Reset value: 0x0000 0000

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

EWIF

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

EWU

WVU

RVU

PVU

r

r

r

r

r

Bits 31:15 Reserved, must be kept at reset value.
Bit 14 EWIF: Watchdog early interrupt flag
This bit is set to ‘1’ by hardware in order to indicate that an early interrupt is pending. This bit
must be cleared by the software by writing the bit EWIC of IWDG_EWCR register to ‘1’.
Bits 13:4 Reserved, must be kept at reset value.
Bit 3 EWU: Watchdog interrupt comparator value update
This bit is set by hardware to indicate that an update of the interrupt comparator value
(EWIT[11:0]) or an update of the EWIE is ongoing. It is reset by hardware when the update
operation is completed in the VDD voltage domain. Refer to Table 623: IWDG delays versus
actions for delay values.
The EWIT[11:0] and EWIE fields can be updated only when EWU bit is reset.

<!-- pagebreak -->

RM0456 Rev 6

RM0456

Independent watchdog (IWDG)

Bit 2 WVU: Watchdog counter window value update
This bit is set by hardware to indicate that an update of the window value is ongoing. It is
reset by hardware when the reload value update operation is completed in the VDD voltage
domain. Refer to Table 623: IWDG delays versus actions for delay values.
The window value can be updated only when WVU bit is reset.
This bit is generated only if generic “window” = 1.
Bit 1 RVU: Watchdog counter reload value update
This bit is set by hardware to indicate that an update of the reload value is ongoing. It is reset
by hardware when the reload value update operation is completed in the VDD voltage
domain. Refer to Table 623: IWDG delays versus actions for delay values.
The reload value can be updated only when RVU bit is reset.
Bit 0 PVU: Watchdog prescaler value update
This bit is set by hardware to indicate that an update of the prescaler value is ongoing. It is
reset by hardware when the prescaler update operation is completed in the VDD voltage
domain. Refer to Table 623: IWDG delays versus actions for delay values.
The prescaler value can be updated only when PVU bit is reset.

Note:

If several reload, prescaler, early interrupt position or window values are used by the
application, it is mandatory to wait until RVU bit is reset before changing the reload value, to
wait until PVU bit is reset before changing the prescaler value, to wait until WVU bit is reset
before changing the window value, and to wait until EWU bit is reset before changing the
early interrupt position value. After updating the prescaler and/or the reload/window/early
interrupt value, it is not necessary to wait until RVU or PVU or WVU or EWU is reset before
continuing code execution, except in case of low power mode entry.

61.7.5

IWDG window register (IWDG_WINR)
Address offset: 0x10
Reset value: 0x0000 0FFF

31

30

29

28

27

26

25

24

23

22

21

20

19

18

17

16

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

Res.

15

14

13

12

11

10

9

8

7

6

5

4

3

2

1

0

Res.

Res.

Res.

Res.
rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

rw

WIN[11:0]
rw

Bits 31:12 Reserved, must be kept at reset value.
Bits 11:0 WIN[11:0]: Watchdog counter window value
These bits are write access protected, see Section 61.4.6.They contain the high limit of the
window value to be compared with the downcounter.
To prevent a reset, the IWDCNT downcounter must be reloaded when its value is lower than
WIN[11:0] + 1 and greater than 1.
The WVU bit in the IWDG status register (IWDG_SR) must be reset to be able to change the
reload value.
Note: Reading this register returns the reload value from the VDD voltage domain. This value
may not be valid if a write operation to this register is ongoing. For this reason the value
read from this register is valid only when the WVU bit in the IWDG status register
(IWDG_SR) is reset.

RM0456 Rev 6

<!-- pagebreak -->

