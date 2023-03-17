import { mockIPC, mockWindows } from '@tauri-apps/api/mocks';
import '@testing-library/jest-dom';

vi.mock('tauri-plugin-log-api');

mockIPC(async (cmd) => {
  switch (cmd) {
    case 'add_feed':
      return;

    case 'get_subscriptions':
      return [];

    case 'get_stories':
      return [];

    default:
      return;
  }
});

beforeEach(() => mockWindows('main'));
