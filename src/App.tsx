import { invoke } from '@tauri-apps/api/tauri';
import { useEffect, useState } from 'react';
import { debug } from 'tauri-plugin-log-api';

import reactLogo from './assets/react.svg';
import './App.css';

function App() {
  const [urls, setFeeds] = useState<string[]>([]);
  const [name, setName] = useState('');

  useEffect(() => {
    debug('Hello from the frontend!');
  }, []);

  useEffect(() => {
    invoke('get_feeds', { urls }).then((feeds) => console.log(feeds));
  }, [urls]);

  return (
    <div className="container">
      <h1>Welcome to Tauri!</h1>

      <div className="row">
        <a href="https://vitejs.dev" target="_blank" rel="noreferrer">
          <img src="/vite.svg" className="logo vite" alt="Vite logo" />
        </a>
        <a href="https://tauri.app" target="_blank" rel="noreferrer">
          <img src="/tauri.svg" className="logo tauri" alt="Tauri logo" />
        </a>
        <a href="https://reactjs.org" target="_blank" rel="noreferrer">
          <img src={reactLogo} className="logo react" alt="React logo" />
        </a>
      </div>

      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <div className="row">
        <form
          onSubmit={(e) => {
            e.preventDefault();
            setFeeds([...urls, name]);
          }}
        >
          <input
            id="greet-input"
            onChange={(e) => setName(e.currentTarget.value)}
            placeholder="Enter a url..."
          />
          <button type="submit">Add</button>
        </form>
      </div>
    </div>
  );
}

export default App;
