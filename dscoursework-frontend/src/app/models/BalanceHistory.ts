/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type BalanceHistory = {
    /**
     * Дата и время операции
     */
    date?: string;
    /**
     * Изменение баланса
     */
    balanceDiff?: string;
    /**
     * UUID билета по которому была операция с бонусами
     */
    ticketUid?: string;
    /**
     * Тип операции
     */
    operationType?: BalanceHistory.operationType;
};
export namespace BalanceHistory {
    /**
     * Тип операции
     */
    export enum operationType {
        FILL_IN_BALANCE = 'FILL_IN_BALANCE',
        DEBIT_THE_ACCOUNT = 'DEBIT_THE_ACCOUNT',
        FILLED_BY_MONEY = 'FILLED_BY_MONEY',
    }
}
