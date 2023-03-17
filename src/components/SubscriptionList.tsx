import { invoke } from '@tauri-apps/api';
import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';

interface Subscription {
  name: string;
  url: string;
}

export default function SubscriptionList() {
  const [urls, setUrls] = useState<Subscription[]>([]);

  useEffect(() => {
    getUrls();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getUrls();
  });

  const getUrls = async () => {
    const result = await invoke<Subscription[]>('get_subscriptions');
    setUrls(result);
  };

  return (
    <ul>
      {urls.map((subscription) => (
        <li key={subscription.name}>{subscription.name}</li>
      ))}
    </ul>
  );
}
