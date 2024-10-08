import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { TicketResponse } from "../models/TicketResponse";
import { TicketPurchaseRequest } from "../models/TicketPurchaseRequest";
import { TicketPurchaseResponse } from "../models/TicketPurchaseResponse";

@Injectable()
export class TicketRepository {

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

    buyTicket(ticket: TicketPurchaseRequest): Signal<TicketPurchaseResponse | null> {
        let ticketSignal = signal<TicketPurchaseResponse | null>(null);
        this.dataSource.postTicket(ticket).subscribe(data => {
            try {
                ticketSignal.set(data);
            } catch (ex) {
                ticketSignal.set(null)
            }
        });
        return ticketSignal.asReadonly();
    }

    deleteTicket(ticketUid: string) {
        this.dataSource.deleteTicket(ticketUid)
    }
}
