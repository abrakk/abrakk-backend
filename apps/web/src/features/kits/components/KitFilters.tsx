"use client";

import { useRouter, useSearchParams, usePathname } from "next/navigation";
import { useCallback } from "react";

const SUBJECTS = [
  { value: "", label: "All Subjects" },
  { value: "mathematics", label: "Mathematics" },
  { value: "literacy", label: "Literacy" },
  { value: "science", label: "Science" },
  { value: "arts", label: "Arts" },
  { value: "social_studies", label: "Social Studies" },
  { value: "languages", label: "Languages" },
  { value: "physical_education", label: "Physical Education" },
  { value: "other", label: "Other" },
];

const DIFFICULTIES = [
  { value: "", label: "All Levels" },
  { value: "beginner", label: "Beginner" },
  { value: "intermediate", label: "Intermediate" },
  { value: "advanced", label: "Advanced" },
];

const LANGUAGES = [
  { value: "", label: "All Languages" },
  { value: "en", label: "English" },
  { value: "fr", label: "French" },
  { value: "yo", label: "Yoruba" },
  { value: "ha", label: "Hausa" },
  { value: "ig", label: "Igbo" },
  { value: "pt", label: "Portuguese" },
];

/**
 * Client-side filter panel for the explore page.
 * Updates the URL search params on change so filters are shareable and bookmarkable.
 */
export function KitFilters() {
  const router = useRouter();
  const pathname = usePathname();
  const searchParams = useSearchParams();

  const updateFilter = useCallback(
    (key: string, value: string) => {
      const params = new URLSearchParams(searchParams.toString());
      if (value) {
        params.set(key, value);
      } else {
        params.delete(key);
      }
      // Reset to page 1 when filters change
      params.delete("page");
      router.push(`${pathname}?${params.toString()}`);
    },
    [router, pathname, searchParams],
  );

  return (
    <div className="rounded-xl border border-gray-200 dark:border-gray-800 p-5 space-y-6">
      <h2 className="font-semibold text-gray-900 dark:text-white">Filters</h2>

      {/* Search */}
      <FilterGroup label="Search">
        <input
          type="search"
          placeholder="Search kits..."
          defaultValue={searchParams.get("search") ?? ""}
          onChange={(e) => updateFilter("search", e.target.value)}
          className="w-full rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm text-gray-900 dark:text-gray-50 placeholder:text-gray-400 focus:outline-none focus:ring-2 focus:ring-brand-500"
          aria-label="Search kits"
        />
      </FilterGroup>

      {/* Subject */}
      <FilterGroup label="Subject">
        <FilterSelect
          value={searchParams.get("subject") ?? ""}
          options={SUBJECTS}
          onChange={(v) => updateFilter("subject", v)}
          aria-label="Filter by subject"
        />
      </FilterGroup>

      {/* Difficulty */}
      <FilterGroup label="Difficulty">
        <FilterSelect
          value={searchParams.get("difficulty") ?? ""}
          options={DIFFICULTIES}
          onChange={(v) => updateFilter("difficulty", v)}
          aria-label="Filter by difficulty"
        />
      </FilterGroup>

      {/* Age range */}
      <FilterGroup label="Age Range">
        <div className="flex items-center gap-2">
          <input
            type="number"
            min={0}
            max={18}
            placeholder="Min"
            defaultValue={searchParams.get("age_min") ?? ""}
            onChange={(e) => updateFilter("age_min", e.target.value)}
            className="w-full rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
            aria-label="Minimum age"
          />
          <span className="text-gray-400 shrink-0" aria-hidden="true">–</span>
          <input
            type="number"
            min={0}
            max={18}
            placeholder="Max"
            defaultValue={searchParams.get("age_max") ?? ""}
            onChange={(e) => updateFilter("age_max", e.target.value)}
            className="w-full rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-brand-500"
            aria-label="Maximum age"
          />
        </div>
      </FilterGroup>

      {/* Language */}
      <FilterGroup label="Language">
        <FilterSelect
          value={searchParams.get("language") ?? ""}
          options={LANGUAGES}
          onChange={(v) => updateFilter("language", v)}
          aria-label="Filter by language"
        />
      </FilterGroup>

      {/* Clear filters */}
      {[...searchParams.entries()].length > 0 && (
        <button
          type="button"
          onClick={() => router.push(pathname)}
          className="w-full rounded-lg border border-gray-300 dark:border-gray-700 px-3 py-2 text-sm text-gray-600 dark:text-gray-400 hover:bg-gray-50 dark:hover:bg-gray-800 transition-colors"
        >
          Clear all filters
        </button>
      )}
    </div>
  );
}

function FilterGroup({
  label,
  children,
}: {
  label: string;
  children: React.ReactNode;
}) {
  return (
    <div className="space-y-2">
      <label className="text-sm font-medium text-gray-700 dark:text-gray-300">{label}</label>
      {children}
    </div>
  );
}

function FilterSelect({
  value,
  options,
  onChange,
  ...props
}: {
  value: string;
  options: { value: string; label: string }[];
  onChange: (value: string) => void;
} & React.SelectHTMLAttributes<HTMLSelectElement>) {
  return (
    <select
      value={value}
      onChange={(e) => onChange(e.target.value)}
      className="w-full rounded-lg border border-gray-300 dark:border-gray-700 bg-white dark:bg-gray-900 px-3 py-2 text-sm text-gray-900 dark:text-gray-50 focus:outline-none focus:ring-2 focus:ring-brand-500"
      {...props}
    >
      {options.map((opt) => (
        <option key={opt.value} value={opt.value}>
          {opt.label}
        </option>
      ))}
    </select>
  );
}
