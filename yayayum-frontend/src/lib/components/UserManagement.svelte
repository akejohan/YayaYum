<script lang="ts">
    import { UsersService } from "../api/services/UsersService";
    import type { User } from "../api/models/User";
    import { createEventDispatcher } from 'svelte';

    export let users: User[] = [];
    
    const dispatch = createEventDispatcher();
    
    let newUsername: string = "";
    let error: string | null = null;

    async function addUser() {
        if (!newUsername.trim()) {
            return;
        }
        
        try {
            const newUser = await UsersService.createUser({username: newUsername});
            // Dispatch event to parent to update the user list
            dispatch('userAdded', newUser);
            // Clear the input field
            newUsername = "";
            error = null;
        } catch (err) {
            error = (err as Error).message;
        }
    }

    function confirmAndDelete(userId: number) {
        const user = users.find(u => u.id === userId);
        const userName = user ? user.username : `User ${userId}`;
        
        if (confirm(`Are you sure you want to delete user "${userName}"?`)) {
            try {
                UsersService.removeUser(userId);
                // Dispatch event to parent to update the user list
                dispatch('userDeleted', userId);
                error = null;
            } catch (err) {
                error = (err as Error).message;
            }
        }
    }
</script>

<div class="user-management">
    <div class="section-header">
        <h4>Add New User</h4>
    </div>
    
    {#if error}
        <p class="error">{error}</p>
    {/if}
    <div class="add-user-form">
        <input type="text" placeholder="Username" bind:value={newUsername} />
        <button on:click={addUser}>
            Add User
        </button>
    </div>
    
    <div class="section-header">
        <h4>User List</h4>
    </div>
    <div class="user-list">
        {#each users as user}
            <div class="user-item">
                <span class="user-info">{user.id} - {user.username}</span>
                <button
                    class="remove-button"
                    on:click={() => confirmAndDelete(user.id)}
                >
                    Delete
                </button>
            </div>
        {/each}
        {#if users.length === 0}
            <p class="no-items">No users found.</p>
        {/if}
    </div>
</div>

<style>
    .user-management {
        margin: 0;
    }

    .section-header {
        margin-bottom: 16px;
        padding-bottom: 8px;
        border-bottom: 2px solid #e2e8f0;
    }

    .section-header h4 {
        margin: 0;
        color: #4a5568;
        font-size: 1.1em;
        font-weight: 600;
    }

    .add-user-form {
        display: flex;
        gap: 8px;
        margin-bottom: 24px;
        max-width: 300px;
    }

    .add-user-form input {
        flex: 1;
        padding: 8px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .add-user-form button {
        padding: 8px 16px;
        background-color: #007acc;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
    }

    .add-user-form button:hover {
        background-color: #005a9e;
    }

    .user-list {
        display: flex;
        flex-direction: column;
        gap: 8px;
    }

    .user-item {
        display: flex;
        align-items: center;
        justify-content: space-between;
        padding: 12px;
        border: 1px solid #ddd;
        border-radius: 4px;
        background-color: white;
    }

    .user-info {
        font-weight: 500;
    }

    .remove-button {
        padding: 6px 12px;
        background-color: #dc3545;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 0.9em;
    }

    .remove-button:hover {
        background-color: #c82333;
    }

    .error {
        color: #dc3545;
        font-size: 0.9em;
        margin-bottom: 8px;
    }

    .no-items {
        color: #666;
        font-style: italic;
        text-align: center;
        padding: 20px;
    }
</style>