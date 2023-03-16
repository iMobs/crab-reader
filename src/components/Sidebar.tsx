import AddFeed from '~/components/AddFeed';

export default function SideBar() {
  return (
    <div className="h-screen">
      <div>
        <AddFeed />
      </div>
      <div>{/* TODO: Add Feed list */}</div>
    </div>
  );
}
