import { Component } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
  standalone: true,
  selector: 'app-dialog-failed-login',
  templateUrl: './dialog-failed-login.component.html',
  styleUrls: ['./dialog-failed-login.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFailedLoginComponent {

  constructor(public dialogRef: MatDialogRef<DialogFailedLoginComponent>) {

  }

}
