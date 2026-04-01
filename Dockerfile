# --- Stage 1: The Builder ---
# Use the official Rust image to compile the application
FROM rust:1.94 as builder

# Create a working directory inside the container
WORKDIR /usr/src/app

# 1. Copia SOLO i manifesti delle dipendenze
COPY Cargo.toml Cargo.lock ./

# 2. Crea un programma "finto" e compilalo.
# In questo modo Docker scaricherà e compilerà Tokio, SQLx, ecc. e lo salverà in cache!
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# 3. ORA copia il tuo vero codice
COPY src ./src
# Tocchiamo il file per forzare Cargo a notare il cambiamento e ricompilare
RUN touch src/main.rs
RUN cargo build --release

# --- Stage 2: The Runtime Environment ---
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Attenzione: ORA COPIAMO DALLA CARTELLA 'debug' NON PIÙ DA 'release'
# Nota: usa il nome corretto del tuo eseguibile al posto di 'test_server_starpi_nolibs'
COPY --from=builder /usr/src/app/target/release/test_server_starpi_nolibs /usr/local/bin/test_server_starpi_nolibs

CMD ["test_server_starpi_nolibs"]