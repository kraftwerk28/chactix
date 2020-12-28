<script>
  let username = '';
  while (!username) {
    username = (prompt('Username') ?? '').trim();
  }

  let ws = new WebSocket('ws://127.0.0.1:8080');
  let users = [];
  let messages = [];
  let messageInput = '';

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

  ws.addEventListener('open', () => {
    send('username', username);
    send('getusers');

    ws.addEventListener('message', (evt) => {
      const { type, data } = JSON.parse(evt.data);
      console.log(type, data);
      switch (type) {
        case 'msg': {
          const [uid, text] = data;
          const user = users.find(u => u.id === uid);
          messages = [...messages, { username, text }];
          break;
        }
        case 'join': {
          users = [...users, data];
          break;
        }
        case 'userlist':
          users = data.slice();
          break;
        default:
          break;
      }
    });
  });
</script>

<main>
  <ul>
    {#each messages as message}
      <li>
        <span>{message.username}:</span>
        <span>{message.text}</span>
      </li>
    {/each}
  </ul>
  <form on:submit|preventDefault={onSubmit}>
    <input type="text" bind:value={messageInput} />
  </form>
</main>

<style>
  :global(body) {
    background: #444;
    color: white;
  }
  main {
    text-align: center;
    padding: 1em;
    max-width: 240px;
    margin: 0 auto;
    border: 1px solid white;
  }
  main ul {
    list-style-type: none;
    text-align: initial;
    padding: 0;
  }
  main li {
    display: flex;
  }
  main li span:first-child {
    flex: 1;
  }
  main li span:nth-child(2) {
    flex: 3;
  }
</style>
