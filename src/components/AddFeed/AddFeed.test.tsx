import { invoke } from '@tauri-apps/api';
import { render } from '@testing-library/react';
import userEvent from '@testing-library/user-event';

import AddFeed from './AddFeed';

const ResizeObserverMock = vi.fn(() => ({
  observe: vi.fn(),
  unobserve: vi.fn(),
  disconnect: vi.fn(),
}));

vi.stubGlobal('ResizeObserver', ResizeObserverMock);
vi.mock('@tauri-apps/api');

describe('AddFeed', () => {
  it('opens a dialog when the button is pressed', async () => {
    const user = userEvent.setup();
    const { getByRole, queryByRole } = render(<AddFeed />);

    // form title should not be visible
    expect(
      queryByRole('heading', { name: /add a feed/i }),
    ).not.toBeInTheDocument();

    // open form
    await user.click(getByRole('button', { name: /add feed/i }));

    // form title should be visible
    expect(queryByRole('heading', { name: /add a feed/i })).toBeInTheDocument();
  });

  it('validates the url', async () => {
    const user = userEvent.setup();
    const { getByRole, getByText } = render(<AddFeed />);

    // open form
    await user.click(getByRole('button', { name: /add feed/i }));

    // type invalid url
    await user.type(getByRole('textbox', { name: /feed url/i }), 'invalid url');

    // submit
    await user.click(getByRole('button', { name: /add/i }));

    // show form error
    expect(getByText(/invalid url/i)).toBeInTheDocument();
  });

  it('submits the url via ipc command', async () => {
    const user = userEvent.setup();
    const { getByRole } = render(<AddFeed />);

    // open form
    await user.click(getByRole('button', { name: /add feed/i }));

    // type invalid url
    await user.type(
      getByRole('textbox', { name: /feed url/i }),
      'https://example.com/feed.xml',
    );

    // submit
    await user.click(getByRole('button', { name: /add/i }));

    expect(invoke).toHaveBeenCalledWith('add_feed', {
      url: 'https://example.com/feed.xml',
    });
  });
});
