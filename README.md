# la simulation

- la cible de consomation en gwh est une input
- production journalière
- l'ordre de consommation permet de faire une demande dans l'ordre de production
    - trie par co2/kwh
    - une option permet de choisir si le nuc est défavorisé ou non
- Demande par centrale
    - demander la production à venir
    - si c'est une ENR, il faut déterminer la production journalière possible en fonction du TC. C'est le même pour le même type de central (nuage ou pas de vent partout, ce qui est vrai à 80%, on peut compenser avec un tc un peu plus haut 30% au lieu de 20%), la production ENR est donc déterminer ici.
    - la prod non ENR utilise le même genre de tc mais de façon indépendante pour gérer la planification.