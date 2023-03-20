import { useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Subscription, getSubscriptions } from '~/lib/bindings';

export default function SubscriptionList() {
  const [urls, setUrls] = useState<Subscription[]>([]);

  useEffect(() => {
    getUrls();
  }, []);

  /* c8 ignore next 3 */
  useTauriEvent('feed-refresh', () => {
    getUrls();
  });

  const getUrls = async () => {
    try {
      const result = await getSubscriptions();
      setUrls(result);
      /* c8 ignore next 3 */
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ul>
      {urls.map((subscription) => (
        <li key={subscription.name}>{subscription.name}</li>
      ))}
    </ul>
  );
}
