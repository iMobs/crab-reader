import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';

export default function SubscriptionList() {
  const [urls, setUrls] = useState<string[]>([]);

  useEffect(() => {
    getUrls();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getUrls();
  });

  const getUrls = async () => {
    const result = await invoke<string[]>('get_subscriptions');
    setUrls(result);
  };

  return (
    <ul>
      {urls.map((url) => (
        <li key={url}>{url}</li>
      ))}
    </ul>
  );
}
