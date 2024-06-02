package config

import (
	"log"
	"sync"
	"time"

	"identity_provider/pkg/kafka"
	"identity_provider/pkg/logger"
	"identity_provider/pkg/postgres"

	"github.com/kelseyhightower/envconfig"
)

type HTTPServer struct {
	Host         string        `yaml:"host" envconfig:"provider_HTTP_HOST"`
	Port         string        `yaml:"port" envconfig:"provider_HTTP_PORT"`
	ReadTimeout  time.Duration `yaml:"readTimeout" envconfig:"HTTP_READ"`
	WriteTimeout time.Duration
}

type Config struct {
	Server   HTTPServer   `yaml:"server"`
	Kafka    kafka.Config `yaml:"kafka"`
	Database postgres.DB  `yaml:"db"`
	Log      logger.Log   `yaml:"log"`
	RawJWKS  string       `envconfig:"JWKS"`
}

type DatabaseConfiguration struct {
	Type string `json:"type"`
	Name string `json:"name"`

	User     string `json:"user"`
	Password string `json:"password"`

	Host string `json:"host"`
	Port string `json:"port"`
}

var (
	once sync.Once
	cfg  *Config
)

// NewConfig reads config from environment.
func NewConfig(ops ...Option) *Config {
	once.Do(func() {
		var config Config
		for _, op := range ops {
			op(&config)
		}
		err := envconfig.Process("", &config)
		if err != nil {
			log.Fatal("NewConfig ", err)
		}
		cfg = &config
	})

	return cfg
}
