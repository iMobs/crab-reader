import { useEffect, useState } from 'react';

import { useCurrentStory } from '~/contexts/currentStory';
import useTauriEvent from '~/hooks/useTauriEvent';
import { Story, getStories } from '~/lib/bindings';
import { formatRelativeDistance } from '~/lib/chrono';

export default function StoryList() {
  const [stories, setStories] = useState<Story[]>([]);
  const { setStory } = useCurrentStory();

  useEffect(() => {
    loadStories();
  }, []);

  /* c8 ignore next 3 */
  useTauriEvent('feed-refresh', () => {
    loadStories();
  });

  const loadStories = async () => {
    try {
      const result = await getStories();
      setStories(result);
      /* c8 ignore next 3 */
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ul className="h-screen">
      {stories.map((story) => (
        <li key={story.link} onClick={() => setStory(story)}>
          <h3>
            <span className="font-bold">{story.title}</span>{' '}
            {formatRelativeDistance(story.pub_date)} ago
          </h3>
        </li>
      ))}
    </ul>
  );
}
