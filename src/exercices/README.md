# Exercices - Comment compiler et exécuter

Pour travailler sur les exercices, utilisez la commande suivante :

```bash
rustc --edition 2021 chemin/vers/exercice.rs -o exercice
./exercice
```

Pour lancer les tests embarqués dans un fichier :

```bash
rustc --edition 2021 chemin/vers/exercice.rs --test -o exercice-tests
./exercice-tests
```

Pour lancer une solution de référence sans compiler les autres exercices :

```bash
cargo run --bin solutions -- list
cargo run --bin solutions -- day1/variables1
```

Pour vérifier tous les exercices avec les tests embarqués :

```bash
cargo run --bin exercices -- check
```

Un exercice dont les tests passent est affiché en vert. Une erreur de compilation ou de test est affichée en rouge. Les exercices sans test restent jaunes et demandent une vérification manuelle de leur sortie.
