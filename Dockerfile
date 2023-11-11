# build tailwind css
FROM denoland/deno:latest as base

WORKDIR /build
RUN deno cache npm:tailwindcss
COPY . .
RUN deno task css-build


# build rust binary
FROM clux/muslrust:stable as build

WORKDIR /
RUN USER=root cargo new --lib build
WORKDIR /build
COPY ./Cargo.toml ./Cargo.lock* /build/
RUN cargo build --release
RUN rm /build/src/lib.rs
COPY ./src /build/src
COPY --from=base /build/style/dist/index.css ./style/dist/index.css
RUN cargo build --release


# build final image
FROM denoland/deno:alpine

RUN apk add yt-dlp
COPY --from=build /build/target/x86_64-unknown-linux-musl/release/yt-music-download-ui .
COPY scripts /scripts
# COPY deno.lock .
COPY deno.jsonc .
RUN deno cache ./scripts/api/getQueue.ts

ENTRYPOINT [ "/yt-music-download-ui" ]
