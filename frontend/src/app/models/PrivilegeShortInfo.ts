/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
export type PrivilegeShortInfo = {
    /**
     * Баланс бонусного счета
     */
    balance?: string;
    /**
     * Статус в бонусной программе
     */
    status?: PrivilegeShortInfo.status;
};
export namespace PrivilegeShortInfo {
    /**
     * Статус в бонусной программе
     */
    export enum status {
        BRONZE = 'BRONZE',
        SILVER = 'SILVER',
        GOLD = 'GOLD',
    }
}

