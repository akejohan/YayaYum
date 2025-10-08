/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateDish } from '../models/CreateDish';
import type { Dish } from '../models/Dish';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class DishesService {
    /**
     * @returns Dish List dishes
     * @throws ApiError
     */
    public static getDishes(): CancelablePromise<Array<Dish>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/dishes',
        });
    }
    /**
     * @param requestBody
     * @returns Dish Dish created
     * @throws ApiError
     */
    public static createDish(
        requestBody: CreateDish,
    ): CancelablePromise<Dish> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/dishes',
            body: requestBody,
            mediaType: 'application/json',
        });
    }
    /**
     * @param id Dish ID to modify
     * @param requestBody
     * @returns Dish Dish updated successfully
     * @throws ApiError
     */
    public static modifyDish(
        id: number,
        requestBody: CreateDish,
    ): CancelablePromise<Dish> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/dishes/{id}',
            path: {
                'id': id,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                404: `Dish not found`,
            },
        });
    }
    /**
     * @param id Dish ID to remove
     * @returns void
     * @throws ApiError
     */
    public static removeDish(
        id: number,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/dishes/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Dish not found`,
            },
        });
    }
}
