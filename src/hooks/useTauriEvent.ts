import { EventCallback, UnlistenFn, listen } from '@tauri-apps/api/event';
import { useEffect, useRef } from 'react';

export default function useTauriEvent<T>(
  event: string,
  callback: EventCallback<T>,
) {
  const callbackRef = useRef(callback);

  useEffect(() => {
    let unlisten: UnlistenFn | undefined;

    (async function () {
      try {
        unlisten = await listen<T>(event, (e) => callbackRef.current(e));
      } catch (error) {
        console.error(error);
      }
    })();

    return () => unlisten?.();
  }, [event]);
}
