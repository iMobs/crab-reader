import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Story, getStories } from '~/utils/bindings';
import { formatRelativeDistance } from '~/utils/chrono';

export default function ItemList() {
  const [items, setList] = useState<Story[]>([]);

  useEffect(() => {
    getItems();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getItems();
  });

  const getItems = async () => {
    const result = await getStories();
    setList(result);
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
