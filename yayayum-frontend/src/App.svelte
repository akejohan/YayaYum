<script lang="ts">
  import yayaLogo from "./assets/yayalogo.png";
  import type { User } from "./lib/api";
  import Users from "./lib/Users.svelte";
  import { AppScreen } from "./lib/types";
  import MealActions from "./lib/MealActions.svelte";

  let selectedUser: User | null = null;
  let currentScreen: AppScreen = AppScreen.UserSelection;

  function handleUserClick(user: User) {
    selectedUser = user;
    currentScreen = AppScreen.MealActions;
  }
</script>

<main>
  <div>
    <span>
      <img src={yayaLogo} class="logo" alt="Yayalogo" />
    </span>
  </div>
  <h1>YayaYum</h1>

  <div class="card">
    {#if currentScreen === AppScreen.MealActions && selectedUser}
      <MealActions {selectedUser} />
    {:else if currentScreen === AppScreen.MealInspiration}
    <p>Meal Inspiration Screen (to be implemented)</p>
    {:else if currentScreen === AppScreen.UserSelection}
    <Users onUserClick={handleUserClick} />
    {/if}
  </div>

  {#if selectedUser}
    <p>Idag är du: {selectedUser.username}</p>
  {:else}
    <p>Vem vill du vara idag?</p>

    <p class="click-your-name">Klicka på ditt namn</p>
  {/if}
</main>

<style>
  main {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: flex-start;
    padding: 1em;
  }

  .card {
    background-color: rgba(255, 255, 255, 0.8); /* semi-transparent card */
    border-radius: 16px;
    padding: 1em;
    margin-top: 1em;
    box-shadow: 0 8px 16px rgba(0, 0, 0, 0.1);
  }

  h1 {
    color: #ff69b4;
    text-shadow: 1px 1px 2px #fff;
  }
  .logo {
    height: 6em;
    padding: 1.5em;
    will-change: filter;
    transition: filter 300ms;
  }
  .logo:hover {
    filter: drop-shadow(0 0 2em #646cffaa);
  }
  .click-your-name {
    color: #888;
  }
</style>
