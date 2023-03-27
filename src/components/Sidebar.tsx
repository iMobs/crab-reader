import AddFeed from '~/components/AddFeed';
import SubscriptionList from '~/components/SubscriptionList';

export default function SideBar() {
  return (
    <div className="px-4">
      <div className="my-4">
        <AddFeed />
      </div>
      <div>
        <SubscriptionList />
      </div>
    </div>
  );
}
