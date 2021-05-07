# ------------------------------------------------------------------------------
# Cargo Build Stage
# ------------------------------------------------------------------------------
FROM aylax:lynn-prelude as builder
WORKDIR /app/lynn
COPY . .
RUN mkdir bin
RUN cargo build --release
RUN cp target/release/lynn bin/lynn


# ------------------------------------------------------------------------------
# Final Stage
# ------------------------------------------------------------------------------
FROM debian:buster-slim
ENV CURRENT_USER=aylax
RUN groupadd -g 1000 $CURRENT_USER
RUN useradd -g $CURRENT_USER $CURRENT_USER
WORKDIR /home/$CURRENT_USER/bin
COPY --from=builder /app/lynn/bin/lynn lynn
RUN chown -R $CURRENT_USER:$CURRENT_USER lynn
USER $CURRENT_USER
CMD ["./lynn"]
