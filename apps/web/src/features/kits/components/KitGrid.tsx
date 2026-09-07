import { getKits } from "../services/kitService";
import { KitCard } from "./KitCard";
import type { KitListParams } from "../types";

interface KitGridProps {
  searchParams: KitListParams;
}

/**
 * Server component — fetches kits from the API and renders the grid.
 *
 * Wrapped in <Suspense> in the page so it shows a skeleton while loading.
 */
export async function KitGrid({ searchParams }: KitGridProps) {
  let kits;

  try {
    kits = await getKits(searchParams);
  } catch {
    return (
      <div
        role="alert"
        className="rounded-xl border border-red-200 dark:border-red-900 bg-red-50 dark:bg-red-950 p-6 text-sm text-red-700 dark:text-red-400"
      >
        Unable to load kits right now. Please try again later.
      </div>
    );
  }

  if (kits.length === 0) {
    return (
      <div className="rounded-xl border border-gray-200 dark:border-gray-800 p-12 text-center">
        <p className="text-gray-600 dark:text-gray-400">
          No kits found. Try adjusting your filters.
        </p>
      </div>
    );
  }

  return (
    <section aria-label="Learning kits">
      <p className="text-sm text-gray-600 dark:text-gray-400 mb-4">
        {kits.length} {kits.length === 1 ? "kit" : "kits"} found
      </p>
      <ul
        className="grid grid-cols-1 gap-6 sm:grid-cols-2 xl:grid-cols-3"
        role="list"
        aria-label="Kit results"
      >
        {kits.map((kit) => (
          <li key={kit.id}>
            <KitCard kit={kit} />
          </li>
        ))}
      </ul>
    </section>
  );
}
