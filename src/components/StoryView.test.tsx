import { render } from '@testing-library/react';

import StoryView from './StoryView';

vi.mock('~/contexts/currentStory', () => ({
  useCurrentStory: vi.fn(() => ({
    story: {
      title: 'Test Story',
      content: 'Test Content',
    },
  })),
}));

describe('StoryView', () => {
  it('shows a story title and content', () => {
    const { getByText } = render(<StoryView />);
    expect(getByText('Test Story')).toBeInTheDocument();
    expect(getByText('Test Content')).toBeInTheDocument();
  });
});
