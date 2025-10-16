/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { CreateRating } from '../models/CreateRating';
import type { Rating } from '../models/Rating';
import type { CancelablePromise } from '../core/CancelablePromise';
import { OpenAPI } from '../core/OpenAPI';
import { request as __request } from '../core/request';
export class RatingsService {
    /**
     * @returns Rating List ratings
     * @throws ApiError
     */
    public static getRatings(): CancelablePromise<Array<Rating>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/ratings',
        });
    }
    /**
     * @param requestBody
     * @returns Rating Rating created
     * @throws ApiError
     */
    public static createRating(
        requestBody: CreateRating,
    ): CancelablePromise<Rating> {
        return __request(OpenAPI, {
            method: 'POST',
            url: '/ratings',
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                400: `Bad request - invalid rating value`,
                409: `Conflict - user already rated today`,
            },
        });
    }
    /**
     * @param dishId Dish ID
     * @returns Rating Ratings for dish
     * @throws ApiError
     */
    public static getRatingsByDish(
        dishId: number,
    ): CancelablePromise<Array<Rating>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/ratings/dish/{dish_id}',
            path: {
                'dish_id': dishId,
            },
        });
    }
    /**
     * @param userId User ID
     * @returns Rating Ratings by user
     * @throws ApiError
     */
    public static getRatingsByUser(
        userId: number,
    ): CancelablePromise<Array<Rating>> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/ratings/user/{user_id}',
            path: {
                'user_id': userId,
            },
        });
    }
    /**
     * @param id Rating ID
     * @returns Rating Rating found
     * @throws ApiError
     */
    public static getRating(
        id: number,
    ): CancelablePromise<Rating> {
        return __request(OpenAPI, {
            method: 'GET',
            url: '/ratings/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Rating not found`,
            },
        });
    }
    /**
     * @param id Rating ID to update
     * @param requestBody
     * @returns Rating Rating updated
     * @throws ApiError
     */
    public static modifyRating(
        id: number,
        requestBody: CreateRating,
    ): CancelablePromise<Rating> {
        return __request(OpenAPI, {
            method: 'PUT',
            url: '/ratings/{id}',
            path: {
                'id': id,
            },
            body: requestBody,
            mediaType: 'application/json',
            errors: {
                404: `Rating not found`,
            },
        });
    }
    /**
     * @param id Rating ID to remove
     * @returns void
     * @throws ApiError
     */
    public static removeRating(
        id: number,
    ): CancelablePromise<void> {
        return __request(OpenAPI, {
            method: 'DELETE',
            url: '/ratings/{id}',
            path: {
                'id': id,
            },
            errors: {
                404: `Rating not found`,
            },
        });
    }
}
