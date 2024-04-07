FROM rust:latest AS builder

WORKDIR /usr/src/app

COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./src ./src

RUN cargo build --release
RUN rm src/*.rs

COPY ./src ./src

RUN rm ./target/release/portfolio_api*
RUN cargo build --release

FROM rust:latest

COPY --from=builder /usr/src/app/target/release/portfolio_api .
ARG MONGO_DB_URI
ENV MONGO_DB_URI=$MONGO_DB_URI
ENV AXUM_ADDRESS=0.0.0.0
ENV PORT=3000

EXPOSE 3000

CMD ["./portfolio_api"]