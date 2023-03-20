import { render, waitFor } from '@testing-library/react';
import { MockedFunction } from 'vitest';

import SubscriptionList from './SubscriptionList';

const invokeMock = window.__TAURI_INVOKE__ as MockedFunction<
  typeof window.__TAURI_INVOKE__
>;

describe('SubscriptionList', () => {
  it('renders names of subscriptions', async () => {
    vi.setSystemTime('2023-03-16');
    invokeMock.mockResolvedValue([
      {
        name: 'Test Subscription',
        url: 'https://example.com',
      },
    ]);

    const { getByText } = render(<SubscriptionList />);

    await waitFor(() => {
      expect(getByText('Test Subscription')).toBeInTheDocument();
    });
  });
});
