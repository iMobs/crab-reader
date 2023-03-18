import { render, waitFor } from '@testing-library/react';
import { MockedFunction } from 'vitest';

import SubscriptionList from './SubscriptionList';

import { getSubscriptions } from '~/lib/bindings';

vi.mock('~/lib/bindings');

const getSubscriptionsMock = getSubscriptions as MockedFunction<
  typeof getSubscriptions
>;

describe('SubscriptionList', () => {
  it('renders names of subscriptions', async () => {
    vi.setSystemTime('2023-03-16');
    getSubscriptionsMock.mockResolvedValue([
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
