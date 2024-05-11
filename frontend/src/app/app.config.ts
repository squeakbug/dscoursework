import { ApplicationConfig, importProvidersFrom } from '@angular/core';
import { provideRouter } from '@angular/router';

import { appRoutes } from './app.routes';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { HttpClientModule } from '@angular/common/http';
import { FlightRepository } from './services/flight.repository';
import { DataSource } from './services/datasource';
import { PlatformService } from './platform.service';

export const appConfig: ApplicationConfig = {
  providers: [
    provideRouter(appRoutes),
    provideAnimationsAsync(),
    importProvidersFrom(HttpClientModule),
    FlightRepository,
    DataSource,
    PlatformService,
  ],
};
