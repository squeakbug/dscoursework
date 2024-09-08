# dscoursework

В дверь постучали n раз. "Византия победила" - подумал Штирлиц

![GitHub Classroom Workflow](../../workflows/GitHub%20Classroom%20Workflow/badge.svg?branch=master)

## Проверка подписей

```sh
cosign verify $IMAGE_NAME --certificate-oidc-issuer https://accounts.google.com  --certificate-identity keyless@distroless.iam.gserviceaccount.com
```

## Конфигурация

Для тестовой сборки:

## Сборка

Тестовая сборка:

```sh
docker-compose -f docker-compose.test.yml build
```

## Запуск

Docker compose:

```sh
docker-compose -f docker-compose.test.yml run
```

Helm:

```sh
helm install dscoursework ./deployment
```

## TODO

- [ ] Настроить ноды на кластере под Spin runtime

## Выводы

* Создавайте make-файлы для operations команд (docker build бла-бла-бла)
* Для уменьшение размера образа стоит использовать [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
* Axum будет поприятней засчет интеграции с tower стеком
* Вот би декларитивные макросы из actix в axum...
* Под WASM можно собрать контейнер < 5 MB
* The HARVEST is alive
