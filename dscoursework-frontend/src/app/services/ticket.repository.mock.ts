import { Injectable, Signal, signal } from "@angular/core";

import { TicketResponse } from "../models/TicketResponse";
import { TicketPurchaseRequest } from "../models/TicketPurchaseRequest";
import { TicketPurchaseResponse } from "../models/TicketPurchaseResponse";

@Injectable()
export class TicketRepositoryMock {
    readonly tickets: Array<TicketResponse> = [
        
    ];

    getTicket(ticketUid: string): Signal<TicketResponse | null> {
        let ticketSignal = signal<TicketResponse>(this.tickets[0]);
        return ticketSignal.asReadonly();
    }

    listTickets(): Signal<TicketResponse[]> {
        let ticketSignal = signal<TicketResponse[]>(this.tickets);
        return ticketSignal.asReadonly();
    }

    buyTicket(ticket: TicketPurchaseRequest): Signal<TicketPurchaseResponse | null> {
        return signal(this.tickets[0]).asReadonly();
    }

    deleteTicket(ticketUid: string) {
        let ticketSignal = signal<TicketResponse[]>([]);
        ticketSignal.set(this.tickets);
    }
}
