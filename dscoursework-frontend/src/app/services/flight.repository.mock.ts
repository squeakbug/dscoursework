import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { FlightResponse } from "../models/FlightResponse";

@Injectable()
export class FlightRepositoryMock {

    readonly flights = new Array<FlightResponse>();

    constructor() {
        // Mock data
        for (let i = 0; i < 100; i++) {
            this.flights.push({
                flightNumber: `Flight ${i + 1}`,
                fromAirport: "City A",
                toAirport: "City B",
                date: "2022-01-01 10:00",
                price: 1000 + i * 100,
            });
        }
    }

    getFlights(page: number, size: number): Signal<FlightResponse[]> {
        let flightsSignal = signal<FlightResponse[]>([]);
        flightsSignal.set(this.flights.slice(page * size, (page + 1) * size));
        return flightsSignal.asReadonly();
    }
}
