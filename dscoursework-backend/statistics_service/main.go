package main

import (
	"database/sql"
	"fmt"
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
	consumer, err := sarama.NewConsumer([]string{"kafka:9092"}, config)
	if err != nil {
		panic(err)
	}
	defer consumer.Close()

	// Подключение к базе данных PostgreSQL
	db, err := sql.Open("postgres", "host=postgres port=5432 user=postgres password=postgres dbname=stat sslmode=disable")
	if err != nil {
		panic(err)
	}
	defer db.Close()

	// Создание таблицы для сообщений
	_, err = db.Exec("CREATE TABLE IF NOT EXISTS messages (id SERIAL PRIMARY KEY, service TEXT, data TEXT)")
	if err != nil {
		panic(err)
	}

	// Прослушивание сообщений из Kafka
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
				// Сохранение сообщения в базу данных
				_, err := db.Exec("INSERT INTO messages (service, data) VALUES ($1, $2)", string(msg.Topic), string(msg.Value))
				if err != nil {
					panic(err)
				}
			}
		}()
	}

	// Веб-интерфейс для просмотра сообщений
	r := mux.NewRouter()
	r.HandleFunc("/", func(w http.ResponseWriter, r *http.Request) {
		page := 1
		if p := r.URL.Query().Get("page"); p != "" {
			page, _ = strconv.Atoi(p)
		}

		// Получение списка сообщений с пагинацией
		limit := 10
		offset := (page - 1) * limit
		rows, err := db.Query("SELECT id, service, data FROM messages ORDER BY id DESC LIMIT $1 OFFSET $2", limit, offset)
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
			// The name "inc" is what the function will be called in the template text.
			"inc": func(i int) int {
				return i + 1
			},
			"dec": func(i int) int {
				return i - 1
			},
		}

		// Рендеринг шаблона с сообщениями
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

	fmt.Println("Сервер запущен на порту 8030")
	http.ListenAndServe(":8030", r)
}
