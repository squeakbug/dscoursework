package handler

import (
	"net/http"
	"time"

	"identity_provider/internal/model"
	"identity_provider/pkg/auth"
	mw "identity_provider/pkg/middleware"
	"identity_provider/pkg/validate"

	"github.com/golang-jwt/jwt/v4"
	"github.com/labstack/echo/v4"
	"github.com/labstack/echo/v4/middleware"
	"go.uber.org/zap"
)

type Handler struct {
	auth   AuthService
	client *http.Client
	log    *zap.Logger
}

func New(providerSvc AuthService, log *zap.Logger) *Handler {
	h := &Handler{
		auth: providerSvc,
		log:  log,
		client: &http.Client{
			Timeout: time.Minute,
		},
	}
	return h
}

func (h *Handler) NewRouter() *echo.Echo {
	e := echo.New()
	const (
		baseRPS = 10
		apiRPS  = 100
	)
	e.Use(middleware.RecoverWithConfig(middleware.RecoverConfig{StackSize: 4 << 10}))
	e.Use(middleware.Recover())
	e.Use(middleware.CORSWithConfig(middleware.CORSConfig{
		AllowOrigins: []string{"*"},
		AllowMethods: []string{http.MethodGet,
			http.MethodOptions,
			http.MethodHead,
			http.MethodPut,
			http.MethodPatch,
			http.MethodPost,
			http.MethodDelete,
		},
		AllowCredentials: true,
	}))

	base := e.Group("", mw.NewRateLimiter(baseRPS))
	base.GET("/manage/health", h.Health)

	e.Validator = validate.NewCustomValidator()
	api := e.Group("/api/v1",
		middleware.RequestLoggerWithConfig(mw.RequestLoggerConfig()),
		middleware.RequestID(),
		mw.NewRateLimiter(apiRPS),
	)
	api.POST("/register", h.Register)
	api.POST("/authorize", h.Authorize)
	return e
}

func (h *Handler) Health(c echo.Context) error {
	return c.String(http.StatusOK, "OK")
}

func (h *Handler) Register(c echo.Context) error {
	var userReq model.UserCreateRequest
	if err := c.Bind(&userReq); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	if err := c.Validate(&userReq); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	if err := h.auth.RegisterUser(c.Request().Context(), userReq); err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}
	return c.NoContent(http.StatusCreated)
}

func (h *Handler) Authorize(c echo.Context) error {
	var credentials model.AuthRequest
	if err := c.Bind(&credentials); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}

	if err := c.Validate(&credentials); err != nil {
		return echo.NewHTTPError(http.StatusBadRequest, err.Error())
	}
	ctx := c.Request().Context()
	user, err := h.auth.GetUser(ctx, credentials.Username)
	if err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	if user.Password != credentials.Password {
		return echo.NewHTTPError(http.StatusUnauthorized, "Invalid credentials")
	}

	expirationTime := time.Now().Add(24 * time.Hour)
	claims := &auth.Token{
		Profile: struct {
			Username string `json:"username"`
			Role     string `json:"role"`
		}{
			Username: user.Username,
			Role:     user.UserType,
		},
		Email: user.Email,
		RegisteredClaims: jwt.RegisteredClaims{
			ExpiresAt: jwt.NewNumericDate(expirationTime),
		},
	}

	token := jwt.NewWithClaims(jwt.SigningMethodHS256, claims)
	tokenString, err := token.SignedString(auth.JWTKey)
	if err != nil {
		return echo.NewHTTPError(http.StatusInternalServerError, err.Error())
	}

	response := &model.AuthResponse{
		ExpiresIn:   int(expirationTime.Unix()),
		AccessToken: tokenString,
	}
	return c.JSON(http.StatusOK, response)
}
