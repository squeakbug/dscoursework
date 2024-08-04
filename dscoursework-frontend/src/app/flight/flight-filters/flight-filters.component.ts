import { Component, Input, OnInit } from '@angular/core';
import { MatCardModule } from '@angular/material/card';
import { MatButtonModule } from '@angular/material/button';
import { FlightResponse } from 'src/app/services';

import { MatExpansionModule } from '@angular/material/expansion';
import { MatListModule } from '@angular/material/list';
import { MatSliderModule } from '@angular/material/slider';

@Component({
  standalone: true,
  selector: 'app-flight-filters',
  templateUrl: './flight-filters.component.html',
  styleUrls: ['./flight-filters.component.scss'],
  imports: [
    MatButtonModule,
    MatCardModule,
    MatExpansionModule,
    MatListModule,
    MatSliderModule,
  ],
})
export class FlightFiltersComponent implements OnInit {

  isPanelOpened: boolean = false;

  constructor() { }

  ngOnInit(): void {

  }

  resetFilters(flight: FlightResponse | null) {

  }

  showAllFilters(flight: FlightResponse | null) {

  }

  formatLabel(value: number): string {
    if (value >= 1000) {
      return Math.round(value / 1000) + 'k';
    }

    return `${value}`;
  }
}
