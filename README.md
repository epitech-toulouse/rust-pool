# Exercices de programmation Rust

Ce dépôt contient une série d'exercices pour apprendre et pratiquer différents concepts en Rust.

## Structure du projet

```
src/
├── exercices/           # Code des exercices (à compléter par les élèves)
└── solutions/           # Implémentations de référence
```

## Consulter les solutions de référence

Chaque solution reste un programme Rust indépendant. Le lanceur `solutions` compile et exécute uniquement la solution demandée : les exercices inachevés des étudiants ne bloquent donc pas cette commande.

```bash
# Afficher tous les identifiants disponibles
cargo run --bin solutions -- list

# Compiler et exécuter une solution précise
cargo run --bin solutions -- day1/variables1
```

Le binaire compilé est placé dans `target/solution-runner/`. Le lanceur n'accepte que les identifiants affichés par `list`.

### Exercices

```bash
# Vérifier toute la piscine
cargo run --bin exercices -- check

# Vérifier un jour ou un exercice précis
cargo run --bin exercices -- check day1
cargo run --bin exercices -- check day1/variables1
```

Le vérificateur compile chaque fichier indépendamment. Il affiche `[OK]` en vert quand les tests passent, `[KO]` en rouge en cas d'erreur de compilation ou de test, et `[??]` en jaune lorsqu'aucun test embarqué ne peut confirmer le résultat. Utilisez `cargo run --bin exercices -- list` pour afficher les cibles disponibles.

## Compiler et tester un exercice

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

Les exercices sont volontairement compilés fichier par fichier : certains sont des squelettes à compléter et ne forment pas tous ensemble un programme Cargo compilable.

Pour vérifier le lanceur et son registre de solutions :

```bash
cargo test --bin solutions
```

Pour vérifier le vérificateur d'exercices :

```bash
cargo test --bin exercices
```
