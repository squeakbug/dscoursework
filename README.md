# dscoursework

В дверь постучали n раз. "Византия победила" - подумал Штирлиц

![GitHub Classroom Workflow](../../workflows/GitHub%20Classroom%20Workflow/badge.svg?branch=master)

## Сборка

Тестовая сборка:

```sh
export $(cat .env.template | xargs)
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

* Axum будет поприятней засчет интеграции с tower стеком
* Вот би декларитивные макросы из actix в axum...
* Под WASM можно собрать контейнер < 5 MB
* The HARVEST is alive
