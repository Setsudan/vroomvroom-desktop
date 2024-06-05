### Énoncé de Problème

Vous avez une voiture équipée de quatre roues motrices indépendantes. La vitesse de chaque roue peut être contrôlée individuellement, avec une plage de vitesse allant de -4095 (recul maximum) à 4095 (avancement maximum), et une vitesse de 0 correspondant à une roue à l'arrêt.

Problème : La voiture ne peut pas incliner ou tourner ses roues. Pour effectuer un virage, il est nécessaire de modifier la vitesse de certaines roues. Deux propositions ont été faites pour gérer cette situation :

1. Faire rouler plus lentement les roues intérieures au virage d'un certain pourcentage par rapport à la vitesse des roues extérieures, en fonction de l'intensité du virage.
2. Faire rouler plus lentement la roue avant intérieure et la roue arrière extérieure, tout en gardant la même vitesse pour les deux autres roues.

**Questions :**
1. Analysez les avantages et les inconvénients de chacune des propositions en termes de dynamique et de stabilité du véhicule.
2. Déterminez si l'une des propositions est meilleure que l'autre, ou proposez une méthode alternative pour effectuer un virage en modifiant les vitesses des roues.

**Données et hypothèses :**
- La voiture est supposée évoluer sur une surface plane et adhérente.
- Les variations de vitesse nécessaires pour effectuer un virage sont proportionnelles à l'angle de virage souhaité.
- L'angle de virage est constant pour une durée suffisante pour permettre une analyse en régime permanent.

**Formulation mathématique :**

Pour chaque proposition, modélisez les vitesses des quatre roues en fonction de la vitesse de référence \( V \) (la vitesse des roues extérieures dans le cas de la première proposition) et du pourcentage de réduction \( p \) pour les roues intérieures.

1. **Proposition 1 :** 
   - Vitesse des roues extérieures \( V_{ext} = V \)
   - Vitesse des roues intérieures \( V_{int} = V \times (1 - p) \)

2. **Proposition 2 :** 
   - Vitesse des roues avant intérieure \( V_{av-int} = V \times (1 - p) \)
   - Vitesse des roues arrière extérieure \( V_{ar-ext} = V \times (1 - p) \)
   - Vitesse des roues avant extérieure et arrière intérieure \( V_{av-ext} = V_{ar-int} = V \)

**Analyse de la stabilité :**

- Considérez les forces latérales et longitudinales sur chaque roue.
- Utilisez les équations de la dynamique pour évaluer la stabilité du véhicule (par exemple, équilibre des moments, adhérence des pneus).
- Discutez de la répartition de la charge sur les roues et de l'impact sur l'adhérence.

Proposez une solution optimale basée sur cette analyse et discutez de la faisabilité pratique de la mise en œuvre de chaque proposition.