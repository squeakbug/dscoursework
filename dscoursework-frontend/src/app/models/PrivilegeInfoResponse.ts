/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { BalanceHistory } from './BalanceHistory';
export type PrivilegeInfoResponse = {
    /**
     * Баланс бонусного счета
     */
    balance?: string;
    /**
     * Статус в бонусной программе
     */
    status?: PrivilegeInfoResponse.status;
    /**
     * История изменения баланса
     */
    history?: Array<BalanceHistory>;
};
export namespace PrivilegeInfoResponse {
    /**
     * Статус в бонусной программе
     */
    export enum status {
        BRONZE = 'BRONZE',
        SILVER = 'SILVER',
        GOLD = 'GOLD',
    }
}
