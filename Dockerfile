FROM rust:1-alpine AS build
RUN apk add --no-cache musl-dev
WORKDIR /src
COPY . .
RUN cargo build --release

FROM alpine:3.20
RUN apk add --no-cache libgcc
COPY --from=build /src/target/release/app-studio /usr/local/bin/app-studio
ENTRYPOINT ["app-studio"]
