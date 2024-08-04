import { PaginationResponse } from "src/app/services";
import { sampleFlightResponses } from "./sample.FlightResponse";

export const samplePaginationResponse: PaginationResponse[] = [{
    page: 1,
    pageSize: 10,
    totalElements: 10,
    items: sampleFlightResponses
}];
