import { render } from '@testing-library/react';

import App from './App';

describe('App', () => {
  // TODO: this is a pretty basic test that will break when any effects are added
  it('renders', () => {
    render(<App />);
  });
});
