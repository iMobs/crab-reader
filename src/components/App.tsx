import SideBar from '~/components/Sidebar';
import StoryList from '~/components/StoryList';

export default function App() {
  return (
    <div className="flex">
      <SideBar />
      <StoryList />
    </div>
  );
}
