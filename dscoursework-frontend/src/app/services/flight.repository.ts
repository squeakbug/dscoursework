import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { FlightResponse } from "../models/FlightResponse";

@Injectable()
export class FlightRepository {

    constructor(private dataSource: DataSource) {}

    getFlights(page: number, size: number): Signal<FlightResponse[]> {
        let flightsSignal = signal<FlightResponse[]>([]);
        this.dataSource.getFlights(page, size).subscribe(data => {
            try {
                flightsSignal.set(data.items as FlightResponse[]);
            } catch (ex) {
                flightsSignal.set([])
            }
        });
        return flightsSignal.asReadonly();
    }
}
