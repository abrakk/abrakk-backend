/**
 * Loading skeleton shown while KitGrid fetches data.
 * Matches the visual structure of KitCard to prevent layout shift.
 */
export function KitGridSkeleton() {
  return (
    <section aria-busy="true" aria-label="Loading kits">
      <div className="h-4 w-24 bg-gray-200 dark:bg-gray-800 rounded animate-pulse mb-4" />
      <ul className="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-3" role="list">
        {Array.from({ length: 6 }).map((_, i) => (
          <li key={i}>
            <div className="rounded-xl border border-gray-200 dark:border-gray-800 p-5 space-y-3 animate-pulse">
              {/* Badges */}
              <div className="flex gap-2">
                <div className="h-5 w-20 rounded-full bg-gray-200 dark:bg-gray-700" />
                <div className="h-5 w-16 rounded-full bg-gray-200 dark:bg-gray-700" />
              </div>
              {/* Title */}
              <div className="h-5 w-3/4 rounded bg-gray-200 dark:bg-gray-700" />
              {/* Description */}
              <div className="space-y-1.5">
                <div className="h-3.5 w-full rounded bg-gray-200 dark:bg-gray-700" />
                <div className="h-3.5 w-5/6 rounded bg-gray-200 dark:bg-gray-700" />
              </div>
              {/* Meta */}
              <div className="flex gap-4 pt-2">
                <div className="h-3 w-16 rounded bg-gray-200 dark:bg-gray-700" />
                <div className="h-3 w-10 rounded bg-gray-200 dark:bg-gray-700" />
              </div>
            </div>
          </li>
        ))}
      </ul>
    </section>
  );
}
