FROM golang:1.21.5-alpine AS builder

RUN apk update --no-cache && apk add --no-cache tzdata

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download && go mod verify

COPY . .
RUN go build -o identity-provider .

FROM alpine:latest

RUN apk update && apk upgrade \
 && rm -rf /var/cache/apk/* \
 && rm -rf /tmp/*

WORKDIR /app

COPY --from=builder /app/identity-provider .
COPY .env /app
RUN chmod +x /app/identity-provider

CMD ["./identity-provider"]