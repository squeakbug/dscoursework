import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { TicketResponse } from "../models/TicketResponse";

@Injectable()
export class TicketRepository {
    private ticketSignal = signal<TicketResponse[]>([]);
    private loaded: boolean = false;

    constructor(private dataSource: DataSource) {}

    loadTickets() {
        this.loaded = true;
        this.dataSource.getTickets().subscribe(data => {
            try {
                this.ticketSignal.set(data);
            } catch (ex) {
                this.ticketSignal.set([])
            }
        });
    }

    getTickets(): Signal<TicketResponse[]> {
        if (!this.loaded) {
            this.loadTickets();
        }
        return this.ticketSignal.asReadonly();
    }
}
