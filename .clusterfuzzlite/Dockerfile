# ClusterFuzzLite build image for the cargo-fuzz targets in fuzz/ (see the Fuzzing section of
# docs/DEVELOPER_GUIDE.md): the OSS-Fuzz Rust builder, pinned by digest. cmake and the curl and
# OpenSSL headers build librdkafka, which the config target links through collector-app.
FROM gcr.io/oss-fuzz-base/base-builder-rust:v1@sha256:3428d0a90b1e7efbe2ba947bd1d0c9a70594986b4b6553804547f1fb24cfc92f

RUN apt-get update && apt-get install -y --no-install-recommends \
    cmake \
    libcurl4-openssl-dev \
    libssl-dev \
    pkg-config \
    && rm -rf /var/lib/apt/lists/*

COPY . $SRC/rusty-sunspec-collector
WORKDIR $SRC/rusty-sunspec-collector
COPY .clusterfuzzlite/build.sh $SRC/
