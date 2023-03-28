import { Menu, Transition } from '@headlessui/react';
import { EllipsisHorizontalIcon, TrashIcon } from '@heroicons/react/20/solid';
import clsx from 'clsx';
import { Fragment, useEffect, useState } from 'react';

import useTauriEvent from '~/hooks/useTauriEvent';
import { Subscription, getSubscriptions } from '~/lib/bindings';

export default function SubscriptionList() {
  const [urls, setUrls] = useState<Subscription[]>([]);

  useEffect(() => {
    getUrls();
  }, []);

  useTauriEvent<Subscription[]>('feed-refresh', (event) => {
    setUrls(event.payload);
  });

  const getUrls = async () => {
    try {
      const result = await getSubscriptions();
      setUrls(result);
    } catch (error) {
      console.error(error);
    }
  };

  return (
    <ul className="">
      {urls.map((subscription) => (
        <SubscriptionListItem
          key={subscription.name}
          subscription={subscription}
        />
      ))}
    </ul>
  );
}

function SubscriptionListItem({
  subscription,
}: {
  subscription: Subscription;
}) {
  const onSelect = () => {
    // TODO
  };

  const onRemove = () => {
    // TODO
  };

  return (
    <li className="flex justify-between p-2 [&:not(:last-child)]:border-b dark:border-gray-500">
      <button type="button" onClick={onSelect}>
        {subscription.name}
      </button>
      <div className="static text-right">
        <Menu as="div" className="ml-auto relative inline-block text-left">
          <Menu.Button className="inline-flex w-full justify-center rounded-md bg-black bg-opacity-20 px-2 py-1 text-sm font-medium text-white hover:bg-opacity-30 focus:outline-none focus-visible:ring-2 focus-visible:ring-white focus-visible:ring-opacity-75">
            <span className="sr-only">Options</span>
            <EllipsisHorizontalIcon className="h-5 w-5" aria-hidden />
          </Menu.Button>
          <Transition
            as={Fragment}
            enter="transition ease-out duration-100"
            enterFrom="transform opacity-0 scale-95"
            enterTo="transform opacity-100 scale-100"
            leave="transition ease-in duration-75"
            leaveFrom="transform opacity-100 scale-100"
            leaveTo="transform opacity-0 scale-95"
          >
            <Menu.Items className="absolute z-10 right-0 mt-2 origin-top-right divide-y divide-gray-100 rounded-md bg-white shadow-lg ring-1 ring-black ring-opacity-5 focus:outline-none">
              <div className="p-1">
                <Menu.Item>
                  {({ active }) => (
                    <button
                      onClick={onRemove}
                      className={clsx(
                        'group flex w-full items-center rounded-md px-2 py-2 text-sm',
                        {
                          'bg-violet-500 text-white': active,
                          'text-gray-900': !active,
                        },
                      )}
                    >
                      <TrashIcon className="h-5 w-5 mr-1" aria-hidden />
                      Remove
                    </button>
                  )}
                </Menu.Item>
              </div>
            </Menu.Items>
          </Transition>
        </Menu>
      </div>
    </li>
  );
}
