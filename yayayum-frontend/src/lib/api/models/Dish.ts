/* generated using openapi-typescript-codegen -- do not edit */
/* istanbul ignore file */
/* tslint:disable */
/* eslint-disable */
import type { DietaryRestriction } from './DietaryRestriction';
import type { DishCategory } from './DishCategory';
export type Dish = {
    category: DishCategory;
    description: string;
    dietary_restrictions: Array<DietaryRestriction>;
    id: number;
    name: string;
    price_kr: number;
};

