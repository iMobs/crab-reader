import ItemList from '~/components/ItemList';
import SideBar from '~/components/Sidebar';

export default function App() {
  return (
    <div className="flex">
      <SideBar />
      <ItemList />
    </div>
  );
}
