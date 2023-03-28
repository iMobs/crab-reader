import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MockedFunction } from 'vitest';

import SubscriptionList from './SubscriptionList';

const invokeMock = window.__TAURI_INVOKE__ as MockedFunction<
  typeof window.__TAURI_INVOKE__
>;

describe('SubscriptionList', () => {
  it('renders names of subscriptions', async () => {
    vi.setSystemTime('2023-03-16');
    invokeMock.mockResolvedValue([
      { name: 'Test Subscription', url: 'https://example.com' },
    ]);

    const { getByText } = render(<SubscriptionList />);

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith('get_subscriptions');
      expect(getByText('Test Subscription')).toBeInTheDocument();
    });
  });

  it('renders options to remove subscription', async () => {
    invokeMock.mockResolvedValue([
      { name: 'Test Subscription', url: 'https://example.com' },
    ]);

    const user = userEvent.setup();
    const { findByRole, getByRole, getByText } = render(<SubscriptionList />);

    await waitFor(() => {
      expect(invokeMock).toHaveBeenCalledWith('get_subscriptions');
      expect(getByText('Test Subscription')).toBeInTheDocument();
    });

    const optionsButton = await findByRole('button', { name: /options/i });
    await user.click(optionsButton);

    const removeButton = getByRole('menuitem', { name: /remove/i });
    await user.click(removeButton);

    expect(invokeMock).toHaveBeenCalledWith('remove_subscription', {
      name: 'Test Subscription',
    });
  });
});
