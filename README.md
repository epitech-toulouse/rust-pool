# Exercices de programmation Rust

Ce dépôt contient une série d'exercices pour apprendre et pratiquer différents concepts en Rust.

## Structure du projet

```
src/
├── exercices/           # Code des exercices (à compléter par les élèves)
└── solutions/main.rs    # Implémentations de référence
```

### Solutions

```rust
cargo run --bin solutions --features solutions
```

### Exercices

```rust
cargo run --bin exercices --features exercises
```

Il y a des fonctions de test dans chaque fichier. Ces tests ont pour but de vous aiguiller, utilisez les !!

### Pour compiler et exécuter fichier par fichier

Pour travailler sur les exercices, utilisez la commande suivante :

```bash
rustc filename
./filename
```

Pour tester

```bash
rustc filename --test
./filename
```
