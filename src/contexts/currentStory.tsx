import { ReactNode, createContext, useContext, useMemo, useState } from 'react';

import { Story } from '~/lib/bindings';

interface CurrentStory {
  story: Story | null;
  setStory: (story: Story | null) => void;
}

export const currentStoryContext = createContext<CurrentStory | null>(null);

export function CurrentStoryProvider({ children }: { children: ReactNode }) {
  const [story, setStory] = useState<Story | null>(null);

  const value = useMemo(() => ({ story, setStory }), [story]);

  return (
    <currentStoryContext.Provider value={value}>
      {children}
    </currentStoryContext.Provider>
  );
}

export function useCurrentStory() {
  const context = useContext(currentStoryContext);

  if (!context) {
    throw new Error(
      'useCurrentStory must be wrapped by a CurrentStoryProvider',
    );
  }

  return context;
}
