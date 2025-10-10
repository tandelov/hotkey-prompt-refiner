<script>
  import { goto } from '$app/navigation';
  import { onMount } from 'svelte';

  onMount(() => {
    // Redirect to settings page on app launch
    goto('/settings');
  });
</script>

<div class="loading">
  <p>Loading...</p>
</div>

<style>
  .loading {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100vh;
    font-size: 13px;
    color: #86868b;
  }
</style>
