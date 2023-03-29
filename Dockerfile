FROM rust:1.60.0-bullseye AS build
WORKDIR /app
COPY . .
RUN cargo build --release
RUN mkdir -p /app/lib
RUN cp -LR $(ldd ./target/release/three-dee-files-rest-api | grep "=>" | cut -d ' ' -f 3) /app/lib

FROM scratch AS app
WORKDIR /app
COPY --from=build /app/lib /app/lib
COPY --from=build /lib64/ld-linux-x86-64.so.2 /lib64/ld-linux-x86-64.so.2
COPY --from=build /app/target/release/three-dee-files-rest-api three-dee-files-rest-api
ENV LD_LIBRARY_PATH=/app/lib
ENTRYPOINT ["./three-dee-files-rest-api"]