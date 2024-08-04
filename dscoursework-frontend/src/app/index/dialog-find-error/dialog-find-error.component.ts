import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from 'src/app/services';

@Component({
  standalone: true,
  selector: 'dialog-find-error',
  templateUrl: './dialog-find-error.component.html',
  styleUrls: ['./dialog-find-error.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFindErrorComponent implements OnInit {
  @Input() flight: FlightResponse | null = null;

  constructor(public dialogRef: MatDialogRef<DialogFindErrorComponent>) {

  }

  ngOnInit(): void {

  }

  orderFlight(flight: FlightResponse | null) {

  }
}
