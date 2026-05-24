RM0456 Rev 6

RM0456

Embedded flash memory (FLASH)

7.9.29

FLASH secure block based bank 1 register x (FLASH_SECBB1Rx)
Address offset: 0x80 + 0x4 * (x - 1), (x = 1 to 8)
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register is secure. It can be written only by secure access. This register can be
protected against unprivileged access (refer to Table 75).

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

SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B
B31
B30
B29
B28
B27
B26
B25
B24
B23
B22
B21
B20
B19
B18
B17
B16
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

rw

rw

rw

rw

rw

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

SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B SEC1B
B15
B14
B13
B12
B11
B10
B9
B8
B7
B6
B5
B4
B3
B2
B1
B0
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

rw

rw

rw

rw

rw

Bits 31:0 SEC1BBi: page secure/nonsecure attribution (i = 31 to 0)
Each bit is used to set one page security attribution in bank 1.
0: Page (32 * (x - 1) + i) in bank 1 not block-based secure
1: Page (32 * (x - 1) + i) in bank 1 block-based secure

7.9.30

FLASH secure block based bank 2 register x (FLASH_SECBB2Rx)
Address offset: 0xA0 + 0x4 * (x - 1), (x = 1 to 8)
Reset value: 0x0000 0000
Access: no wait state; word, half-word, and byte access
This register is secure. It can be written only by a secure access. This register can be
protected against unprivileged access (refer to Table 75).

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

SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B
B31
B30
B29
B28
B27
B26
B25
B24
B23
B22
B21
B20
B19
B18
B17
B16
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

rw

rw

rw

rw

rw

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

SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B SEC2B
B15
B14
B13
B12
B11
B10
B9
B8
B7
B6
B5
B4
B3
B2
B1
B0
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

rw

rw

rw

rw

rw

Bits 31:0 SEC2BBi: page secure/nonsecure attribution (i = 31 to 0)
Each bit is used to set one page security attribution in bank 2.
0: Page (32 * (x - 1) + i) in bank 2 not block-based secure
1: Page (32 * (x - 1) + i) in bank 2 block-based secure

RM0456 Rev 6

<!-- pagebreak -->

