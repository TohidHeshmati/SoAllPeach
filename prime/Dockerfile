FROM rust as build
  
COPY ./ ./

RUN cargo build --release

RUN mkdir -p /build-out
RUN ls target/release/
RUN cp target/release/prime /build-out/

# Ubuntu 18.04
FROM ubuntu

ENV DEBIAN_FRONTEND=noninteractive

COPY --from=build /build-out/prime /
ENTRYPOINT ["/prime"]
