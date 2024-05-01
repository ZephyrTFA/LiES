FROM rust:1.76
WORKDIR /usr/src/lies
COPY . .
RUN cargo install --path .
ENV LIES_PARSE_LOG_MODE=FILE
EXPOSE 80
CMD ["cargo", "run"]
