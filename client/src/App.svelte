<script>
  import { onMount } from 'svelte';
  let ws;

  onMount(() => {
    ws = new WebSocket(`ws://${location.host}`);
    let username = '';
    while (!username) {
      username = (prompt('Username') ?? '').trim();
    }
    ws.addEventListener('open', () => {
      send('username', username);
      ws.addEventListener('message', onMessage);
    });
  });

  let users = [];
  let messages = [];
  let messageInput = '';
  let messageElem;

  function send(type, data) {
    const p = { type };
    if (data !== undefined) {
      Object.assign(p, { data });
    }
    ws.send(JSON.stringify(p));
  }

  function onSubmit() {
    let raw = messageInput.trim();
    if (!raw) return;
    send('text', raw);
    messageInput = '';
  }

  function onMessage(evt) {
    const { type, data } = JSON.parse(evt.data);
    switch (type) {
      case 'msg': {
        messages = [...messages, data];
        messageElem.scrollBy(0, 500);
        break;
      }
      case 'join': {
        users = [...users, data];
        break;
      }
      case 'restore':
        users = data.users;
        messages = data.messages;
      case 'leave': {
        const idx = users.findIndex(u => u.id === data);
        if (idx >= 0) {
          users = users.slice(0, idx).concat(users.slice(idx + 1));
        }
        break;
      }
      default:
        break;
    }
  }
</script>

<style>
  :global(body) {
    background: #444;
    color: white;
    font-family: 'JetBrains Mono', monospace;
    font-size: 1.2em;
  }

  main {
    text-align: center;
    padding: 0.5em;
    margin: 0 auto;
    border: 1px solid white;
    display: grid;
    grid-template-areas: 'a a b'
                         'a a b'
                         'c c c';
    grid-template-rows: 1fr auto;
    grid-gap: 0.5em;
    height: 50vh;
    width: 75vw;
  }

  .messages ul {
    list-style-type: none;
    text-align: initial;
    padding: 0;
  }
  .messages li {
    display: flex;
  }
  .messages li span:first-child {
    flex: 1;
  }
  .messages li span:nth-child(2) {
    flex: 3;
  }

  .messages {
    grid-area: a;
    overflow: auto;
  }
  .users {
    grid-area: b;
  }
  .users ul {
    list-style-type: decimal;
    text-align: initial;
    padding: 0;
    padding-left: 2em;
  }
  .send {
    grid-area: c;
    margin: 2px;
  }
  .send input {
    color: white;
    background: transparent;
    width: 100%;
  }
</style>

<main>

  <fieldset bind:this={messageElem} class="messages">
    <legend>Messages</legend>
    <ul>
      {#each messages as message}
        <li>
          <span>
            {users.find(user => user.id === message.user_id)?.username ??
              '*ghost*'
            }:
          </span>
          <span>{message.text}</span>
        </li>
      {/each}
    </ul>
  </fieldset>

  <fieldset class="users">
    <legend>Users</legend>
    <ul>
      {#each users as user}
        <li>
          <span>{user.username}</span>
        </li>
      {/each}
    </ul>
  </fieldset>

  <form class="send" on:submit|preventDefault={onSubmit}>
    <input type="text" placeholder="write here..." bind:value={messageInput} />
  </form>
</main>
