import { Component } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

@Component({
  standalone: true,
  selector: 'app-dialog-failed-register',
  templateUrl: './dialog-failed-register.component.html',
  styleUrls: ['./dialog-failed-register.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFailedRegisterComponent {

  constructor(public dialogRef: MatDialogRef<DialogFailedRegisterComponent>) {

  }

}
