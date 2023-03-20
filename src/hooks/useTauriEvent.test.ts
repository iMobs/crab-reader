import { Event, listen } from '@tauri-apps/api/event';
import { renderHook, waitFor } from '@testing-library/react';
import { MockedFunction } from 'vitest';

import useTauriEvent from './useTauriEvent';

vi.mock('@tauri-apps/api/event');

const listenMock = listen as MockedFunction<typeof listen>;

describe('useTauriEvent', () => {
  it('calls the callback on an event', async () => {
    const callback = vi.fn();
    renderHook(() => useTauriEvent('test', callback));
    await waitFor(() =>
      expect(listenMock).toHaveBeenCalledWith('test', expect.anything()),
    );

    const handler = listenMock.mock.lastCall?.[1];
    const event = { payload: null, event: 'test' };
    handler?.(event as Event<null>);
    expect(callback).toHaveBeenCalledWith(event);
  });
});
