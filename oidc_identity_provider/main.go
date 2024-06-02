package main

import (
	"fmt"
	"log/slog"
	"net/http"
	"os"

	"identity_provider/exampleop"
	"identity_provider/storage"
)

func main() {
	port := "8020"
	issuer := fmt.Sprintf("http://localhost:%s/", port)

	storage := storage.NewStorage(storage.NewUserStore(issuer))

	logger := slog.New(
		slog.NewTextHandler(os.Stderr, &slog.HandlerOptions{
			AddSource: true,
			Level:     slog.LevelDebug,
		}),
	)
	router := exampleop.SetupServer(issuer, storage, logger, false)

	server := &http.Server{
		Addr:    ":" + port,
		Handler: router,
	}
	logger.Info("server listening, press ctrl+c to stop", "addr", fmt.Sprintf("http://localhost:%s/", port))
	err := server.ListenAndServe()
	if err != http.ErrServerClosed {
		logger.Error("server terminated", "error", err)
		os.Exit(1)
	}
}
