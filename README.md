Parser pour le markdown AE.

## Pourquoi ?

Parce que générer l'HTML à partir du markdown n'est pas une opération
excessivement couteuse, mais ça n'est pas non plus anodin.
Au final, on perd du temps qu'on aurait pu économiser.

De plus, l'interface est très simple : 
on donne une chaine de caractère à la fonction, on en récupère une autre.
Le processus est donc facile à isoler et à optimiser dans son coin.

Enfin, avoir un dépôt à part pour le code du parser
permet de l'intégrer plus facilement dans d'autres projets
et d'utiliser le markdown AE sans avoir à appeler tout le temps l'API du sith.