FROM rust:1.41.0

WORKDIR ../rustrest
COPY . .

RUN cargo install --path .

CMD ["rustrest"]
