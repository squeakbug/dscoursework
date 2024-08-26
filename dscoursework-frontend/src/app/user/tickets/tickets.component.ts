import { Component, computed, Input, OnInit, Signal, signal } from '@angular/core';

import { MatGridListModule } from '@angular/material/grid-list';
import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';

import { TicketResponse } from 'src/app/models/TicketResponse';
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
export class TicketsComponent implements OnInit {
  tickets: Signal<TicketResponse[]> = signal([]);
  pagedTickets: Signal<TicketResponse[] | undefined> = computed(() => {
    return this.tickets().slice(
      this.pageIndex() * this.pageSize(),
      (this.pageIndex() + 1) * this.pageSize()
    )
  });

  readonly pageSizeOptions: number[] = [10, 20, 50, 100];
  pageIndex = signal(0);
  pageSize = signal(this.pageSizeOptions[this.pageIndex()]);
  length = computed(() => this.tickets().length);

  constructor(
    private ticketRepository: TicketRepository,
  ) {
  }

  ngOnInit(): void {
    this.tickets = this.ticketRepository.listTickets();
  }

  changePage(event: PageEvent) {
    this.pageIndex.set(event.pageIndex);
    this.pageSize.set(event.pageSize);
  }
}
