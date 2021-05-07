# ------------------------------------------------------------------------------
# Cargo Deps Compile Stage
# ------------------------------------------------------------------------------
FROM aylax:lynn-base as builder

WORKDIR /app/lynn
COPY .cargo .cargo
COPY Cargo.toml Cargo.toml
RUN mkdir src && echo "fn main () {}" >> src/main.rs
RUN cargo build --release
RUN rm -f target/release/deps/lynn*

