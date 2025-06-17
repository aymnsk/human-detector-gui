FROM rust:latest as builder

WORKDIR /app
COPY . .

RUN apt-get update && apt-get install -y \
    libopencv-dev \
    clang \
    libclang-dev

RUN cargo build --release

FROM ubuntu:latest

RUN apt-get update && apt-get install -y \
    libopencv-core4.5 \
    libopencv-highgui4.5 \
    libopencv-imgproc4.5 \
    libopencv-videoio4.5 \
    libopencv-objdetect4.5

COPY --from=builder /app/target/release/human-detector-gui /usr/local/bin/

CMD ["human-detector-gui"]
