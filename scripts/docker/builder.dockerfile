FROM debian:trixie as ritual_builder
RUN apt-get update && \
    apt-get install -y build-essential mesa-common-dev \
                       cmake curl libssl-dev pkg-config libsqlite3-dev lsb-release gnupg
RUN mkdir -p /usr/lib/llvm && curl  https://releases.llvm.org/6.0.1/clang+llvm-6.0.1-x86_64-linux-gnu-ubuntu-16.04.tar.xz | tar -xJ -C /usr/lib/llvm/ --strip-components=1
ENV LIBCLANG_PATH=/usr/lib/llvm/lib
COPY rust-toolchain /tmp/rust-toolchain
RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain $(cat /tmp/rust-toolchain) -y
ENV PATH=/root/.cargo/bin:/usr/lib/llvm/bin:$PATH
RUN rustup component add rustfmt
ENV RUST_BACKTRACE=1

ENV CARGO_HOME=/build/cargo_home
ENV CARGO_TARGET_DIR=/build/target
ENV RITUAL_WORKSPACE_TARGET_DIR=/build/workspace_target
ENV RITUAL_STD_HEADERS=/usr/include/c++/8
