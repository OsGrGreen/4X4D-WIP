# Story/lore/ide

Världen har preis börjat att gå under, genom spelets gång kommer kartan (Världen) att förfalla mer och mer, och kartan kommer bli tommare och tommare. Världens hålls ihop av fyra (kanske ändra) "ankare", men för varje som går sönder desto snabbare kommer undergången.
De gamla gudarna (tänker typ cthulu mythos men gör om) börjar vakna allt mer till liv, och genom att få kraft från de kan man säkerställa sin överlevnad. 
Man spelar främst som ett imperium som redan är stort (är i sin "stormakts" tid), och har mycket resurser. Men dessa imperium kommer då förfalla allt mer och mer.


# Mer gameplay ide

I början av varje "match" så får man placea ut någon/några städer och hur stort/vart man vill ha sin influence (sina borders från CIV). 
EFter det här kan man fortfarande expandera men är svårt att göra. 
Efter x-antal turns kommer tiles börja försvinna från kartan (som man kan få tech att röra sig över, men ger inga resources). Man kan spendera eller bygga saker för att skydda/ändra vilka tiles som ska försvinna. 
Då resurser blir allt svårare att få tag på måste man spara mycket och göra val ifall man vill spendera de nu (och få fördel) eller spara de och överleva längre och kanske göra bättre saker med dem. 

Man kan kommunicera med de old gods och be/offra saker till dem för krafter. Men de old gods har olika relations så tillfredställer man en blir en annan sur osv..


## Map

Hexagonal, but with quite small tiles. One building or unit can take up multiple tiles.

Alla tiles av samma typ ger samma antal resurser, om de inte är improved

Each tile is represented as one byte:

En tile är en byte, där det är
3 bits = biome
1 bit = improved
3 bit = rescource
1 bit = occupied
