/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { PrivilegeShortInfo } from './PrivilegeShortInfo';
import type { TicketResponse } from './TicketResponse';
export type UserInfoResponse = {
    /**
     * Информация о билетах пользоватлея
     */
    tickets?: Array<TicketResponse>;
    privilege?: PrivilegeShortInfo;
};
