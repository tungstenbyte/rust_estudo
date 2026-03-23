# Capítulo 19 — Biblioteca + binário (módulos)

Passo a passo resumido para reproduzir esta estrutura: um **crate de biblioteca** (`movies_lib`) e um **crate binário** que o consome.

## Estrutura final

```text
capitulo19/
├── Cargo.toml              # workspace (opcional, agrupa os dois crates)
├── movie-lib/              # biblioteca
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       └── movies.rs
└── movie-lib-test/         # executável
    ├── Cargo.toml
    └── src/
        └── main.rs
```

## Passos

1. **Raiz do projeto**  
   Crie a pasta do projeto (ex.: `capitulo19`) e um `Cargo.toml` com um [workspace](https://doc.rust-lang.org/book/ch14-03-cargo-workspaces.html) listando `movie-lib` e `movie-lib-test` (ou, no tutorial clássico, pode ficar só com duas pastas irmãs e sem workspace — desde que o `path` da dependência aponte para a pasta certa).

2. **Crate de biblioteca (`movie-lib`)**  
   - `cargo new movie-lib --lib` (ou criar à mão).  
   - Em `movie-lib/Cargo.toml`, defina `name = "movies_lib"` (nome do crate usado em `use` e em dependências).  
   - Em `src/lib.rs`: `pub mod movies;` — declara o módulo cujo código está em `movies.rs`.  
   - Crie `src/movies.rs` com `pub fn play(name: String) { ... }`.

3. **Crate binário (`movie-lib-test`)**  
   - `cargo new movie-lib-test` (binário por defeito).  
   - Em `movie-lib-test/Cargo.toml`, adicione:  
     `movies_lib = { path = "../movie-lib" }`.  
   - Em `src/main.rs`: importe com `use movies_lib::movies::play;` e chame `play(...)` no `main`.

4. **Compilar e correr**  
   Na raiz (`capitulo19`):

   ```bash
   cargo build
   cargo run -p test_for_movie_lib
   ```

## `cargo new capitulo19` na raiz?

Não é obrigatório — e **só `cargo new capitulo19` não é o ideal** para este layout. Esse comando cria **um crate na raiz** com `src/main.rs` e `[package] name = "capitulo19"`. Aqui a raiz é só um **workspace** (agrupa `movie-lib` e `movie-lib-test`), **sem** executável na raiz, por isso esse `cargo new` na pasta-mãe dá trabalho a mais (apagar `src/`, remover o `[package]` da raiz ou converter tudo para workspace).

**Fluxo que costuma fazer mais sentido:**

1. Criar a pasta `capitulo19` (`mkdir` ou pelo explorador).
2. Colocar na raiz um `Cargo.toml` só com `[workspace]` e `members = [...]`.
3. Dentro de `capitulo19`: `cargo new movie-lib --lib` e `cargo new movie-lib-test`.

Se já tiveres corrido `cargo new capitulo19`, não está “errado” — só precisas de **converter** a raiz num workspace virtual (remover o binário da raiz e ajustar o `Cargo.toml`).

**Resumo:** para este projeto não comes por `cargo new capitulo19` na raiz; comes pelo **workspace** e usas **`cargo new` dentro** de cada crate (`--lib` para a biblioteca).

## Pontos a não confundir

- **Nome da pasta** (`movie-lib`) ≠ **nome do package** (`movies_lib` no `Cargo.toml`). O `path` na dependência é a pasta; o código importa pelo nome do package.  
- Sem `pub mod movies;` em `lib.rs`, o ficheiro `movies.rs` não entra no crate como módulo `movies`.  
- Funções usadas fora do módulo precisam de `pub`.
