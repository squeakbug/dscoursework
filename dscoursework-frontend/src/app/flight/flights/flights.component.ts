import { Component, Input, Signal, computed, signal } from "@angular/core";
import { Router } from "@angular/router";
import { NgFor, CurrencyPipe } from "@angular/common";

import { MatPaginatorModule } from '@angular/material/paginator';
import { MatToolbarModule } from '@angular/material/toolbar';
import { MatSelectModule } from '@angular/material/select';
import { MatCardModule } from '@angular/material/card';
import { MatGridListModule } from '@angular/material/grid-list';
import { MatDialogModule } from '@angular/material/dialog';

import { FlightResponse } from "src/app/services";
import { FlightRepository } from "src/app/services/flight.repository";
import { PlatformService } from "src/app/platform.service";
import { CounterDirective } from "src/app/shared/counter.directive";
import { ToolbarComponent } from "src/app/shared/toolbar/toolbar.component";

import { FlightDetailComponent } from "../flight-detail/flight-detail.component";
import { FlightFiltersComponent } from "../flight-filters/flight-filters.component";

@Component({
    selector: 'app-flights',
    standalone: true,
    imports: [
        FlightDetailComponent,
        FlightFiltersComponent,
        ToolbarComponent,
        MatPaginatorModule,
        MatSelectModule,
        MatToolbarModule,
        MatCardModule,
        MatGridListModule,
        MatDialogModule,
        NgFor, CurrencyPipe
    ],
    templateUrl: './flights.component.html',
    styleUrl: './flights.component.scss',
    viewProviders: [CounterDirective],
})
export class FlightsComponent {
    readonly perPage: number[] = [10, 20, 50, 100];
    flightsPerPage = signal(this.perPage[0]);
    selectedPage = signal(1);
    pagedFligts: Signal<FlightResponse[]> = signal([]);
    pageCount: Signal<number>;

    @Input() flights: Signal<FlightResponse[]> = signal([]);

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

    get isServer() { return this.ps.isServer }
}
