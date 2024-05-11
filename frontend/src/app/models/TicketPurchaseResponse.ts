/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PrivilegeShortInfo } from './PrivilegeShortInfo';
export type TicketPurchaseResponse = {
    /**
     * UUID билета
     */
    ticketUid?: string;
    /**
     * Номер полета
     */
    flightNumber?: string;
    /**
     * Страна и аэропорт прибытия
     */
    fromAirport?: string;
    /**
     * Страна и аэропорт прибытия
     */
    toAirport?: string;
    /**
     * Время вылета
     */
    date?: string;
    /**
     * Статус билета
     */
    status?: TicketPurchaseResponse.status;
    /**
     * Стоимость
     */
    price?: number;
    /**
     * Сумма оплаченная деньгами
     */
    paidByMoney?: number;
    /**
     * Сумма оплаченная бонусами
     */
    paidByBonuses?: number;
    privilege?: PrivilegeShortInfo;
};
export namespace TicketPurchaseResponse {
    /**
     * Статус билета
     */
    export enum status {
        PAID = 'PAID',
        CANCELED = 'CANCELED',
    }
}

