import { Component, Signal, computed, signal } from "@angular/core";
import { Router } from "@angular/router";
import { NgFor, CurrencyPipe } from "@angular/common";
import { MatToolbarModule } from '@angular/material/toolbar';

import { FlightSummaryComponent } from '../../shared/flight-summary/flight-summary.component';
import { FlightResponse } from "src/app/services";
import { FlightRepository } from "src/app/services/flight.repository";
import { PlatformService } from "src/app/platform.service";
import { CounterDirective } from "src/app/shared/counter.directive";

@Component({
    selector: 'app-flights',
    standalone: true,
    imports: [FlightSummaryComponent, MatToolbarModule, NgFor, CurrencyPipe],
    templateUrl: './flights.component.html',
    styleUrl: './flights.component.css',
    viewProviders: [CounterDirective],
})
export class FlightsComponent {
    flights: Signal<FlightResponse[]> = signal([]);
    flightsPerPage = signal(4);
    selectedPage = signal(1);
    pagedFligts: Signal<FlightResponse[]> = signal([]);
    pageCount: Signal<number>;

    constructor(private repository: FlightRepository, 
        private router: Router, 
        private ps: PlatformService) {

        this.flights = this.repository.getFlights(this.selectedPage(), this.flightsPerPage())

        let pageIndex = computed(() => {
            return (this.selectedPage() - 1) * this.flightsPerPage()
        });        
    
        this.pagedFligts = computed(() => {
            return this.flights().slice(pageIndex(), 
                pageIndex() + this.flightsPerPage());
        });

        this.pageCount = computed(() => {
            return Math.ceil(this.flights().length 
                / this.flightsPerPage());
        });
    }

    changePage(newPage: number) {
        this.selectedPage.set(newPage);
    }

    changePageSize(newSize: number) {
        this.flightsPerPage.set(Number(newSize));
        this.changePage(1);
    }

    orderFlight(flight: FlightResponse) {
        
    }

    get isServer() { return this.ps.isServer }  
}
