<script lang="ts">
    import { onMount } from "svelte";
    import { UsersService } from "./api/services/UsersService";
    import { DishesService } from "./api/services/DishesService";
    import type { User } from "./api/models/User";
    import { selectedUser } from "./shared";
    import { currentScreen } from "./shared";
    import { Component } from "./types";
    import type { CreateUser, Dish } from "./api";

    let users: User[] = [];
    let dishes: Dish[] = [];
    let loading = true;
    let error: string | null = null;
    let newUsername: string = "";

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

    function confirmAndDelete(userId: number) {
        if (confirm("Are you sure you want to delete this user?")) {
            UsersService.removeUser(userId);
            // Remove user from local list
            users = users.filter((user) => user.id !== userId);
        }
    }
</script>

{#if loading}
    <p>Loading users and dishes...</p>
{:else if error}
    <p style="color: red;">Error: {error}</p>
{:else if users.length === 0}
    <p>No users/dishes found.</p>
{:else}
    <h3>Add user</h3>
    <input type="text" placeholder="Username" bind:value={newUsername} />
    <button onclick={() => {
        UsersService.createUser({username: newUsername})
        }}>
        Add User
    </button>
    
    <h3>Remove user</h3>
    <div class="list">
        {#each users as user}
            <div class="list-item">
                <p>{user.id} - {user.username}</p>
                <button
                    class="remove-button"
                    onclick={() => {
                        confirmAndDelete(user.id);
                    }}
                >
                    X
                </button>
            </div>
        {/each}
    </div>

    <h3>Remove dish</h3>
    <div class="list">
        {#each dishes as dish}
            <div class="list-item">
                <p>{dish.id} - {dish.name}</p>
                <button
                    class="remove-button"
                    onclick={() => {
                        DishesService.removeDish(dish.id);
                        dishes = dishes.filter((d) => d.id !== dish.id);
                    }}
                >
                    X
                </button>
            </div>
        {/each}
    </div>
{/if}

<style>
    .list {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .list-item {
        display: flex;
        align-items: center;
    }

    .remove-button {
        margin-left: 8px;
        background-color: red;
        color: white;
        border: none;
        padding: 4px 8px;
        cursor: pointer;
    }

    .remove-button:hover {
        background-color: darkred;
    }
</style>
