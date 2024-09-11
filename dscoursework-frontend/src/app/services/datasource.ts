import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { HttpHeaders } from '@angular/common/http';

import { Observable, from } from "rxjs";

import { PlatformService } from "../platform.service";
import { PaginationResponse } from "../models/PaginationResponse";
import { TicketResponse } from "../models/TicketResponse";
import { TicketPurchaseRequest } from "../models/TicketPurchaseRequest";
import { TicketPurchaseResponse } from "../models/TicketPurchaseResponse";
import { PrivilegeInfoResponse } from "../models/PrivilegeInfoResponse";
import { environment } from "src/environments/environment";
import { samplePaginationResponse } from "src/assets/sample.PaginationResponse";
import { PrivilegeShortInfo } from "../models/PrivilegeShortInfo";

const SCHEMA = "http";
const PORT = 30000;

@Injectable()
export class DataSource {
    baseUrl: string;
    auth_token?: string;

    constructor(private http: HttpClient) {
        this.baseUrl = environment.gatewayApiUrl
    }

    getFlights(page: number, size: number): Observable<PaginationResponse> {
        return this.http.get<PaginationResponse>(
            `${this.baseUrl}/flights/?page=${page}&size=${size}`,
        )
    }

    listTickets(): Observable<TicketResponse[]> {
        return this.http.get<TicketResponse[]>(`${this.baseUrl}/tickets`)
    }

    postTicket(ticket: TicketPurchaseRequest): Observable<TicketPurchaseResponse> {
        return this.http.post<TicketPurchaseResponse>(`${this.baseUrl}/products`,
            ticket);
    }

    getTicket(ticketUid: string): Observable<TicketResponse> {
        return this.http.get<TicketResponse>(`${this.baseUrl}/tickets/${ticketUid}`)
    }

    deleteTicket(ticketUid: string): Observable<TicketResponse> {
        return this.http.delete(`${this.baseUrl}/tickets/${ticketUid}`)
    }

    getMe(): Observable<PrivilegeShortInfo> {
        return this.http.get<PrivilegeShortInfo>(`${this.baseUrl}/me`)
    }

    getPrivilege(): Observable<PrivilegeInfoResponse> {
        return this.http.get<PrivilegeInfoResponse>(`${this.baseUrl}/privilege`)
    }
}
