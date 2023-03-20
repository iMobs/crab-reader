import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MockedFunction } from 'vitest';

import StoryList from './StoryList';

import { getStories } from '~/lib/bindings';

const setStoryMock = vi.fn();
vi.mock('~/contexts/currentStory', () => ({
  useCurrentStory: vi.fn(() => ({ setStory: setStoryMock })),
}));
vi.mock('~/lib/bindings');

const getStoriesMock = getStories as MockedFunction<typeof getStories>;

describe('StoryList', () => {
  it('renders titles and relative dates of stories', async () => {
    vi.setSystemTime('2023-03-16');
    getStoriesMock.mockResolvedValue([
      {
        title: 'Test Story',
        content: 'This is a test',
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

  it('sets the current story on click', async () => {
    const story = {
      title: 'Test Story',
      content: 'This is a test',
      link: 'https://example.com',
      pub_date: '2023-03-15',
    };
    getStoriesMock.mockResolvedValue([story]);

    const user = userEvent.setup();
    const { findByText } = render(<StoryList />);
    await user.click(await findByText('Test Story'));
    expect(setStoryMock).toHaveBeenCalledWith(story);
  });
});
