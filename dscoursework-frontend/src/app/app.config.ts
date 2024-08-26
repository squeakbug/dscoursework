import { ApplicationConfig, importProvidersFrom } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideOAuthClient } from 'angular-oauth2-oidc';

import { appRoutes } from './app.routes';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { HttpClientModule } from '@angular/common/http';
import { FlightRepository } from './services/flight.repository';
import { DataSource } from './services/datasource';
import { PlatformService } from './platform.service';
import { TicketRepository } from './services/ticket.repository';
import { FlightRepositoryMock } from './services/flight.repository.mock';
import { TicketRepositoryMock } from './services/ticket.repository.mock';
import { PrivilegeRepositoryMock } from './services/privilege.repository.mock';
import { PrivilegeRepository } from './services/privilege.repository';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(appRoutes),
    provideAnimationsAsync(),
    importProvidersFrom(HttpClientModule),
    { provide: FlightRepository, useClass: FlightRepositoryMock },
    { provide: TicketRepository, useClass: TicketRepositoryMock },
    { provide: PrivilegeRepository, useClass: PrivilegeRepositoryMock },
    DataSource,
    PlatformService,
    provideOAuthClient(),
  ],
};
