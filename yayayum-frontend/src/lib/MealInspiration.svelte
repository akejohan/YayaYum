<script lang="ts">
    import { onMount } from "svelte";
    import { DishesService, type Dish, DishCategory, RatingsService, type Rating } from "./api";
    import DishListItem from "./DishListItem.svelte";
    import { selectedUser } from "./shared";

    let dishes: Dish[] = [];
    let userRatings: Rating[] = [];
    let loading = true;
    let error: string | null = null;

    // Group dishes by category
    $: dishesByCategory = dishes.reduce((groups, dish) => {
        const category = dish.category;
        if (!groups[category]) {
            groups[category] = [];
        }
        groups[category].push(dish);
        return groups;
    }, {} as Record<string, Dish[]>);

    // Get category display names
    function getCategoryDisplayName(category: string): string {
        switch (category) {
            case DishCategory.WOK_WITH_NOODLES: return "Wok with Noodles";
            case DishCategory.WOK_WITH_RICE: return "Wok with Rice";
            case DishCategory.RAMEN: return "Ramen";
            case DishCategory.SPECIAL_DISH: return "Special Dishes";
            case DishCategory.STEW: return "Stews";
            case DishCategory.KIDS_MENU: return "Kids Menu";
            case DishCategory.SIDE_ORDER: return "Side Orders";
            default: return category;
        }
    }

    // Get user rating for a specific dish
    function getDishRating(dishId: number): number {
        const rating = userRatings.find(r => r.dish_id === dishId);
        return rating ? rating.rating : 0;
    }

    // Get how many times user has eaten a specific dish
    function getDishEatenCount(dishId: number): number {
        return userRatings.filter(r => r.dish_id === dishId).length;
    }

    async function fetchData() {
        try {
            loading = true;
            dishes = await DishesService.getDishes();
            
            // Fetch user ratings if user is selected
            if ($selectedUser) {
                try {
                    userRatings = await RatingsService.getRatingsByUser($selectedUser.id);
                } catch (ratingsErr) {
                    // If ratings fail, just continue without them
                    console.warn('Could not load user ratings:', ratingsErr);
                    userRatings = [];
                }
            }
        } catch (err) {
            error = (err as Error).message;
        } finally {
            loading = false;
        }
    }

    onMount(fetchData);

    // Refetch ratings when user changes
    $: if ($selectedUser) {
        fetchData();
    }
</script>

{#if loading}
    <p>Ringer yayamee...</p>
{:else if error}
    <p style="color: red;">Error: {error}</p>
{:else if dishes.length === 0}
    <p>No meals found.</p>
{:else}
    <div class="dish-list">
        {#each Object.entries(dishesByCategory) as [category, categoryDishes] (category)}
            <div class="category-section">
                <h2 class="category-heading">{getCategoryDisplayName(category)}</h2>
                <div class="category-dishes">
                    {#each categoryDishes as dish (dish.id)}
                        <DishListItem 
                            {dish} 
                            rating={getDishRating(dish.id)} 
                            eatenCount={getDishEatenCount(dish.id)}
                        />
                    {/each}
                </div>
            </div>
        {/each}
        
        {#if Object.keys(dishesByCategory).length === 0}
            <p>Inga rätter tillgängliga just nu 🍶</p>
        {/if}
    </div>
{/if}

<style>
    .dish-list {
        max-width: 700px;
        margin: 0 auto;
        padding: 2rem 1rem;
    }

    .category-section {
        margin-bottom: 2.5rem;
    }

    .category-heading {
        font-size: 1.5rem;
        font-weight: 700;
        color: #ff7e6b;
        margin: 0 0 1rem 0;
        padding: 0.5rem 0;
        border-bottom: 3px solid #ff7e6b;
        text-align: center;
        position: relative;
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }

    .category-heading:after {
        content: '';
        position: absolute;
        bottom: -3px;
        left: 50%;
        transform: translateX(-50%);
        width: 60px;
        height: 3px;
        background: linear-gradient(135deg, #ff7e6b, #ff9985);
        border-radius: 2px;
    }

    .category-dishes {
        display: flex;
        flex-direction: column;
        gap: 0.5rem;
    }

    /* Mobile responsive */
    @media (max-width: 500px) {
        .category-heading {
            font-size: 1.3rem;
        }
        
        .dish-list {
            padding: 1rem 0.5rem;
            min-width: 17em;
        }
    }
</style>
