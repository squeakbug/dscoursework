import { Component , OnInit } from '@angular/core';
import { NgIf } from '@angular/common'
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormBuilder, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';

import { ImageLoaderService } from '../../services/image-loader.service';

@Component({
  standalone: true,
  selector: 'app-register',
  imports: [NgIf, MatButtonModule, ReactiveFormsModule, MatCardModule, MatFormFieldModule, MatInputModule],
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.scss']
})
export class RegisterComponent implements OnInit {
  registerForm: FormGroup;
  logoUrl = "";

  constructor(private formBuilder: FormBuilder, private imageLoader: ImageLoaderService) {
    this.registerForm = this.formBuilder.group({
      email: ['', [Validators.required, Validators.email]],
      password: ['', [Validators.required, Validators.minLength(6)]],
      firstName: ['', [Validators.required, Validators.minLength(6)]],
      secondName: ['', [Validators.required, Validators.minLength(6)]],
      lastName: ['', [Validators.minLength(6)]],
      login: ['', [Validators.required, Validators.minLength(6)]],
      mobilePhone: ['', [Validators.required, Validators.pattern('[- +()0-9]+')]],
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
    if (this.registerForm.valid) {
      console.log('Register form submitted');
    } else {
      return;
    }
  }
}
