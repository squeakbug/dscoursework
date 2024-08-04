import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from 'src/app/services';

@Component({
  standalone: true,
  selector: 'dialog-flight-buy',
  templateUrl: './dialog-flight-buy.component.html',
  styleUrls: ['./dialog-flight-buy.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFlightBuyComponent implements OnInit {
  @Input() flight: FlightResponse | null = null;

  constructor(public dialogRef: MatDialogRef<DialogFlightBuyComponent>) {

  }

  ngOnInit(): void {

  }

  orderFlight(flight: FlightResponse | null) {

  }
}
