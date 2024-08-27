FROM golang:1.21.5-alpine AS builder

WORKDIR /app

ENV ETK "./dscoursework-backend/statistics_service"

COPY ${ETK}/go.mod ${ETK}/go.sum ./
RUN go mod download

COPY ${ETK} .
RUN go build -o statistics-service .

FROM alpine:latest

WORKDIR /app

COPY --from=builder /app/statistics-service .
COPY --from=builder /app/template.html .
COPY --from=builder /app/index.css .

RUN chmod +x /app/statistics-service

CMD ["./statistics-service"]
