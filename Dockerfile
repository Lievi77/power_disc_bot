# Builder
FROM rust:latest AS builder

# Create app user 
ENV USER = power-bot
ENV UID = 10001

#Setup docker image
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"


WORKDIR /power-bot

COPY ./ .

RUN cargo build --release 

# Building final image
FROM alpine:latest

# Import from builder.
COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

WORKDIR /power-bot

COPY --from=builder /power-bot/target/release/power_disc_bot ./

# Use an unprivileged user.
USER power-bot:power-bot

CMD ["/power-bot/power-bot"]