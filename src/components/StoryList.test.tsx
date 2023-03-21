import { render, waitFor } from '@testing-library/react';
import userEvent from '@testing-library/user-event';
import { MockedFunction } from 'vitest';

import StoryList from './StoryList';

const invokeMock = window.__TAURI_INVOKE__ as MockedFunction<
  typeof window.__TAURI_INVOKE__
>;

const setStoryMock = vi.fn();
vi.mock('~/contexts/currentStory', () => ({
  useCurrentStory: vi.fn(() => ({ setStory: setStoryMock })),
}));

describe('StoryList', () => {
  it('renders titles and relative dates of stories', async () => {
    vi.setSystemTime('2023-03-16');
    invokeMock.mockResolvedValue([
      {
        title: 'Test Story',
        content: 'This is a test',
        link: 'https://example.com',
        pubDate: '2023-03-15',
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
      pubDate: '2023-03-15',
    };
    invokeMock.mockResolvedValue([story]);

    const user = userEvent.setup();
    const { findByText } = render(<StoryList />);
    await user.click(await findByText('Test Story'));
    expect(setStoryMock).toHaveBeenCalledWith(story);
  });
});
