import { render, renderHook } from '@testing-library/react';

import {
  CurrentStoryProvider,
  currentStoryContext,
  useCurrentStory,
} from './currentStory';

describe('currentStory', () => {
  describe('CurrentStoryProvider', () => {
    it('wraps children', () => {
      // TODO: Test provided context somehow
      const { getByText } = render(
        <CurrentStoryProvider>TODO</CurrentStoryProvider>,
      );
      expect(getByText('TODO')).toBeInTheDocument();
    });
  });

  describe('useCurrentStory', () => {
    it('use current story context', () => {
      const value = { story: null, setStory: vi.fn() };
      const { result } = renderHook(useCurrentStory, {
        wrapper: ({ children }) => (
          <currentStoryContext.Provider value={value}>
            {children}
          </currentStoryContext.Provider>
        ),
      });

      expect(result.current).toBe(value);
    });

    it('throws an error if not in context', () => {
      expect(() => renderHook(useCurrentStory)).toThrowError(
        'useCurrentStory must be wrapped by a CurrentStoryProvider',
      );
    });
  });
});
