import { Component } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { Signal, signal } from '@angular/core';

import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatFormFieldModule } from '@angular/material/form-field';

@Component({
  selector: 'app-not-found',
  standalone: true,
  imports: [
    MatInputModule,
    MatButtonModule,
    MatDatepickerModule,
    MatFormFieldModule,
],
  templateUrl: './not-found.component.html',
  styleUrl: './not-found.component.scss',
  providers: [ ],
})
export class NotFoundComponent {

}
