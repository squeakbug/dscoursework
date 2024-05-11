/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type TicketResponse = {
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
     * Дата и время вылета
     */
    date?: string;
    /**
     * Стоимость
     */
    price?: number;
    /**
     * Статус билета
     */
    status?: TicketResponse.status;
};
export namespace TicketResponse {
    /**
     * Статус билета
     */
    export enum status {
        PAID = 'PAID',
        CANCELED = 'CANCELED',
    }
}

