import { Component , OnInit } from '@angular/core';
import { NgIf } from '@angular/common'
import { MatCardModule } from '@angular/material/card';
import { MatInputModule } from '@angular/material/input';
import { MatFormFieldModule } from "@angular/material/form-field";
import { FormBuilder, FormGroup, Validators, ReactiveFormsModule } from '@angular/forms';

@Component({
  standalone: true,
  selector: 'app-register',
  imports: [NgIf, ReactiveFormsModule, MatCardModule, MatFormFieldModule, MatInputModule],
  templateUrl: './register.component.html',
  styleUrls: ['./register.component.css']
})
export class RegisterComponent implements OnInit {
  registerForm: FormGroup;

  constructor(private formBuilder: FormBuilder) {
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

  }

  onSubmit(): void {
    if (this.registerForm.valid) {
      console.log('Register form submitted');
    } else {
      return;
    }
  }
}