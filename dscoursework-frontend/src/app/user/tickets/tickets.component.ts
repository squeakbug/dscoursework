import { Component, Input, Signal, signal } from '@angular/core';
import { Router } from '@angular/router';

import { MatGridListModule } from '@angular/material/grid-list';
import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';

import { PlatformService } from 'src/app/platform.service';
import { TicketResponse } from 'src/app/services';
import { TicketRepository } from 'src/app/services/ticket.repository';

import { TicketDetailsComponent } from '../ticket-details/ticket-details.component';

@Component({
  selector: 'app-tickets',
  standalone: true,
  imports: [
    TicketDetailsComponent,
    MatGridListModule,
    MatPaginatorModule,
  ],
  templateUrl: './tickets.component.html',
  styleUrl: './tickets.component.scss'
})
export class TicketsComponent {
  readonly pageSizeOptions: number[] = [10, 20, 50, 100];
  pageIndex = 0;
  pageSize = this.pageSizeOptions[this.pageIndex];
  length = 50;

  @Input() tickets: Signal<TicketResponse[]> = signal([]);

  constructor(private repository: TicketRepository,
    private router: Router,
    private ps: PlatformService) {

    this.tickets = this.repository.listTickets()
  }

  changePage(e: PageEvent) {

  }

  get isServer() { return this.ps.isServer }
}
