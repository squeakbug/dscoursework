import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from 'src/app/services';

@Component({
  standalone: true,
  selector: 'dialog-failed-login',
  templateUrl: './dialog-failed-login.component.html',
  styleUrls: ['./dialog-failed-login.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFailedLoginComponent implements OnInit {
  @Input() flight: FlightResponse | null = null;

  constructor(public dialogRef: MatDialogRef<DialogFailedLoginComponent>) {

  }

  ngOnInit(): void {

  }

  orderFlight(flight: FlightResponse | null) {

  }
}
