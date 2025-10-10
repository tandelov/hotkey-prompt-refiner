<script>
  import { page } from '$app/stores';

  const navItems = [
    {
      path: '/settings',
      icon: '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M12.22 2h-.44a2 2 0 0 0-2 2v.18a2 2 0 0 1-1 1.73l-.43.25a2 2 0 0 1-2 0l-.15-.08a2 2 0 0 0-2.73.73l-.22.38a2 2 0 0 0 .73 2.73l.15.1a2 2 0 0 1 1 1.72v.51a2 2 0 0 1-1 1.74l-.15.09a2 2 0 0 0-.73 2.73l.22.38a2 2 0 0 0 2.73.73l.15-.08a2 2 0 0 1 2 0l.43.25a2 2 0 0 1 1 1.73V20a2 2 0 0 0 2 2h.44a2 2 0 0 0 2-2v-.18a2 2 0 0 1 1-1.73l.43-.25a2 2 0 0 1 2 0l.15.08a2 2 0 0 0 2.73-.73l.22-.39a2 2 0 0 0-.73-2.73l-.15-.08a2 2 0 0 1-1-1.74v-.5a2 2 0 0 1 1-1.74l.15-.09a2 2 0 0 0 .73-2.73l-.22-.38a2 2 0 0 0-2.73-.73l-.15.08a2 2 0 0 1-2 0l-.43-.25a2 2 0 0 1-1-1.73V4a2 2 0 0 0-2-2z"/><circle cx="12" cy="12" r="3"/></svg>',
      label: 'Settings'
    },
    {
      path: '/history',
      icon: '<svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><circle cx="12" cy="12" r="10"/><polyline points="12 6 12 12 16 14"/></svg>',
      label: 'History'
    },
  ];
</script>

<aside class="sidebar">
  <nav class="sidebar-nav">
    {#each navItems as item}
      <a
        href={item.path}
        class="nav-item"
        class:active={$page.url.pathname === item.path}
      >
        <span class="nav-icon">{@html item.icon}</span>
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

  .sidebar-nav {
    padding: 12px 8px 8px;
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
    background: rgba(255, 255, 255, 0.1);
    color: white;
    font-weight: 500;
  }

  .nav-icon {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    margin-right: 12px;
    color: #A0A0A0;
  }

  .nav-icon :global(svg) {
    width: 16px;
    height: 16px;
  }

  .nav-item.active .nav-icon {
    color: white;
  }

  .nav-label {
    flex: 1;
  }

  @media (prefers-color-scheme: dark) {
    .sidebar {
      background: rgba(30, 30, 30, 0.8);
      border-right-color: rgba(255, 255, 255, 0.1);
    }

    .nav-item {
      color: #f5f5f7;
    }

    .nav-item:hover {
      background: rgba(255, 255, 255, 0.08);
    }

    .nav-item.active {
      background: rgba(255, 255, 255, 0.1);
      color: white;
    }

    .nav-item.active .nav-icon {
      color: white;
    }
  }
</style>
