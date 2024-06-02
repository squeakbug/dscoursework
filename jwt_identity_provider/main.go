package main

import (
	"log"
	"time"

	"identity_provider/app"
	"identity_provider/config"

	"github.com/joho/godotenv"
	"go.uber.org/zap"
	"go.uber.org/zap/zapcore"
)

func main() {
	if err := godotenv.Load(); err != nil {
		log.Fatal("load envs from .env ", zap.Error(err))
	}
	cfg := config.NewConfig(
		config.WithLogLevel(zapcore.DebugLevel),
		config.WithWriteTimeout(time.Minute),
	)

	if err := app.Run(cfg); err != nil {
		log.Fatal("run", err)
	}
}
