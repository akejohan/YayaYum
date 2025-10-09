<script>
  import ManagePage from "./lib/ManagePage.svelte";
  import MainPage from "./lib/pages/MainPage.svelte";
  import { currentPage } from "./lib/shared";
  import { Page } from "./lib/types";

  const params = new URLSearchParams(window.location.search);
  let param = params.get("view");
  if (param === "manage") {
    currentPage.set(Page.Manage);
  }
</script>

{#if $currentPage == Page.Manage}
  <ManagePage />
{:else}
  <MainPage />
{/if}
