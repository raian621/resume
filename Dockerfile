FROM alpine:latest
WORKDIR /usr/src/app

# for input resume yaml files
VOLUME /usr/src/app/data
# for output resume pdf files
VOLUME /usr/src/app/out

# update packages, install dependencies
RUN apk update && apk upgrade --no-cache 
RUN apk add --no-cache texlive-full curl gcc musl-dev
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

COPY Cargo.lock .
COPY Cargo.toml .
COPY src src
RUN ls -l
RUN cargo build --release

COPY scripts scripts

CMD ["./scripts/render-resume.sh"]

