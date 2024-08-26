import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { TicketResponse } from "../models/TicketResponse";
import { PrivilegeInfoResponse } from "../models/PrivilegeInfoResponse";
import { PrivilegeShortInfo } from "../models/PrivilegeShortInfo";

@Injectable()
export class PrivilegeRepository {

    constructor(private dataSource: DataSource) {}

    getMe(): Signal<PrivilegeShortInfo | null> {
        let privilegeSignal = signal<PrivilegeShortInfo | null>(null);
        this.dataSource.getMe().subscribe(data => {
            try {
                privilegeSignal.set(data);
            } catch (ex) {
                privilegeSignal.set(null)
            }
        });
        return privilegeSignal.asReadonly();
    }

    getPrivilege(): Signal<PrivilegeInfoResponse | null> {
        let privilegeSignal = signal<PrivilegeShortInfo | null>(null);
        this.dataSource.getPrivilege().subscribe(data => {
            try {
                privilegeSignal.set(data);
            } catch (ex) {
                privilegeSignal.set(null)
            }
        });
        return privilegeSignal.asReadonly();
    }
}
