import { useEffect } from 'react';

import SideBar from '~/components/Sidebar';
import StoryList from '~/components/StoryList';
import StoryView from '~/components/StoryView';
import { CurrentStoryProvider } from '~/contexts/currentStory';
import { refresh } from '~/lib/bindings';

export default function App() {
  useEffect(() => {
    refresh();
  }, []);

  return (
    <div className="flex">
      <SideBar />
      <CurrentStoryProvider>
        <StoryList />
        <StoryView />
      </CurrentStoryProvider>
    </div>
  );
}
