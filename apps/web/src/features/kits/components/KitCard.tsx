import Link from "next/link";
import { Download, Globe } from "lucide-react";
import { Badge } from "@/components/ui/Badge";
import type { Kit } from "../types";

interface KitCardProps {
  kit: Kit;
}

/**
 * Card displaying a summary of a learning kit.
 *
 * Accessibility: the card title is a link, and supplementary info is
 * grouped in a descriptive list so screen readers can navigate it.
 */
export function KitCard({ kit }: KitCardProps) {
  return (
    <article className="flex flex-col rounded-xl border border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-900 overflow-hidden hover:shadow-md transition-shadow duration-200">
      {/* Card body */}
      <div className="flex flex-col gap-3 p-5 flex-1">
        {/* Badges */}
        <div className="flex flex-wrap gap-1.5">
          <Badge variant="subject">{kit.subject.replace("_", " ")}</Badge>
          <Badge variant="difficulty">{kit.difficulty}</Badge>
        </div>

        {/* Title */}
        <h3 className="text-base font-semibold text-gray-900 dark:text-white leading-snug">
          <Link
            href={`/kits/${kit.id}`}
            className="hover:text-brand-600 dark:hover:text-brand-400 transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand-500 rounded"
          >
            {kit.title}
          </Link>
        </h3>

        {/* Description — clamped to 2 lines */}
        <p className="text-sm text-gray-600 dark:text-gray-400 line-clamp-2">
          {kit.description}
        </p>

        {/* Meta */}
        <dl className="mt-auto flex flex-wrap gap-x-4 gap-y-1 text-xs text-gray-500 dark:text-gray-400">
          <div className="flex items-center gap-1">
            <dt className="sr-only">Age range</dt>
            <dd>Ages {kit.age_min}–{kit.age_max}</dd>
          </div>
          <div className="flex items-center gap-1">
            <Globe className="h-3 w-3" aria-hidden="true" />
            <dt className="sr-only">Language</dt>
            <dd>{kit.language.toUpperCase()}</dd>
          </div>
          <div className="flex items-center gap-1">
            <Download className="h-3 w-3" aria-hidden="true" />
            <dt className="sr-only">Downloads</dt>
            <dd>{kit.download_count.toLocaleString()}</dd>
          </div>
        </dl>
      </div>

      {/* Card footer */}
      <div className="border-t border-gray-100 dark:border-gray-800 px-5 py-3">
        <Link
          href={`/kits/${kit.id}`}
          className="text-sm font-medium text-brand-600 dark:text-brand-400 hover:underline focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand-500 rounded"
          aria-label={`View details for ${kit.title}`}
        >
          View kit →
        </Link>
      </div>
    </article>
  );
}
