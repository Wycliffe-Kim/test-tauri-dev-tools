/*
 * Copyright (C) 2024 nota inc. All rights reserved.
 * This source code is the property of nota inc. and is protected by copyright law. THE RECEIPT OR POSSESSION OF  THIS SOURCE CODE AND/OR RELATED INFORMATION DOES NOT CONVEY OR IMPLY TRANSFER OF OWNERSHIP, COPYRIGHT OR ANY RIGHTS INCLUDING BUT NOT LIMITED TO REPRODUCE, DISCLOSE OR DISTRIBUTE ITS CONTENTS, OR TO MANUFACTURE, USE, OR SELL ANYTHING THAT IT  MAY DESCRIBE, IN WHOLE OR IN PART.
 * Unauthorized disclosure, reproduction, modification, distribution, or commercial use of all or any part of this source code without prior written consent from Nota Inc. is strictly prohibited.
 * This source code is provided "as is" and without any warranty, express or implied.
 * Nota Inc. shall not be liable for any damages arising from the use of this source code.
 * For inquiries regarding the use of this source code, please contact Nota Inc. at:
 * Email: contact@nota.ai
 */
import { useState } from 'react';
import reactLogo from './assets/react.svg';
import { invoke } from '@tauri-apps/api';
import './App.css';
import { useCommandWithKeyboardEvent } from './hooks/useCommandWithKeyboardEvent';
import { openAppDataDir } from './tauri-commands/openAppDataDir';
import { openDevtools } from './tauri-commands/openDevtools';

function App() {
  const [message, setMessage] = useState('');
  const [rtspSrc, setRtspSrc] = useState(
    'rtsp://210.99.70.120:1935/live/cctv050.stream'
  );

  async function run() {
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    setMessage(await invoke('run', { rtspSrc }));
  }

  useCommandWithKeyboardEvent({
    command: openAppDataDir,
    name: 'openAppDataDir',
    windows: {
      ctrl: true,
      shift: true,
      key: 'd',
    },
    macos: {
      meta: true,
      shift: true,
      key: 'd',
    },
  });
  useCommandWithKeyboardEvent({
    command: openDevtools,
    name: 'openDevtools',
    windows: { ctrl: true, shift: true, key: 'i' },
    macos: {
      meta: true,
      shift: true,
      key: 'i',
    },
  });

  return (
    <main className='container'>
      <h1>Welcome to Tauri + React</h1>

      <div className='row'>
        <a href='https://vitejs.dev' target='_blank'>
          <img src='/vite.svg' className='logo vite' alt='Vite logo' />
        </a>
        <a href='https://tauri.app' target='_blank'>
          <img src='/tauri.svg' className='logo tauri' alt='Tauri logo' />
        </a>
        <a href='https://reactjs.org' target='_blank'>
          <img src={reactLogo} className='logo react' alt='React logo' />
        </a>
      </div>
      <p>Click on the Tauri, Vite, and React logos to learn more.</p>

      <form
        className='row'
        onSubmit={(e) => {
          e.preventDefault();
          run();
        }}
      >
        <input
          id='greet-input'
          onChange={(e) => setRtspSrc(e.currentTarget.value)}
          placeholder='Enter a name...'
        />
        <button type='submit'>Greet</button>
      </form>
      <p>{message}</p>
    </main>
  );
}

export default App;
