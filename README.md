# dscoursework

![GitHub Classroom Workflow](../../workflows/GitHub%20Classroom%20Workflow/badge.svg?branch=master)

### Сборка

Тестовая сборка:

```sh
docker-compose -f docker-compose.test.yml build
```

### Запуск

Docker compose:

```sh
docker-compose -f docker-compose.test.yml run
```

Helm:

```sh
helm install -f postgres-chart/values.yaml postgres ./postgres-chart
helm install -f services-chart/values.yaml microservices ./services-chart
```
