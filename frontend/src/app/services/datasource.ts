import { Injectable } from "@angular/core";
import { HttpClient } from "@angular/common/http";
import { HttpHeaders } from '@angular/common/http';

import { Observable } from "rxjs";

import { PlatformService } from "../platform.service";
import { PaginationResponse } from "../models/PaginationResponse";
import { TicketResponse } from "../models/TicketResponse";
import { TicketPurchaseRequest } from "../models/TicketPurchaseRequest";
import { TicketPurchaseResponse } from "../models/TicketPurchaseResponse";
import { UserInfoResponse } from "../models/UserInfoResponse";
import { PrivilegeInfoResponse } from "../models/PrivilegeInfoResponse";

const SCHEMA = "http";
const PORT = 3500;

@Injectable()
export class DataSource {
    baseUrl: string;
    auth_token?: string;

    constructor(private http: HttpClient, ps: PlatformService) {
        this.baseUrl = ps.isServer 
            ? `${SCHEMA}://localhost:${PORT}` 
            : "/api/v1";
    }

    getFlights(page: number, size: number): Observable<PaginationResponse> {
        return this.http.get<PaginationResponse>(`${this.baseUrl}/flights/?page=${page}&size=${size}`)
    }

    getTickets(): Observable<TicketResponse[]> {
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

    getMe(): Observable<UserInfoResponse> {
        return this.http.get<UserInfoResponse>(`${this.baseUrl}/me`)
    }

    getPrivilege(): Observable<PrivilegeInfoResponse> {
        return this.http.get<PrivilegeInfoResponse>(`${this.baseUrl}/privilege`)
    }

    private getOptions() {
        return {
            headers: new HttpHeaders({
                "Authorization": `Bearer<${this.auth_token}>`
            })
        }
    }
}