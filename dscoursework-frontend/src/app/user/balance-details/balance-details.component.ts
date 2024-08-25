import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';

import { BalanceHistory, TicketResponse } from 'src/app/services';
import { sampleAnimeHeroesImageUrls } from 'src/assets/sample.animeHeroesImagesUrls';

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
export class BalanceDetailsComponent implements OnInit {

  @Input() balanceHistory: BalanceHistory | null = null;

  constructor() { }

  ngOnInit(): void {

  }
}
