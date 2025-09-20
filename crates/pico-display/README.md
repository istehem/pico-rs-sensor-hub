# Scaling a die on a 265×265 pixel display

When scaling dice pip dimensions to a 265×265 pixel display (approximately 265 pixels per inch),
the proportions of a standard American die can be approximated as follows:

Die face size: The entire die face would occupy roughly 265×265 pixels, representing a physical
size of about 0.75 inches (19 mm) per side, which is standard for precision dice.

## Pip (dot) placement

On a standard six-sided die, the pips are arranged in specific symmetric patterns:

The center pip (on faces 1, 3, and 5) is placed at the exact center: (132.5, 132.5) pixels.

The corner pips (on faces 2, 3, 4, 5, 6) are typically positioned about 20–25% in from each edge.
For a 265-pixel grid, this places them around 53 to 66 pixels from each edge.


## For example:

```text

    Face-by-face layout:

    Face 1 (center dot):

        Center: (132, 132)

    Face 2 (diagonal corners):

        Top-left: (66, 66)

        Bottom-right: (198, 198)

    Face 3 (adds center to face 2):

        Adds: (132, 132)

    Face 4 (four corners):

        Top-left: (66, 66)

        Top-right: (198, 66)

        Bottom-left: (66, 198)

        Bottom-right: (198, 198)

    Face 5 (four corners + center):

        All of face 4 + (132, 132)

    Face 6 (two columns):

        Left column (x ≈ 66):
        (66, 66), (66, 132), (66, 198)

        Right column (x ≈ 198):
        (198, 66), (198, 132), (198, 198)
```

Pip size: On a physical die, pips are drilled about 0.43 mm deep and sized proportionally to the face. On a 265-pixel display, a typical pip diameter would be 31 to 39 pixels (odd numbers are better), ensuring visibility and balance within the face.

Centering: Yes, pips are always symmetrically centered relative to their positions on the face. The overall pattern is carefully aligned so that the visual weight is balanced, especially in precision or casino dice.

Other characteristics: In digital representation, the pips are often filled with solid color (traditionally white or red on a black or colored background), and the edges of the die may be slightly rounded or shaded to simulate 3D depth.

These proportions preserve the standard look and symmetry of American-style dice while adapting them to a pixel-based grid.


## Why Odd Grids & Odd Sizes Help

A 265×265 grid has a true center pixel at (132, 132).
A 33-pixel diameter circle has radial symmetry when centered at integer coordinates.
Using even diameters (e.g., 32 px) forces the center between pixels, causing slight blurring or asymmetry in raster rendering.

