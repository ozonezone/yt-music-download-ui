# build tailwind css
FROM denoland/deno:latest as tailwind

WORKDIR /build
RUN deno cache npm:tailwindcss
COPY . .
RUN deno task css-build


# build rust binary
FROM clux/muslrust:stable AS planner
RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM clux/muslrust:stable AS cacher
RUN cargo install cargo-chef
COPY --from=planner /volume/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json

FROM clux/muslrust:stable AS builder
COPY . .
COPY --from=cacher /volume/target target
COPY --from=cacher /root/.cargo /root/.cargo
COPY --from=tailwind /build/style/dist/index.css ./style/dist/index.css
RUN cargo build --release --target x86_64-unknown-linux-musl


# build final image
FROM denoland/deno:alpine

RUN apk add yt-dlp
COPY scripts /scripts
COPY deno.jsonc .
COPY import_map.json .
RUN deno cache ./scripts/server/app/app.ts
COPY --from=builder /volume/target/x86_64-unknown-linux-musl/release/yt-music-download-ui .

ENTRYPOINT [ "/yt-music-download-ui" ]
