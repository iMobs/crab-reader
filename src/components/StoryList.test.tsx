import { render, waitFor } from '@testing-library/react';
import { MockedFunction } from 'vitest';

import StoryList from './StoryList';

import { getStories } from '~/lib/bindings';

vi.mock('~/lib/bindings');

const getStoriesMock = getStories as MockedFunction<typeof getStories>;

describe('StoryList', () => {
  it('renders titles and relative dates of stories', async () => {
    vi.setSystemTime('2023-03-16');
    getStoriesMock.mockResolvedValue([
      {
        title: 'Test Story',
        description: 'This is a test',
        link: 'https://example.com',
        pub_date: '2023-03-15',
      },
    ]);

    const { getByText } = render(<StoryList />);

    await waitFor(() => {
      expect(getByText('Test Story')).toBeInTheDocument();
      expect(getByText('1 day ago')).toBeInTheDocument();
    });
  });
});
