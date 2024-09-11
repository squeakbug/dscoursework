import { inject, Injectable } from '@angular/core';
import { Router } from '@angular/router';

import { LoginResponse, OidcSecurityService } from 'angular-auth-oidc-client';
import { BehaviorSubject, combineLatest, Observable } from 'rxjs';
import { filter, map } from 'rxjs/operators';

@Injectable({
  providedIn: 'root'
})
export class AuthService {

  configuration$ = this.oauthService.getConfiguration();
  userData$ = this.oauthService.userData$;

  isAuthenticated$ = this.oauthService.isAuthenticated$;

  constructor(private readonly oauthService: OidcSecurityService) {
    this.oauthService
      .checkAuth()
      .subscribe(({ isAuthenticated, accessToken }) => {
        console.log('app authenticated', isAuthenticated);
    });
  }

  public loginSSO() {
    this.oauthService.authorize();
  }
  public login() {
    this.oauthService.authorize();
  }
  public logout() { 
    this.oauthService
      .logoff()
      .subscribe((result) => console.log(result));
  }
  public logoffAndRevokeTokens(): void {
    this.oauthService
      .logoffAndRevokeTokens()
      .subscribe((result) => console.log(result));
  }

  public revokeRefreshToken(): void {
    this.oauthService
      .revokeRefreshToken()
      .subscribe((result) => console.log(result));
  }

  public revokeAccessToken(): void {
    this.oauthService
      .revokeAccessToken()
      .subscribe((result) => console.log(result));
  }

  public get accessToken() { 
    return this.oauthService.getAccessToken(); 
  }
  public get refreshToken() { 
    return this.oauthService.getRefreshToken(); 
  }
  public get idToken() { 
    return this.oauthService.getIdToken(); 
  }

  public refresh(): void {
    this.oauthService
      .forceRefreshSession()
      .subscribe((result) => console.log(result));
  }

  public get nickname(): Observable<string> {
    return this.userData$.pipe(map((userData) => 
      userData.userData["preferred_nickname"]
    ))
  }

}