import { Component , OnInit } from '@angular/core';
import { NgIf } from '@angular/common'
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormBuilder, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';

import { ImageLoaderService } from '../services/image-loader.service';

@Component({
  standalone: true,
  selector: 'app-login',
  imports: [NgIf, ReactiveFormsModule, MatCardModule, MatFormFieldModule, MatInputModule],
  templateUrl: './login.component.html',
  styleUrls: ['./login.component.css']
})
export class LoginComponent implements OnInit {
  loginForm: FormGroup;
  logoUrl: string = '';

  constructor(private formBuilder: FormBuilder, private imageLoader: ImageLoaderService) {
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
}