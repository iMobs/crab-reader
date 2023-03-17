import { Dialog, Transition } from '@headlessui/react';
import { zodResolver } from '@hookform/resolvers/zod';
import { Fragment, useState } from 'react';
import { useForm } from 'react-hook-form';
import * as log from 'tauri-plugin-log-api';
import { z } from 'zod';

import { addFeed } from '~/utils/bindings';

export default function AddFeed() {
  const [isOpen, setIsOpen] = useState(false);

  const onClose = () => {
    setIsOpen(false);
  };

  return (
    <>
      <div>
        <button onClick={() => setIsOpen(true)}>Add Feed +</button>
      </div>
      <Transition appear show={isOpen} as={Fragment}>
        <Dialog className="relative z-10" onClose={onClose}>
          <Transition.Child
            as={Fragment}
            enter="ease-out duration-300"
            enterFrom="opacity-0"
            enterTo="opacity-100"
            leave="ease-in duration-200"
            leaveFrom="opacity-100"
            leaveTo="opacity-0"
          >
            <div className="fixed inset-0 bg-black bg-opacity-25" />
          </Transition.Child>
          <div className="fixed inset-0 overflow-y-auto">
            <div className="flex min-h-full items-center justify-center p-4 text-center">
              <Transition.Child
                as={Fragment}
                enter="ease-out duration-300"
                enterFrom="opacity-0 scale-95"
                enterTo="opacity-100 scale-100"
                leave="ease-in duration-200"
                leaveFrom="opacity-100 scale-100"
                leaveTo="opacity-0 scale-95"
              >
                <Dialog.Panel className="w-full max-w-md transform overflow-hidden rounded-2xl bg-white dark:bg-gray-500 p-6 text-left align-middle shadow-xl transition-all">
                  <Dialog.Title
                    as="h3"
                    className="text-lg font-medium leading-6 text-gray-900 dark:text-white"
                  >
                    Add a Feed
                  </Dialog.Title>
                  <AddFeedForm onClose={onClose} />
                </Dialog.Panel>
              </Transition.Child>
            </div>
          </div>
        </Dialog>
      </Transition>
    </>
  );
}

const formSchema = z.object({
  url: z.string().url(),
});

type FormData = z.infer<typeof formSchema>;

function AddFeedForm({ onClose }: { onClose: () => void }) {
  const { handleSubmit, register, formState } = useForm<FormData>({
    resolver: zodResolver(formSchema),
  });

  const onSubmit = async (data: FormData) => {
    log.debug(`Adding feed: ${JSON.stringify(data)}`);

    try {
      await addFeed(data.url);
      log.debug('success!');
      onClose();
    } catch (e) {
      if (e instanceof Error) {
        log.error(`Error submitting feed: ${e.message}`);
      }
    }
  };

  return (
    <form onSubmit={handleSubmit(onSubmit)}>
      <div className="flex items-center py-4">
        <input
          className="appearance-none border-none w-full mr-3 py-1 px-2 leading-tight focus:outline-none text-black"
          type="text"
          aria-label="Feed URL"
          placeholder="http://example.com/feed"
          {...register('url')}
        />
        <input
          className="flex-shrink-0 bg-teal-500 hover:bg-teal-700 border-teal-500 hover:border-teal-700 text-sm border-4 text-white py-1 px-2 rounded"
          type="submit"
          value="Add"
        />
        <button
          className="flex-shrink-0 border-transparent border-4 text-teal-500 hover:text-teal-800 text-sm py-1 px-2 rounded"
          onClick={onClose}
          type="button"
        >
          Cancel
        </button>
      </div>
      {formState.errors.url && (
        <p className="text-red-500 text-xs italic">
          {formState.errors.url.message}
        </p>
      )}
    </form>
  );
}
