# Tiles and hexes

Start of by creating hexes that cover the entire screen with the `Hex` struct. 

It is these hexes that are shown on the screen, and are never actually changed.They are used for interaction with the underlying ``tiles``.

The ``tiles`` use the same coordinate system as the ``hexes``, however the amount of ``tiles`` can be much larger than the amount of `hexes`. 

This should be relatively simple since we can just add an offset to the `tile` coordinate system to match up the hexes and tiles.

I think this should minimize the amount of memory needed.



## TODO 06/08

### Main goals:


- [ ] Convert points and matrices to glam library for sppeeeed and simplicity
- [ ] Make camera struct
- [ ] Make camera movable
- [ ] Make big underlying tile table
- [ ] Make hexes represent underlying tile table

### If possible:

- [ ] Make bigger movement with camera by jumping "back" to center when camera has moved a whole tile.