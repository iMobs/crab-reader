import { useCurrentStory } from '~/contexts/currentStory';

export default function StoryView() {
  const { story } = useCurrentStory();

  if (!story) {
    return null;
  }

  return (
    <div>
      <h3 className="font-bold">{story.title}</h3>
      <div
        className="prose dark:prose-invert"
        dangerouslySetInnerHTML={{ __html: story.content }}
      />
    </div>
  );
}
