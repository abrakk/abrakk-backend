import { Suspense } from "react";
import type { Metadata } from "next";
import { KitGrid } from "@/features/kits/components/KitGrid";
import { KitFilters } from "@/features/kits/components/KitFilters";
import { KitGridSkeleton } from "@/features/kits/components/KitGridSkeleton";

export const metadata: Metadata = {
  title: "Explore Kits",
  description: "Browse educational learning kits filtered by age, subject, and difficulty.",
};

interface ExplorePageProps {
  searchParams: {
    subject?: string;
    age_min?: string;
    age_max?: string;
    difficulty?: string;
    language?: string;
    search?: string;
    page?: string;
  };
}

export default function ExplorePage({ searchParams }: ExplorePageProps) {
  return (
    <div className="container-page py-12">
      <div className="mb-8">
        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">Explore Kits</h1>
        <p className="mt-2 text-gray-600 dark:text-gray-400">
          Browse educational learning kits for all ages and subjects.
        </p>
      </div>

      <div className="flex flex-col gap-8 lg:flex-row">
        {/* Filters sidebar */}
        <aside className="w-full lg:w-64 shrink-0" aria-label="Filter learning kits">
          <KitFilters />
        </aside>

        {/* Kit grid */}
        <div className="flex-1">
          <Suspense fallback={<KitGridSkeleton />}>
            <KitGrid searchParams={searchParams} />
          </Suspense>
        </div>
      </div>
    </div>
  );
}
