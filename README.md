En stack er en virtuel arkitektur i ram hvor data kan blive husket og gemt indtil det skal videre til endestationen. man kan push og pop på en stacks data, man kan putte data oven på toppen og fjerne fra toppen, men man kan ikke fjerne noget indimellem (LIFO)
man kan tilgengæld peek for at se hvad data der ligger i midten.

&self i en metode kan kun læse (immutable)
hvorimod &mut self må ændre i data da det bliver mutable.

Selve Vec bliver gemt på stack, hvor at indholdet vec har bliver gemt på heapen, da det er en ukendt størrelse.

len er længden af vec altså hvor mange elementer der er i, hvis vec har 10 tal så er len 10
capacity starter på 4 og hvis len bliver 4 eller mere så bliver capacity fordoblet indtil len bliver det samme eller over capacity så fordobler det igen.


Vec kan vokse jo flere elementer der kommer i 'boxen', men vec har en størrelse på heapen,
som bliver fyldt på et tidspunkt, her er vec nødt til at finde et større sted den kan være
og så reallokere alle sine elementer over i den nye 'box'. vec størrelse fordobler tilgengæld
hver gang ved reallokering så det sker færre og færre gange.
Så det er ikke gratis at vokse på heapen hele tiden da computeren bruger ram plads på at 
reallokere sin data og laver en kopi af elementer under processen. 

ved preallokere kan man forudbestemme en vec størrelse hvis nu man ved at den skal bruge
1000 pladser, så skal den ikke fordoble hele vejen fra 4 til 8 til 16 til 32 osv hele vejen 
til der er plads til 1000 elementer.
ulempen ved dette er så at computeren ved der skal være plads til alle de elementer
og det kan ende med at fylde på RAM

