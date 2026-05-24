1861

Neo-Chrom graphic processor (GPU2D)

45

RM0456

Neo-Chrom graphic processor (GPU2D)
This section only applies to STM32U599/5A9 and STM32U5Fx/5Gx devices.

45.1

GPU2D introduction
GPU2D is a dedicated graphics processing unit accelerating numerous 2.5D graphics
applications such as graphical user interface (GUI), menu display or animations.
The GPU2D peripheral works together with an optimized software stack designed for state
of the art graphic rendering.

45.2

GPU2D main features
Main features
•

Multithreaded fragment (pixel) processing core with a VLIW (very long instruction word)
instruction set

•

Fixed-point functional units

•

Command-list-based DMAs to minimize CPU overhead

•

Two 32-bit AHB master interfaces for texture, command list and frame buffer access

•

32-bit AHB slave interface for register bank access

•

Up to four general-purpose flags for system-level synchronization

•

Texture decompression unit with TSC™4 and TSC™6/TSC™6a support

2D drawing features
•

Pixel/line drawing

•

Filled rectangles

•

Triangles, quadrilateral drawing

•

Anti-aliasing 8xMSAA (multisample anti-aliasing)

Vector graphic acceleration
•

Path drawing (lines, polygons, rectangles, arcs, ellipses, circles)

•

Bezier curves (cubic and quadratic)

•

Path transformation (3x3 matrix)

•

Path stroking

•

Filling (event-odd and nonzero with 8x MSAA anti-aliasing)

•

Gradient generation (linear, radial, conic)

Image transformations

<!-- pagebreak -->

