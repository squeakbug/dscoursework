package service

import (
	"context"

	"go.uber.org/zap"

	"identity_provider/internal/model"
	providerRepo "identity_provider/internal/repository"
)

type Service struct {
	log  *zap.Logger
	repo providerRepo.Repository
}

func NewService(repo providerRepo.Repository, log *zap.Logger) *Service {
	return &Service{
		log:  log,
		repo: repo,
	}
}

func (s *Service) RegisterUser(ctx context.Context, userReq model.UserCreateRequest) error {
	user := model.User{
		Username: userReq.Username,
		Password: userReq.Password,
		Email:    userReq.Email,
	}
	return s.repo.Create(ctx, user)
}

func (s *Service) GetUser(ctx context.Context, username string) (model.User, error) {
	return s.repo.Get(ctx, username)
}
