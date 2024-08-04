import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { FlightResponse } from "../models/FlightResponse";

@Injectable()
export class FlightRepository {
    private flightsSignal = signal<FlightResponse[]>([]);
    private loaded: boolean = false;

    constructor(private dataSource: DataSource) {}

    loadFlights(page: number, size: number) {
        this.loaded = true;
        this.dataSource.getFlights(page, size).subscribe(data => {
            try {
                this.flightsSignal.set(data.items as FlightResponse[]);
            } catch (ex) {
                this.flightsSignal.set([])
            }
        });
    }

    getFlights(page: number, size: number): Signal<FlightResponse[]> {
        if (!this.loaded) {
            this.loadFlights(page, size);
        }
        return this.flightsSignal.asReadonly();
    }
}
