import { Component, Input, Signal, signal } from '@angular/core';
import { Router } from '@angular/router';

import { MatGridListModule } from '@angular/material/grid-list';
import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';

import { PlatformService } from 'src/app/platform.service';
import { BalanceHistory } from 'src/app/services';
import { TicketRepository } from 'src/app/services/ticket.repository';

import { BalanceDetailsComponent } from '../balance-details/balance-details.component';

@Component({
  selector: 'app-balance-history',
  standalone: true,
  imports: [
    BalanceDetailsComponent,
    MatGridListModule,
    MatPaginatorModule,
  ],
  templateUrl: './balance-history.component.html',
  styleUrl: './balance-history.component.scss'
})
export class BalanceHistoryComponent {
  readonly pageSizeOptions: number[] = [10, 20, 50, 100];
  pageIndex = 0;
  pageSize = this.pageSizeOptions[this.pageIndex];
  length = 50;

  @Input() balanceHistory: Signal<BalanceHistory[]> = signal([]);

  constructor(private repository: TicketRepository,
    private router: Router,
    private ps: PlatformService
  ) {

  }

  changePage(e: PageEvent) {

  }

  get isServer() { return this.ps.isServer }
}
