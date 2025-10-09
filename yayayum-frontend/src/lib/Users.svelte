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

  onMount(async () => {
    try {
      users = await UsersService.getUsers();
    } catch (err) {
      error = (err as Error).message;
    } finally {
      loading = false;
    }
  });
</script>

{#if loading}
  <p>Loading users...</p>
{:else if error}
  <p style="color: red;">Error: {error}</p>
{:else if users.length === 0}
  <p>No users found.</p>
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
  </div>
{/if}
