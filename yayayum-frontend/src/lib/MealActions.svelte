<script lang="ts">
    import type { User } from "./api/models/User";
    let { selectedUser }: { selectedUser: User } = $props();
</script>

<h2>Välkommen, {selectedUser.username}!</h2>