import { Component , OnInit } from '@angular/core';
import { NgIf } from '@angular/common'
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormBuilder, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';

import { OAuthService } from 'angular-oauth2-oidc';
import { filter } from 'rxjs/operators';

import { authCodeFlowConfig } from './login.config';
import { ImageLoaderService } from '../../services/image-loader.service';

@Component({
  standalone: true,
  selector: 'app-login',
  imports: [NgIf, MatButtonModule, ReactiveFormsModule, MatCardModule, MatFormFieldModule, MatInputModule],
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.scss']
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;
  logoUrl = "";
  title = "Test";

  constructor(
    private formBuilder: FormBuilder, 
    private imageLoader: ImageLoaderService,
    private oauthService: OAuthService,
  ) {
    this.oauthService.configure(authCodeFlowConfig);
    this.oauthService.loadDiscoveryDocumentAndLogin();
    this.oauthService.events
      .pipe(filter((e) => e.type === 'token_received'))
      .subscribe((_subscriber) => this.oauthService.loadUserProfile());

    this.loginForm = this.formBuilder.group({
      email: ['', [Validators.required, Validators.email]],
      password: ['', [Validators.required, Validators.minLength(6)]]
    });
  }

  ngOnInit(): void {
    this.loadLogo()


  }

  loadLogo() {
    const logoUrl = 'https://avatars.githubusercontent.com/u/124091980';
    this.imageLoader.loadImage(logoUrl).subscribe((blob: Blob) => {
      this.logoUrl = URL.createObjectURL(blob);
    });
  }

  onSubmit(): void {
    if (this.loginForm.valid) {
      console.log('Login form submitted');
    } else {
      return;
    }
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

  refresh() {
    this.oauthService.refreshToken();
  }

}
