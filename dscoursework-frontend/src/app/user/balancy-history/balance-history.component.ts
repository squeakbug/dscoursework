import { Component, computed, Input, OnInit, Signal, signal } from '@angular/core';

import { MatGridListModule } from '@angular/material/grid-list';
import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';

import { BalanceHistory } from 'src/app/models/BalanceHistory';

import { BalanceDetailsComponent } from '../balance-details/balance-details.component';
import { PrivilegeInfoResponse } from 'src/app/models/PrivilegeInfoResponse';
import { PrivilegeRepository } from 'src/app/services/privilege.repository';

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
export class BalanceHistoryComponent implements OnInit {
  privilege: Signal<PrivilegeInfoResponse | null> = signal(null);
  history: Signal<BalanceHistory[] | undefined> = computed(() => this.privilege()?.history );
  pagedHistory: Signal<BalanceHistory[] | undefined> = computed(() => {
    return this.privilege()?.history?.slice(
      this.pageIndex() * this.pageSize(),
      (this.pageIndex() + 1) * this.pageSize()
    )
  });

  readonly pageSizeOptions: number[] = [10, 20, 50, 100];
  pageIndex = signal(0);
  pageSize = signal(this.pageSizeOptions[this.pageIndex()]);
  length = computed(() => this.history()?.length);

  constructor(
    private privilegeRepository: PrivilegeRepository,
  ) {

  }
  
  ngOnInit(): void {
    this.privilege = this.privilegeRepository.getPrivilege();
  }

  changePage(event: PageEvent) {
    this.pageIndex.set(event.pageIndex);
    this.pageSize.set(event.pageSize);
  }
}
