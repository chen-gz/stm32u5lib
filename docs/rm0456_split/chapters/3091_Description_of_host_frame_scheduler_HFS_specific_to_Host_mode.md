3291

USB on-the-go full-speed (OTG_FS)

RM0456

72.15.22 OTG host frame interval register (OTG_HFIR)
Address offset: 0x404
Reset value: 0x0000 EA60
This register stores the frame interval information for the current speed to which the
OTG_FS controller has enumerated.
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
RLD
CTRL

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

rw

rw

rw

rw

rw

rw

rw

rw

FRIVL[15:0]
rw

rw

rw

rw

rw

rw

rw

rw

rw

Bits 31:17 Reserved, must be kept at reset value.
Bit 16 RLDCTRL: Reload control
This bit allows dynamic reloading of the HFIR register during run time.
0: The HFIR cannot be reloaded dynamically
1: The HFIR can be dynamically reloaded during run time.
This bit needs to be programmed during initial configuration and its value must not be
changed during run time.
Caution: RLDCTRL = 0 is not recommended.
Bits 15:0 FRIVL[15:0]: Frame interval
The value that the application programs to this field, specifies the interval between two
consecutive SOFs (FS) or Keep-Alive tokens (LS). This field contains the number of PHY
clocks that constitute the required frame interval. The application can write a value to this
register only after the port enable bit of the host port control and status register (PENA bit in
OTG_HPRT) has been set. If no value is programmed, the core calculates the value based
on the PHY clock specified in the FS/LS PHY clock select field of the host configuration
register (FSLSPCS in OTG_HCFG). Do not change the value of this field after the initial
configuration, unless the RLDCTRL bit is set. In such case, the FRIVL is reloaded with each
SOF event.
– Frame interval = 1 ms × (FRIVL - 1)

<!-- pagebreak -->

