/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type TicketPurchaseRequest = {
    /**
     * Номер полета
     */
    flightNumber?: string;
    /**
     * Стоимость
     */
    price?: number;
    /**
     * Выполнить списание бонусных баллов в учет покупки билета
     */
    paidFromBalance?: boolean;
};

