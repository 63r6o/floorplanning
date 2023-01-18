# Floorplanning
My final project for the Algorithms for VLSI class.

## Task description
>The project consists in placing a set of blocks in a 2D surface. The quality of the layout will be determined by the area of the bounding box and the total wire length. See https://en.wikipedia.org/wiki/Floorplan_(microelectronics).
The final layout will have to be a sliceable floorplan (representable as a binary tree). The floorplanning algorithm will allow the blocks to be rotated according to some shape functions defined a priori.
A possible strategy for the implementation of the algorithm is described in [^1].

_I have also used the book Electronic Design Automation [^2] as a reference_

## Usage
```bash
cargo run path/to/input/file
```
## Input
csv file, where each row is representing a module:
```
name of the module, width, height, is it rotatable?
```

pattern_generator.py can generate a random input:
```
python3 pattern_generator.py number_of_modules > output.csv
```

## Output
```
Starting expression
Starting Width
Starting Height
Starting floorplan area
Starting wirelength
THE OPTIMIZED FLOORPLAN
Width
Height
Floorplan area
Wirelength
```

## References

[^1]: D. F. Wong and C. L. Liu,
  "A New Algorithm for Floorplan Design,"
  23rd ACM/IEEE Design Automation Conference, 1986, pp. 101-107,
  doi: 10.1109/DAC.1986.1586075.

[^2]: Tung-Chieh Chen, Yao-Wen Chang,
  CHAPTER 10 - Floorplanning,
  Editor(s): Laung-Terng Wang, Yao-Wen Chang, Kwang-Ting (Tim) Cheng,
  Electronic Design Automation,
  Morgan Kaufmann,
  2009,
  Pages 575-634,
  ISBN 9780123743640,
  doi: 10.1016/B978-0-12-374364-0.50017-5.

