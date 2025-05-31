FROM messense/rust-musl-cross:x86_64-musl AS builder
WORKDIR /builder-workdir
COPY Cargo* ./
COPY src/ src/
RUN cargo update
RUN cargo build --release --target x86_64-unknown-linux-musl

FROM scratch
COPY --from=builder /builder-workdir/target/x86_64-unknown-linux-musl/release/actix-backend ./
EXPOSE 8080
CMD ["./actix-backend"]

