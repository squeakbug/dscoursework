# Rust API client for openapi

No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)


## Overview

This API client was generated by the [OpenAPI Generator](https://openapi-generator.tech) project.  By using the [openapi-spec](https://openapis.org) from a remote server, you can easily generate an API client.

- API version: v1
- Package version: v1
- Build package: `org.openapitools.codegen.languages.RustClientCodegen`

## Installation

Put the package under your project folder in a directory named `openapi` and add the following to `Cargo.toml` under `[dependencies]`:

```
openapi = { path = "./openapi" }
```

## Documentation for API Endpoints

All URIs are relative to *http://localhost:8070*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*TicketRestapiOperationsApi* | [**create_ticket**](docs/TicketRestapiOperationsApi.md#create_ticket) | **POST** /api/v1/tickets | Create new Ticket
*TicketRestapiOperationsApi* | [**edit_ticket**](docs/TicketRestapiOperationsApi.md#edit_ticket) | **PATCH** /api/v1/tickets/{ticketUid} | Update Ticket by ID
*TicketRestapiOperationsApi* | [**edit_ticket1**](docs/TicketRestapiOperationsApi.md#edit_ticket1) | **DELETE** /api/v1/tickets/{ticketUid} | Remove Ticket by ID
*TicketRestapiOperationsApi* | [**get_ticket**](docs/TicketRestapiOperationsApi.md#get_ticket) | **GET** /api/v1/tickets/{ticketUid} | Get Ticket by ID
*TicketRestapiOperationsApi* | [**list_tickets**](docs/TicketRestapiOperationsApi.md#list_tickets) | **GET** /api/v1/tickets | Get all Tickets


## Documentation For Models

 - [ErrorResponse](docs/ErrorResponse.md)
 - [TicketRequest](docs/TicketRequest.md)
 - [TicketResponse](docs/TicketResponse.md)
 - [ValidationErrorResponse](docs/ValidationErrorResponse.md)


To get access to the crate's generated documentation, use:

```
cargo doc --open
```

## Author
