// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.

declare global {
  interface Window {
    __TAURI_INVOKE__<T>(
      cmd: string,
      args?: Record<string, unknown>,
    ): Promise<T>;
  }
}

const invoke = window.__TAURI_INVOKE__;

export function getStories() {
  return invoke<Story[]>('get_stories');
}

export function getSubscriptions() {
  return invoke<Subscription[]>('get_subscriptions');
}

export function addFeed(url: string) {
  return invoke<null>('add_feed', { url });
}

export function refresh() {
  return invoke<null>('refresh');
}

export type Subscription = { name: string; url: string };
export type Story = {
  title: string;
  link: string;
  description: string;
  pub_date: string;
};
