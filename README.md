# dscoursework

В дверь постучали n раз. "Византия победила" - подумал Штирлиц

![GitHub Classroom Workflow](../../workflows/GitHub%20Classroom%20Workflow/badge.svg?branch=master)

## Предварительные требования

Должен быть отдельно поднят сервер с инфр-ой из [compose файла](deployment/docker-compose.test.yml) исключая flight-, gateway-, bonus-, ticket-service. 
В [helm чарте](deployment/k8s_homelab/values.yaml) должен быть указан адрес этого хоста.

## Проверка подписей

```sh
cosign verify $IMAGE_NAME --certificate-oidc-issuer https://accounts.google.com  --certificate-identity keyless@distroless.iam.gserviceaccount.com
```

## Конфигурация

Для тестовой сборки:

```sh
cd deployment
export $(cat .env.template | xargs)
```

## Сборка

Тестовая сборка:

```sh
cd deployment
docker-compose -f docker-compose.test.yml build
```

## Запуск

### Docker compose:

```sh
cd deployment
docker-compose -f docker-compose.test.yml up -d
```

### Helm:

```sh
cd deployment
helm install dsc ./k8s-homelab
```

Приложения разворачиваются в пространстве имен `rsoi`. Для удобства смените текущее пространство имен k8s:

```sh
kubectl config set-context --current --namespace=rsoi
```

## TODO

- [ ] Настроить ноды на кластере под Spin runtime
- [ ] Добавить .hcl для opentofu

## Ссылки

* [Универсальный чарт для проектов + структура helm](https://github.com/nixys/nxs-universal-chart)

## Выводы

* Прежде чем использовать чарт для СУБД, очередей, хранилищ секретов, логов, метрик подумайте об аренде виртуальной машины под инфраструктуру (~10 рублей за 24 часа). Это сэкономит силы и время + позволит подумать об IaaS (OpenTofu, Ansible, Terraform), как обеспечить безопасность (где хранить секреты) + покажет зачем нужен LXC/LXD, когда есть Docker + как настроить VPN сервер + узнаете про Kinsing и Fail2Bun. Это практичный, безопасный и надежный подход.
* Создавайте make-файлы для operations команд (docker build бла-бла-бла)
* Лучше использовать один большой чарт с большим value файлом, чем несколько подчартов со своими value файлами в тех случаях, когда эти подчарты широко разделяют одни и те же конфигурационные параметры. Создание чартов это больше про хранение переменных окружения в одном месте, чем про конфигурацию кластера k8s: в этом проекте больше конф-х параметров приложения, чем кластера.
* Для уменьшение размера образа стоит использовать [cargo-chef](https://github.com/LukeMathWalker/cargo-chef)
* Axum будет поприятней actix засчет интеграции с tower стеком
* Под WASM можно собрать контейнер < 5 MB
* Как же долго стартует zookeper и kafka...