import SideBar from '~/components/Sidebar';
import StoryList from '~/components/StoryList';
import StoryView from '~/components/StoryView';
import { CurrentStoryProvider } from '~/contexts/currentStory';

export default function App() {
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
