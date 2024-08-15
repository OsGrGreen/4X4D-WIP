# Tiles and hexes

Start of by creating hexes that cover the entire screen with the `Hex` struct. 

It is these hexes that are shown on the screen, and are never actually changed.They are used for interaction with the underlying ``tiles``.

The ``tiles`` use the same coordinate system as the ``hexes``, however the amount of ``tiles`` can be much larger than the amount of `hexes`. 

This should be relatively simple since we can just add an offset to the `tile` coordinate system to match up the hexes and tiles.

I think this should minimize the amount of memory needed.


## TODO 06/08

### Main goals:


- [ ] Convert points and matrices to glam library for sppeeeed and simplicity
- [x] Make camera struct
- [x] Make camera movable
- [x] Make big underlying tile table
- [x] Make hexes represent underlying tile table

### If possible:

- [x] Make bigger movement with camera by jumping "back" to center when camera has moved a whole tile.

## TODO next

### Main goals

- [ ] Create line outlines for each hex, to make each hex pop out more.
- [ ] Create "random" terrain generation see
    - https://gamedev.stackexchange.com/questions/16541/random-map-generation-strategies-for-scattering-clustering-random-nodes
    - https://www.youtube.com/watch?v=-H01z7cdOW0
    - https://www.youtube.com/watch?v=JzyOWGoB4t8
    - https://www.youtube.com/watch?v=3t4W-E0PKUE
    - https://www.researchgate.net/publication/233732668_Procedural_Map_Generation_for_a_RTS_Game
    - https://www.youtube.com/watch?v=j-rCuN7uMR8&list=PLbghT7MmckI7JHf0pdEQ8fbPb-LoDXEno
- [ ] Make all random constants now actually be grounded in real numbers...
    - Like the width of the screen or how many hexes should be generated and so on.
- [ ] Disable V-sync?
