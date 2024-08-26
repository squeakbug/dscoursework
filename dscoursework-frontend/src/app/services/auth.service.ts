import { Inject, Injectable, InjectionToken } from '@angular/core';
import { BrowserStorageService } from './storage.service';

import { OAuthService } from 'angular-oauth2-oidc';
import { filter } from 'rxjs/operators';

import { authCodeFlowConfig } from './login.config';

@Injectable()
export class AuthService {

  constructor(
    private oauthService: OAuthService,
  ) {
    this.oauthService.configure(authCodeFlowConfig);
    this.oauthService.loadDiscoveryDocumentAndLogin();
    this.oauthService.events
      .pipe(filter((e) => e.type === 'token_received'))
      .subscribe((_subscriber) => this.oauthService.loadUserProfile());
  }

  get userName(): string | null {
    const claims = this.oauthService.getIdentityClaims();
    if (!claims) return null;
    return claims['given_name'];
  }

  get idToken(): string {
    return this.oauthService.getIdToken();
  }

  get accessToken(): string {
    return this.oauthService.getAccessToken();
  }

  login() {
    this.oauthService.initLoginFlow();
  }

  logout() {
    this.oauthService.logOut();
  }

  refresh() {
    this.oauthService.refreshToken();
  }

}