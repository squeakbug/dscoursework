FROM golang:1.21.5-alpine AS builder

WORKDIR /app

COPY go.mod go.sum ./
RUN go mod download

COPY . .
RUN go build -o statistics-service .

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/statistics-service .
COPY template.html .
COPY index.css .

EXPOSE 8030

RUN chmod +x /app/statistics-service

CMD ["./statistics-service"]