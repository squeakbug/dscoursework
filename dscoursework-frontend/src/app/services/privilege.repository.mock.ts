import { Injectable, Signal, signal } from "@angular/core";
import { DataSource } from "./datasource";
import { FlightResponse } from "../models/FlightResponse";
import { PrivilegeShortInfo } from "../models/PrivilegeShortInfo";
import { PrivilegeInfoResponse } from "../models/PrivilegeInfoResponse";
import { BalanceHistory } from "../models/BalanceHistory";

@Injectable()
export class PrivilegeRepositoryMock {
    readonly privilege: PrivilegeInfoResponse = {
        balance: "1000",
        status: PrivilegeInfoResponse.status.BRONZE,
        history: [
            {
                date: "2021-12-31",
                balanceDiff: "900",
                ticketUid: "some-uid",
                operationType: BalanceHistory.operationType.FILL_IN_BALANCE,
            },
            {
                date: "2021-11-30",
                balanceDiff: "1000",
                ticketUid: "another-uid",
                operationType: BalanceHistory.operationType.DEBIT_THE_ACCOUNT,
            },
        ],
    }

    getMe(): Signal<PrivilegeShortInfo> {
        return signal({
            balance: this.privilege.balance,
            status: this.privilege.status,
        }).asReadonly();
    }

    getPrivilege(): Signal<PrivilegeInfoResponse> {
        return signal(this.privilege).asReadonly();
    }
}
