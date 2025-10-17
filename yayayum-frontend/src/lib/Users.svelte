<script lang="ts">
  import { onMount } from "svelte";
  import { UsersService } from "./api/services/UsersService";
  import type { User } from "./api/models/User";
  import { selectedUser } from "./shared";
  import { currentScreen } from "./shared";
  import { Component } from "./types";

  let users: User[] = [];
  let loading = true;
  let error: string | null = null;
  let showAddForm = false;
  let newUsername = "";
  let isCreating = false;

  onMount(async () => {
    try {
      users = await UsersService.getUsers();
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  });

  function showAddUserForm() {
    showAddForm = true;
    newUsername = "";
  }

  function hideAddUserForm() {
    showAddForm = false;
    newUsername = "";
  }

  async function createUser() {
    if (!newUsername.trim()) {
      return;
    }

    isCreating = true;
    try {
      const newUser = await UsersService.createUser({ username: newUsername.trim() });
      users = [...users, newUser];
      hideAddUserForm();
    } catch (err) {
      error = `Kunde inte skapa användare: ${(err as Error).message}`;
    } finally {
      isCreating = false;
    }
  }

  function handleKeyPress(event: KeyboardEvent) {
    if (event.key === 'Enter') {
      createUser();
    } else if (event.key === 'Escape') {
      hideAddUserForm();
    }
  }
</script>

{#if loading}
  <p>Kontaktar firefly...</p>
{:else if error}
  <p style="color: red;">Error: {error}</p>
{:else if users.length === 0}
  <p>No users found.</p>
  <div style="display: flex; flex-wrap: wrap; gap: 8px;">
    <button
      class="add-user-btn"
      onclick={showAddUserForm}
    >
      + Lägg till användare
    </button>
  </div>
{:else}
  <div style="display: flex; flex-wrap: wrap; gap: 8px;">
    {#each users as user}
      <button
        style="
          padding: 6px 12px;
          border-radius: 12px;
          border: 1px solid #ccc;
          background-color: #f0f0f0;
          cursor: pointer;
          font-size: 0.9rem;
        "
        onclick={() => {
          selectedUser.update(() => user);
          currentScreen.update(() => Component.MealActions);
        }}
      >
        {user.username}
      </button>
    {/each}
    
    <!-- Add user button -->
    {#if !showAddForm}
      <button
        class="add-user-btn"
        onclick={showAddUserForm}
        title="Lägg till ny användare"
      >
        +
      </button>
    {/if}
  </div>
{/if}

<!-- Add user form -->
{#if showAddForm}
  <div class="add-user-form">
    <div class="form-content">
      <h3>Lägg till ny användare</h3>
      <input
        type="text"
        bind:value={newUsername}
        placeholder="Skriv användarnamn..."
        onkeydown={handleKeyPress}
        disabled={isCreating}
        class="username-input"
        autofocus
      />
      <div class="form-buttons">
        <button
          onclick={createUser}
          disabled={!newUsername.trim() || isCreating}
          class="create-btn"
        >
          {isCreating ? 'Skapar...' : 'Skapa'}
        </button>
        <button
          onclick={hideAddUserForm}
          disabled={isCreating}
          class="cancel-btn"
        >
          Avbryt
        </button>
      </div>
    </div>
  </div>
{/if}

<style>
  .add-user-btn {
    padding: 6px 12px;
    border-radius: 12px;
    border: 2px dashed #ff69b4;
    background-color: rgba(255, 105, 180, 0.1);
    color: #ff69b4;
    cursor: pointer;
    font-size: 0.9rem;
    font-weight: 600;
    transition: all 0.2s ease;
    min-width: 40px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .add-user-btn:hover {
    background-color: rgba(255, 105, 180, 0.2);
    transform: translateY(-1px);
    box-shadow: 0 2px 8px rgba(255, 105, 180, 0.3);
  }

  .add-user-btn:active {
    transform: translateY(0);
  }

  .add-user-form {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.5);
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
  }

  .form-content {
    background: white;
    padding: 2rem;
    border-radius: 16px;
    box-shadow: 0 8px 32px rgba(0, 0, 0, 0.2);
    min-width: 300px;
    max-width: 90vw;
  }

  .form-content h3 {
    margin: 0 0 1.5rem 0;
    color: #ff69b4;
    text-align: center;
    font-size: 1.2rem;
  }

  .username-input {
    width: 100%;
    padding: 0.8rem;
    border: 2px solid #ddd;
    border-radius: 8px;
    font-size: 1rem;
    margin-bottom: 1.5rem;
    box-sizing: border-box;
    transition: border-color 0.2s ease;
  }

  .username-input:focus {
    outline: none;
    border-color: #ff69b4;
    box-shadow: 0 0 0 3px rgba(255, 105, 180, 0.1);
  }

  .username-input:disabled {
    background-color: #f5f5f5;
    color: #666;
  }

  .form-buttons {
    display: flex;
    gap: 0.8rem;
    justify-content: flex-end;
  }

  .create-btn, .cancel-btn {
    padding: 0.6rem 1.2rem;
    border: none;
    border-radius: 8px;
    font-size: 0.9rem;
    font-weight: 600;
    cursor: pointer;
    transition: all 0.2s ease;
  }

  .create-btn {
    background: linear-gradient(135deg, #ff69b4, #ff8cc8);
    color: white;
    box-shadow: 0 2px 8px rgba(255, 105, 180, 0.3);
  }

  .create-btn:hover:not(:disabled) {
    transform: translateY(-1px);
    box-shadow: 0 4px 12px rgba(255, 105, 180, 0.4);
  }

  .create-btn:disabled {
    background: #ccc;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }

  .cancel-btn {
    background: #f0f0f0;
    color: #666;
    border: 1px solid #ddd;
  }

  .cancel-btn:hover:not(:disabled) {
    background: #e0e0e0;
    transform: translateY(-1px);
  }

  .cancel-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }
</style>
