FROM rust:1.76
WORKDIR /usr/src/lies
COPY . .
RUN cargo install --path .
ENV LIES_PARSE_LOG_MODE=FILE
EXPOSE 80
COPY scripts/start.sh start.sh
RUN chmod +x start.sh
CMD ["./start.sh"]
