/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { FlightResponse } from './FlightResponse';
export type PaginationResponse = {
    /**
     * Номер страницы
     */
    page?: number;
    /**
     * Количество элементов на странице
     */
    pageSize?: number;
    /**
     * Общее количество элементов
     */
    totalElements?: number;
    items?: Array<FlightResponse>;
};
