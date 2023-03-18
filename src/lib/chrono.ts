import { formatDistanceToNow } from 'date-fns';

export function formatRelativeDistance(date: Date | string) {
  if (typeof date === 'string') {
    date = new Date(date);
  }

  return formatDistanceToNow(date);
}
