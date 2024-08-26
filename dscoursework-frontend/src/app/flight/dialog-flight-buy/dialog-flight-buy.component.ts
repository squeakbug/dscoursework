import { Component, Input } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { MatDialogRef } from '@angular/material/dialog';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from 'src/app/models/FlightResponse';
import { TicketPurchaseRequest } from 'src/app/models/TicketPurchaseRequest';
import { TicketRepository } from 'src/app/services/ticket.repository';

@Component({
  standalone: true,
  selector: 'app-dialog-flight-buy',
  templateUrl: './dialog-flight-buy.component.html',
  styleUrls: ['./dialog-flight-buy.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatDialogModule,
  ],
})
export class DialogFlightBuyComponent {
  @Input() flight: FlightResponse | null = null;

  constructor(
    public dialogRef: MatDialogRef<DialogFlightBuyComponent>,
    private ticketRepository: TicketRepository,
  ) {

  }

  orderFlight() {
    const buyRequest: TicketPurchaseRequest = {
      flightNumber: this.flight?.flightNumber,
      paidFromBalance: false,
      price: this.flight?.price,
    }
    this.ticketRepository.buyTicket(buyRequest);
  }
}
