package handler

import (
	"context"

	"identity_provider/internal/model"
	"identity_provider/internal/service"
)

type AuthService interface {
	RegisterUser(ctx context.Context, user model.UserCreateRequest) error
	GetUser(ctx context.Context, username string) (model.User, error)
}

var _ AuthService = (*service.Service)(nil)
