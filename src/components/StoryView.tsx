import { useCurrentStory } from '~/contexts/currentStory';
import { sanitizeHtml } from '~/lib/utils';

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
        dangerouslySetInnerHTML={{ __html: sanitizeHtml(story.description) }}
      />
    </div>
  );
}
