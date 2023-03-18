import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Story, getStories } from '~/lib/bindings';
import { formatRelativeDistance } from '~/lib/chrono';

export default function StoryList() {
  const [stories, setStories] = useState<Story[]>([]);

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
    <ul className="h-screen">
      {stories.map((item) => (
        <li key={item.link}>
          <h3>
            <span className="font-bold">{item.title}</span>{' '}
            {formatRelativeDistance(item.pub_date)} ago
          </h3>
        </li>
      ))}
    </ul>
  );
}
