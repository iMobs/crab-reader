import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Story, getStories } from '~/lib/bindings';
import { formatRelativeDistance } from '~/lib/chrono';

export default function ItemList() {
  const [items, setList] = useState<Story[]>([]);

  useEffect(() => {
    getItems();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getItems();
  });

  const getItems = async () => {
    try {
      const result = await getStories();
      setList(result);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ul className="h-screen">
      {items.map((item) => (
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
