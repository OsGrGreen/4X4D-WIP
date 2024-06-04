# Mechanics


Explore, Expand, Exploit, Exterminate
Decline, Destruction, Dissolution, Division

## Combat

### Units

## Division

## Gods

## Civil wars

##  Resources/Statistics

TODO: Make subsections from these statistics

- Food
- Production (Should be able to be saved in some way, since the name of the game is saving stuff)
- Science (Is science needed?)
- Natural resources
    - Minerals
        - Iron
        - Copper
        - And so on
    - Luxuouries
    - Mana/Magic stuff
- Faith/Offerings
- Money
- Orders (the amount of stuff you can do in a turn) (the bigger the empire, maybe you can issue fewer orders, since more bureaucracy?)

Resources are gained mainly in three ways. From gathering it from tiles, from creating it in buildings and stealing it from others. 

There should be an incentive for players to steal it from others (Should not be massivly penalized). An incentive could be that worked tiles slowly "deplete", making them unusable in gathering new resources.

Stockpiles are also needed to store these resources. Some can be stored in cities, however others needs to be placed on tiles. If that tile is lost or raided you lose a percentage of the resources stored in that stockpile. You can move around resources in stockpiles, however it is not instant and needs to be done over time.

These resources can then be spent on creating new things.
  

## Tiles

Each tile should be tiny. Essentially making bigger tiles from the smaller ones. If added in code, this could help speedup AI choices and world generation since each big tile should be semi-coherent. 

Goal is to make the tile system relatively simple. This is to minimize micro managment, since it can be hard, frustrating, time consuming and frankly quite boring. 

Each tile can essentially have two states. Natural or improved.

An improved tile gives more of the same resources for a [[standard tile]], or the corresponding material that tile provides (which it does not before it is improved). 

Since the goal is to have small tiles, each resource can span multiple tiles. The goal is that this creates both cooperation but also hostility. Since taking every vein of a resource can be quite destructive for others, but not much is needed for any player to snag one tile. 

### Biomes

Maybe it is stupid to have each biome type have a specific focus/thing they are better at

(Biome: Focus)

- Desert: Science
- Woods: Production
- Plains: Food
- Hills: General
- Mountains: Money
- Sea: General
- Void: Nothing (if not specific faction) or maybe Faith (since the more void you control the bigger the connection with the gods)
___________________________
City: General
Improvement: Specific

General contains:
- Production
- Food
- Money

### Cities

For inspiration look at Pandora: First Contact

### Size

20 x 20 pixels (would work on 16:9 monitors, most of the time atleast), however problems with tileability may occur.
By just rendering enough hexagons to fill the screen, then update how that hexagon should look based on were the camera is in the world. This should make rendering faster since on a standard 1920x1080 monitor only 5184 tiles need to be rendered at a time. However, this could mean that more data needs to be transferred between CPU and GPU to update the look of each tile, however more data for each rendered tile does not necessarily mean more data in total. So could also be a way to make things faster. Furthermore, this potentially needs a more robust camera/data transfer system since we always need to know which tiles are visible from a given point.

World should loop (very important)!

In total there should not be an overly big world. However, big enough so that not one screen can cover it all. You have to scroll around to see it all. 


## Stretch goals

### City states

### Designing units

### Leaders

### Deep trade mechanics
