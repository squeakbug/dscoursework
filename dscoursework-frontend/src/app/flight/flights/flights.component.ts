import { Component, Input, Signal, computed, signal } from "@angular/core";
import { Router } from "@angular/router";
import { NgFor, CurrencyPipe } from "@angular/common";

import { MatPaginatorModule, PageEvent } from '@angular/material/paginator';
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
    readonly pageSizeOptions: number[] = [10, 20, 50, 100];
    pageIndex = 0;
    pageSize = this.pageSizeOptions[this.pageIndex];
    length = 50;

    @Input() flights: Signal<FlightResponse[]> = signal([]);

    constructor(
        private repository: FlightRepository,
        private router: Router,
        private ps: PlatformService
    ) {

    }

    changePage(event: PageEvent) {
        this.pageIndex = event.pageIndex;
        this.pageSize = event.pageSize;
        this.length = event.length;
        this.flights = this.repository.getFlights(
            this.pageIndex,
            this.pageSize
        );
    }

    get isServer() { return this.ps.isServer }
}
