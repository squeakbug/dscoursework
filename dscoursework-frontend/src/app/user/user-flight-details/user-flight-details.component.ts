import { Component, Signal, signal } from '@angular/core';
import { MatGridListModule } from '@angular/material/grid-list';
import { Router } from '@angular/router';
import { PlatformService } from 'src/app/platform.service';
import { TicketResponse } from 'src/app/services';
import { TicketRepository } from 'src/app/services/ticket.repository';

import { UserFlightDetailComponent } from '../user-flight-detail/flight-detail.component';

@Component({
  selector: 'app-user-flight-details',
  standalone: true,
  imports: [
    UserFlightDetailComponent,
    MatGridListModule
  ],
  templateUrl: './user-flight-details.component.html',
  styleUrl: './user-flight-details.component.scss'
})
export class UserFlightDetailsComponent {
  readonly perPage: number[] = [10, 20, 50, 100];
    tickets: Signal<TicketResponse[]> = signal([]);

    constructor(private repository: TicketRepository,
        private router: Router,
        private ps: PlatformService) {

        this.tickets = this.repository.getTickets()
    }

    get isServer() { return this.ps.isServer }
}
