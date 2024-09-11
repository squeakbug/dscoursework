import { Component , OnInit } from '@angular/core';
import { NgIf } from '@angular/common'
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormBuilder, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';

import { ImageLoaderService } from '../../services/image-loader.service';
import { AuthService } from 'src/app/services/auth.service';

@Component({
  standalone: true,
  selector: 'app-login',
  imports: [
    NgIf, MatButtonModule, 
    ReactiveFormsModule, MatCardModule, 
    MatFormFieldModule, MatInputModule
  ],
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
    private authService: AuthService,
  ) {
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

  onLoginSSO() {
    this.authService.loginSSO();
  }

  onLogin() {
    this.authService.login();
  }

}
