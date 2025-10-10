<script>
  import { page } from '$app/stores';

  const navItems = [
    { path: '/settings', icon: '⚙️', label: 'Settings' },
    { path: '/history', icon: '📊', label: 'History' },
  ];
</script>

<aside class="sidebar">
  <div class="sidebar-header">
    <div class="app-icon">⌨️</div>
    <div class="app-name">Hotkey Prompt Refiner</div>
  </div>

  <nav class="sidebar-nav">
    {#each navItems as item}
      <a
        href={item.path}
        class="nav-item"
        class:active={$page.url.pathname === item.path}
      >
        <span class="nav-icon">{item.icon}</span>
        <span class="nav-label">{item.label}</span>
      </a>
    {/each}
  </nav>
</aside>

<style>
  .sidebar {
    width: 200px;
    height: 100vh;
    background: rgba(242, 242, 247, 0.8);
    backdrop-filter: blur(20px);
    -webkit-backdrop-filter: blur(20px);
    border-right: 1px solid rgba(0, 0, 0, 0.1);
    display: flex;
    flex-direction: column;
    flex-shrink: 0;
  }

  .sidebar-header {
    padding: 20px 16px;
    border-bottom: 1px solid rgba(0, 0, 0, 0.08);
  }

  .app-icon {
    font-size: 32px;
    margin-bottom: 8px;
    text-align: center;
  }

  .app-name {
    font-size: 11px;
    font-weight: 600;
    text-align: center;
    color: #1d1d1f;
    line-height: 1.3;
  }

  .sidebar-nav {
    padding: 8px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .nav-item {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 8px 12px;
    border-radius: 6px;
    text-decoration: none;
    color: #1d1d1f;
    font-size: 13px;
    transition: background-color 0.15s ease;
    cursor: pointer;
  }

  .nav-item:hover {
    background: rgba(0, 0, 0, 0.05);
  }

  .nav-item.active {
    background: rgba(0, 122, 255, 0.15);
    color: #007aff;
    font-weight: 500;
  }

  .nav-icon {
    font-size: 16px;
    width: 20px;
    text-align: center;
  }

  .nav-label {
    flex: 1;
  }

  @media (prefers-color-scheme: dark) {
    .sidebar {
      background: rgba(40, 40, 43, 0.9);
      border-right-color: rgba(255, 255, 255, 0.1);
    }

    .sidebar-header {
      border-bottom-color: rgba(255, 255, 255, 0.08);
    }

    .app-name {
      color: #f5f5f7;
    }

    .nav-item {
      color: #f5f5f7;
    }

    .nav-item:hover {
      background: rgba(255, 255, 255, 0.08);
    }

    .nav-item.active {
      background: rgba(10, 132, 255, 0.25);
      color: #0a84ff;
    }
  }
</style>
