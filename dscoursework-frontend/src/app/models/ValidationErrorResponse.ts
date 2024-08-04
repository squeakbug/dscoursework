/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { ErrorDescription } from './ErrorDescription';
export type ValidationErrorResponse = {
    /**
     * Информация об ошибке
     */
    message?: string;
    /**
     * Массив полей с описанием ошибки
     */
    errors?: Array<ErrorDescription>;
};
