import { StrictMode } from 'react';
import { createRoot } from 'react-dom/client';

import App from '~/components/App';
import './styles.scss';

createRoot(document.getElementById('root') as HTMLElement).render(
  <StrictMode>
    <App />
  </StrictMode>,
);
