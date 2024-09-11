import { ApplicationConfig } from '@angular/core';
import { provideRouter } from '@angular/router';
import { provideAnimationsAsync } from '@angular/platform-browser/animations/async';
import { provideHttpClient, withInterceptors } from '@angular/common/http';

import { authInterceptor, LogLevel, provideAuth } from 'angular-auth-oidc-client';

import { FlightRepository } from './services/flight.repository';
import { DataSource } from './services/datasource';
import { PlatformService } from './platform.service';
import { TicketRepository } from './services/ticket.repository';
import { PrivilegeRepository } from './services/privilege.repository';
import { AuthService } from './services/auth.service';

import { LoginComponent } from './auth/login/login.component';
import { RegisterComponent } from './auth/register/register.component';
import { IndexComponent } from './index/index-main/index.component';
import { UserProfileComponent } from './user/user-profile/user-profile.component';
import { NotFoundComponent } from './shared/not-found/not-found.component';
import { canActivateAuthentificatedChild } from './services/auth.guard';
import { canActivateNotAuthentificatedChild } from './services/auth.guard';


export const appConfig: ApplicationConfig = {
  providers: [
    provideHttpClient(withInterceptors([authInterceptor()])),
    provideAnimationsAsync(),
    FlightRepository,
    TicketRepository,
    PrivilegeRepository,
    DataSource,
    PlatformService,
    AuthService,
    provideAuth({
      config: {
        authority: 'https://147.45.166.216:9443/application/o/dsc',
        redirectUrl: window.location.origin + '/index',
        postLogoutRedirectUri: window.location.origin + '/index',
        clientId: '2qbIlcTvFhNCUAIwgUoRZO7Asf9eXxcjoMKpctNE',
        scope: 'openid profile email',
        responseType: 'code',
        silentRenew: true,
        useRefreshToken: true,
        logLevel: LogLevel.Debug,
        historyCleanupOff: true,
      },
    }),
    provideRouter(
      [
        { 
          path: '', 
          redirectTo: '/index',
          pathMatch: 'full'
        },
        { 
          path: 'index', 
          component: IndexComponent, 
          canActivate: [canActivateAuthentificatedChild], 
        },
        { 
          path: 'login', 
          component: LoginComponent,
          canActivate: [canActivateNotAuthentificatedChild],
        },
        { 
          path: 'register', 
          component: RegisterComponent,
          canActivate: [canActivateNotAuthentificatedChild],
        },
        { 
          path: 'profile', 
          component: UserProfileComponent, 
          canActivate: [canActivateAuthentificatedChild], 
        },
        { path: '**', component: NotFoundComponent}
      ]
    )
  ],
};
