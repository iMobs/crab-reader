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

  useTauriEvent('feed-refresh', () => {
    loadStories();
  });

  const loadStories = async () => {
    try {
      const result = await getStories();
      setStories(result);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ul className="overflow-auto pt-4">
      {stories.map((story) => (
        <li
          key={story.link}
          className="p-2 [&:not(:last-child)]:border-b dark:border-gray-500"
        >
          <button className="flex" onClick={() => setStory(story)}>
            <h3 className="font-bold dark:text-gray-200">{story.title}</h3>
            <span className="ml-auto">
              {formatRelativeDistance(story.pubDate)} ago
            </span>
          </button>
        </li>
      ))}
    </ul>
  );
}
