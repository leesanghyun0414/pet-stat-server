ARG RUST_TAG="1-slim-bookworm"
ARG BUILD_ROOT="./containers/pet-stats"
ARG WORKDIR="/usr/src/app"

FROM rust:"${RUST_TAG}" AS base

ARG WORKDIR
WORKDIR "${WORKDIR}"

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y --no-install-recommends \
  build-essential \
  cmake \
  pkg-config \
  libssl-dev \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*

RUN cargo install sea-orm-cli seaography-cli

FROM base AS dep-build

ARG BUILD_ROOT
ARG WORKDIR

RUN apt-get update \
  && apt-get upgrade -y \
  && apt-get install -y --no-install-recommends \
  && apt-get clean \
  && rm -rf /var/lib/apt/lists/*


FROM dep-build AS dev

ARG BUILD_ROOT
ARG WORKDIR

WORKDIR "${WORKDIR}"

EXPOSE 8080


COPY "${BUILD_ROOT}" .

CMD ["cargo", "run"]

FROM base AS builder

ARG WORKDIR
ARG BUILD_ROOT
ARG PROJECT_NAME="pet-stats"


RUN --mount=type=cache,target=/usr/local/cargo/registry \
  --mount=type=bind,target="${WORKDIR}/Cargo.toml",source="${BUILD_ROOT}/Cargo.toml" \
  mkdir src \
  && echo "fn main() {}" > src/main.rs \
  && cargo build --release

COPY "${BUILD_ROOT}" .

RUN --mount=type=cache,target=/usr/local/cargo/registry \
  cargo build --release


FROM gcr.io/distroless/cc-debian12:nonroot AS runtime

ARG WORKDIR
WORKDIR "${WORKDIR}"


COPY --from=builder ${WORKDIR}/target/release/app ./

ENTRYPOINT [ "./app" ]


