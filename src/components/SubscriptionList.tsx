import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Subscription, getSubscriptions } from '~/lib/bindings';

export default function SubscriptionList() {
  const [urls, setUrls] = useState<Subscription[]>([]);

  useEffect(() => {
    getUrls();
  }, []);

  useTauriEvent('feed-refresh', () => {
    getUrls();
  });

  const getUrls = async () => {
    const result = await getSubscriptions();
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
