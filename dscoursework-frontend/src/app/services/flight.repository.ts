import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { FlightResponse } from "../models/FlightResponse";

@Injectable()
export class FlightRepository {
    private flightsSignal = signal<FlightResponse[]>([]);

    constructor(private dataSource: DataSource) {}

    getFlights(page: number, size: number): Signal<FlightResponse[]> {
        this.dataSource.getFlights(page, size).subscribe(data => {
            try {
                this.flightsSignal.set(data.items as FlightResponse[]);
            } catch (ex) {
                this.flightsSignal.set([])
            }
        });
        return this.flightsSignal.asReadonly();
    }
}
