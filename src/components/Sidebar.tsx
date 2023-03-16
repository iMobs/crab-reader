import AddFeedButton from '~/components/AddFeed';

export default function SideBar() {
  return (
    <div className="h-screen">
      <div>
        <AddFeedButton />
      </div>
      <div>{/* TODO: Add Feed list */}</div>
    </div>
  );
}
