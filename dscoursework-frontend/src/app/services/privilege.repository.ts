import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { TicketResponse } from "../models/TicketResponse";

@Injectable()
export class PrivilegeRepository {

    constructor(private dataSource: DataSource) {}

    getTicket(ticketUid: string): Signal<TicketResponse | null> {
        let ticketSignal = signal<TicketResponse | null>(null);
        this.dataSource.getTicket(ticketUid).subscribe(data => {
            try {
                ticketSignal.set(data);
            } catch (ex) {
                ticketSignal.set(null)
            }
        });
        return ticketSignal.asReadonly();
    }

    listTickets(): Signal<TicketResponse[]> {
        let ticketSignal = signal<TicketResponse[]>([]);
        this.dataSource.listTickets().subscribe(data => {
            try {
                ticketSignal.set(data);
            } catch (ex) {
                ticketSignal.set([])
            }
        });
        return ticketSignal.asReadonly();
    }
}
