import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { formatRelativeDistance } from '~/utils/chrono';

// TODO: Move this or have the backend generate it
interface Story {
  title: string;
  link: string;
  description: string;
  pub_date: string;
}

export default function ItemList() {
  const [items, setList] = useState<Story[]>([]);

  useEffect(() => {
    getItems();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getItems();
  });

  const getItems = async () => {
    const result = await invoke<Story[]>('get_stories');
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
