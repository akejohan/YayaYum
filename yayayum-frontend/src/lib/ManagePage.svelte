<script lang="ts">
    import { onMount } from "svelte";
    import { UsersService } from "./api/services/UsersService";
    import { DishesService } from "./api/services/DishesService";
    import type { User } from "./api/models/User";
    import type { Dish } from "./api/models/Dish";
    import UserManagement from "./components/UserManagement.svelte";
    import DishManagement from "./components/DishManagement.svelte";
    import CollapsibleSection from "./components/CollapsibleSection.svelte";

    let users: User[] = [];
    let dishes: Dish[] = [];
    let loading = true;
    let error: string | null = null;
    
    // Collapsible section states
    let usersOpen = true;
    let dishesOpen = true;

    onMount(async () => {
        try {
            users = await UsersService.getUsers();
            dishes = await DishesService.getDishes();
        } catch (err) {
            error = (err as Error).message;
        } finally {
            loading = false;
        }
    });

    // Event handlers for user management
    function handleUserAdded(event: CustomEvent<User>) {
        users = [...users, event.detail];
    }

    function handleUserDeleted(event: CustomEvent<number>) {
        users = users.filter(user => user.id !== event.detail);
    }

    // Event handlers for dish management
    function handleDishAdded(event: CustomEvent<Dish>) {
        dishes = [...dishes, event.detail];
    }

    function handleDishUpdated(event: CustomEvent<Dish>) {
        dishes = dishes.map(dish => dish.id === event.detail.id ? event.detail : dish);
    }

    function handleDishDeleted(event: CustomEvent<number>) {
        dishes = dishes.filter(dish => dish.id !== event.detail);
    }
</script>

{#if loading}
    <div class="loading">
        <p>Ringer johan...</p>
    </div>
{:else if error}
    <div class="error-container">
        <p class="error">Error: {error}</p>
    </div>
{:else}
    <div class="manage-page">
        <h2>Management Dashboard</h2>
        
        <CollapsibleSection 
            title="User Management" 
            icon="👥" 
            bind:isOpen={usersOpen}
        >
            <svelte:fragment slot="count">
                {users.length} {users.length === 1 ? 'user' : 'users'}
            </svelte:fragment>
            
            <UserManagement 
                {users}
                on:userAdded={handleUserAdded}
                on:userDeleted={handleUserDeleted}
            />
        </CollapsibleSection>
        
        <CollapsibleSection 
            title="Dish Management" 
            icon="🍽️" 
            bind:isOpen={dishesOpen}
        >
            <svelte:fragment slot="count">
                {dishes.length} {dishes.length === 1 ? 'dish' : 'dishes'}
            </svelte:fragment>
            
            <DishManagement 
                {dishes}
                on:dishAdded={handleDishAdded}
                on:dishUpdated={handleDishUpdated}
                on:dishDeleted={handleDishDeleted}
            />
        </CollapsibleSection>
    </div>
{/if}

<style>
    .loading {
        display: flex;
        justify-content: center;
        align-items: center;
        min-height: 200px;
        font-size: 1.1em;
        color: #666;
    }

    .error-container {
        display: flex;
        justify-content: center;
        margin: 20px 0;
    }

    .error {
        color: #dc3545;
        font-weight: 500;
        padding: 12px 20px;
        background-color: #f8d7da;
        border: 1px solid #f5c6cb;
        border-radius: 8px;
    }

    .manage-page {
        max-width: 900px;
        margin: 0 auto;
        min-height: 100vh;
    }

    .manage-page h2 {
        text-align: center;
        color: #2d3748;
        margin-bottom: 40px;
        font-size: 2.5em;
        font-weight: 700;
        text-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        -webkit-background-clip: text;
        -webkit-text-fill-color: transparent;
        background-clip: text;
    }
</style>
