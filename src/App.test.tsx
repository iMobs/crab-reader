import { render } from '@testing-library/react';

import App from './App';

describe('App', () => {
  it('should show links', () => {
    const { getByRole } = render(<App />);

    expect(getByRole('link', { name: 'Vite logo' })).toBeInTheDocument();
    expect(getByRole('link', { name: 'Tauri logo' })).toBeInTheDocument();
    expect(getByRole('link', { name: 'React logo' })).toBeInTheDocument();
  });
});
