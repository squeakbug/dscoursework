import { Injectable, Signal, signal } from "@angular/core";

import { TicketResponse } from "../models/TicketResponse";
import { TicketPurchaseRequest } from "../models/TicketPurchaseRequest";
import { TicketPurchaseResponse } from "../models/TicketPurchaseResponse";

@Injectable()
export class TicketRepositoryMock {
    readonly tickets = new Array<TicketResponse>();

    constructor() {
        for (let i = 0; i < 100; i++) {
            this.tickets.push(
                {
                    ticketUid: "1",
                    flightNumber: "A123",
                    fromAirport: "Moscow",
                    toAirport: "London",
                    date: "2022-01-01 10:00",
                    price: 1000,
                    status: TicketResponse.status.PAID
                }
            )
        }
    }

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
