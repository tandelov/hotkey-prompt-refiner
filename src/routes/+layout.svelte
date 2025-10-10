<script>
  import Sidebar from '$lib/components/Sidebar.svelte';
</script>

<div class="titlebar" data-tauri-drag-region></div>
<div class="app-layout">
  <Sidebar />
  <main class="main-content">
    <slot />
  </main>
</div>

<style>
  :global(body) {
    margin: 0;
    padding: 0;
    font-family: -apple-system, BlinkMacSystemFont, "SF Pro Text", "SF Pro Display", "Helvetica Neue", Arial, sans-serif;
    font-size: 13px;
    line-height: 1.5;
    color: #1d1d1f;
    background: #ffffff;
    overflow: hidden;
    -webkit-font-smoothing: antialiased;
    -moz-osx-font-smoothing: grayscale;
  }

  :global(*) {
    box-sizing: border-box;
  }

  :root {
    --titlebar-height: 28px;
  }

  .titlebar {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    height: var(--titlebar-height);
    z-index: 1000;
    -webkit-app-region: drag;
    user-select: none;
  }

  .app-layout {
    display: flex;
    height: 100vh;
    width: 100vw;
    overflow: hidden;
    padding-top: var(--titlebar-height);
  }

  .main-content {
    flex: 1;
    overflow: hidden;
    background: #ffffff;
    display: flex;
    flex-direction: column;
  }

  @media (prefers-color-scheme: dark) {
    :global(body) {
      color: #f5f5f7;
      background: #1c1c1e;
    }

    .main-content {
      background: #1c1c1e;
    }
  }
</style>
