# Tiles and hexes

Start of by creating hexes that cover the entire screen with the `Hex` struct. 

It is these hexes that are shown on the screen, and are never actually changed.They are used for interaction with the underlying ``tiles``.

The ``tiles`` use the same coordinate system as the ``hexes``, however the amount of ``tiles`` can be much larger than the amount of `hexes`. 

This should be relatively simple since we can just add an offset to the `tile` coordinate system to match up the hexes and tiles.

I think this should minimize the amount of memory needed.
