import { Component } from '@angular/core';
import { FormsModule, ReactiveFormsModule } from '@angular/forms';
import { Signal, signal } from '@angular/core';

import { MatInputModule } from '@angular/material/input';
import { MatButtonModule } from '@angular/material/button';
import { MatDatepickerModule } from '@angular/material/datepicker';
import { MatFormFieldModule } from '@angular/material/form-field';
import { provideNativeDateAdapter } from '@angular/material/core';

import { FlightRepository } from 'src/app/services/flight.repository';
import { FlightsComponent } from "../../flight/flights/flights.component";
import { FlightResponse } from'src/app/services';

@Component({
  selector: 'app-index',
  standalone: true,
  imports: [
    MatInputModule,
    MatButtonModule,
    MatDatepickerModule,
    MatFormFieldModule,
    FormsModule,
    ReactiveFormsModule,
    FlightsComponent,
],
  templateUrl: './index.component.html',
  styleUrl: './index.component.scss',
  providers: [
    provideNativeDateAdapter()
  ],
})
export class IndexComponent {
  today: Date = new Date();
  month = this.today.getMonth();
  year = this.today.getFullYear();

  departureDate: Date | null = null;
  fromAirport: string = '';
  toAirport: string = '';

  isFinded: boolean = false;
  flights: Signal<FlightResponse[]> = signal([]);

  readonly initPageNumber: number = 0;
  readonly initPageSize: number = 10;

  constructor(private flightRepo: FlightRepository) { 
    this.flightRepo = flightRepo;
  }

  async findFlights() {
    this.flights = this.flightRepo.getFlights(this.initPageNumber, this.initPageSize);
    this.isFinded = true;
  }
}
