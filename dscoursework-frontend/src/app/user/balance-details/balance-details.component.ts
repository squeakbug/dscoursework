import { Component, Input } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

import { BalanceHistory } from 'src/app/models/BalanceHistory';

@Component({
  standalone: true,
  selector: 'app-balance-details',
  templateUrl: './balance-details.component.html',
  styleUrls: ['./balance-details.component.scss'],
  imports: [ 
    MatButtonModule,
    MatCardModule 
  ],
})
export class BalanceDetailsComponent {

  @Input() balanceHistory: BalanceHistory | null = null;

}
