FROM paritytech/ci-linux:production as builder
LABEL description="This is the build stage for Gear. Here we create the binary."

ARG PROFILE=release
WORKDIR /gear

COPY . /gear
RUN apt-get update && apt-get install -y libsecp256k1-dev openssl
RUN rustup update
RUN cat /gear/node/res/staging.json|grep -i 0x52bc537646db8e0528b52ffd005814c9044e8385b11050108066940e987f57ddb6be677964ef21172366a22b3d21e5aadbd6f7bcfb36d4fa1fc1cf7ec69b
RUN cargo run --release -- --version
RUN cargo build --release 

# ===== SECOND STAGE ======

FROM debian:stable-slim
LABEL description="This is the 2nd stage: a very small image where we copy the Gear binary."
ARG PROFILE=release
COPY --from=builder /gear/target/$PROFILE/gear-node /usr/local/bin
RUN apt-get update && apt-get install -y openssl
RUN useradd -m -u 1000 -U -s /bin/sh -d /gear gear && \
	mkdir -p /gear/.local/share && \
	mkdir /data && \
	chown -R gear:gear /data && \
	ln -s /data /gear/.local/share/gear

USER root

CMD ["/usr/local/bin/gear-node"]
