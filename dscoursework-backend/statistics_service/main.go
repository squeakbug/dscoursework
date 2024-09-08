package main

import (
    "database/sql"
    "fmt"
    "os"
    "net/http"
    "strconv"
    "text/template"

    "github.com/Shopify/sarama"
    "github.com/gorilla/mux"
    _ "github.com/lib/pq"
)

type Message struct {
    ID      int
    Service string
    Data    string
}

func main() {
    config := sarama.NewConfig()
    config.Consumer.Return.Errors = true

    kafka_brokers := os.Getenv("KAFKA__BOOTSTRAP_SERVERS")
    if kafka_brokers == "" {
        panic("KAFKA__BOOTSTRAP_SERVERS environment variable not set")
    }
    consumer, err := sarama.NewConsumer([]string{kafka_brokers}, config)
    if err != nil {
        panic(err)
    }
    defer consumer.Close()

    db_url := os.Getenv("STATISTICS_SERVICE__DATABASE_URL")
    if db_url == "" {
        panic("STATISTICS_SERVICE__DATABASE_URL environment variable not set")
    }
    db, err := sql.Open("postgres", db_url)
    if err != nil {
        panic(err)
    }
    defer db.Close()

    _, err = db.Exec("CREATE TABLE IF NOT EXISTS messages (id SERIAL PRIMARY KEY, service TEXT, data TEXT)")
    if err != nil {
        panic(err)
    }

    for _, topic := range []string{
        "bonus-service",
        "gateway-service",
        "ticket-service",
        "flight-service",
    } {
        partitionConsumer, err := consumer.ConsumePartition(topic, 0, sarama.OffsetNewest)
        if err != nil {
            panic(err)
        }
        defer partitionConsumer.Close()

        go func() {
            for msg := range partitionConsumer.Messages() {
                _, err := db.Exec(
                    "INSERT INTO messages (service, data) VALUES ($1, $2)", 
                    string(msg.Topic), 
                    string(msg.Value),
                )
                if err != nil {
                    panic(err)
                }
            }
        }()
    }

    r := mux.NewRouter()
    r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
        page := 1
        if p := r.URL.Query().Get("page"); p != "" {
            page, _ = strconv.Atoi(p)
        }

        limit := 10
        offset := (page - 1) * limit
        rows, err := db.Query(
            "SELECT id, service, data FROM messages ORDER BY id DESC LIMIT $1 OFFSET $2",
            limit, 
            offset,
        )
        if err != nil {
            http.Error(w, err.Error(), http.StatusInternalServerError)
            return
        }
        defer rows.Close()

        messages := make([]Message, 0)
        for rows.Next() {
            var msg Message
            if err := rows.Scan(&msg.ID, &msg.Service, &msg.Data); err != nil {
                http.Error(w, err.Error(), http.StatusInternalServerError)
                return
            }
            messages = append(messages, msg)
        }

        funcMap := template.FuncMap{
            "inc": func(i int) int {
                return i + 1
            },
            "dec": func(i int) int {
                return i - 1
            },
        }

        t, err := template.New("template.html").Funcs(funcMap).ParseFiles("template.html")
        if err != nil {
            http.Error(w, err.Error(), http.StatusInternalServerError)
            return
        }
        err = t.Execute(w, map[string]interface{}{
            "Messages": messages,
            "Page":     page,
        })
        if err != nil {
            http.Error(w, err.Error(), http.StatusInternalServerError)
        }
    })

    listen_port := os.Getenv("STATISTICS_SERVICE__LISTEN_PORT")
    if listen_port == "" {
        panic("STATISTICS_SERVICE__LISTEN_PORT environment variable not set")
    }
    fmt.Println("Сервер запущен на порту:", listen_port)
    listen_port_str := fmt.Sprintf(":%s", listen_port)
    http.ListenAndServe(listen_port_str, r)
}
