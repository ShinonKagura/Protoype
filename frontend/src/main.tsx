import * as React from 'react'
import * as ReactDOM from 'react-dom/client'
import App from './App'
import { theme } from './theme'
import { MantineProvider } from '@mantine/core';
import '@mantine/core/styles.css';

function Root() {
  return (
    <MantineProvider theme={{ 
      ...theme, 
      primaryColor: 'blue',
      defaultRadius: 'sm',
    }}>
      <App />
    </MantineProvider>
  );
}

ReactDOM.createRoot(document.getElementById('root') as HTMLElement).render(
  <React.StrictMode>
    <Root />
  </React.StrictMode>,
)
