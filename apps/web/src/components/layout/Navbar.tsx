import Link from "next/link";
import { BookOpen, Menu } from "lucide-react";
import { Button } from "@/components/ui/Button";

/**
 * Top navigation bar.
 *
 * Accessibility: uses a <nav> landmark with aria-label, and
 * each link has descriptive text.
 */
export function Navbar() {
  return (
    <header className="sticky top-0 z-40 border-b border-gray-200 dark:border-gray-800 bg-white/80 dark:bg-gray-950/80 backdrop-blur-sm">
      <nav
        className="container-page flex h-16 items-center justify-between gap-4"
        aria-label="Main navigation"
      >
        {/* Logo */}
        <Link
          href="/"
          className="flex items-center gap-2 font-bold text-xl text-gray-900 dark:text-white focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-brand-500 rounded"
          aria-label="EduKit home"
        >
          <BookOpen className="h-6 w-6 text-brand-600 dark:text-brand-400" aria-hidden="true" />
          EduKit
        </Link>

        {/* Desktop links */}
        <ul className="hidden md:flex items-center gap-6 list-none" role="list">
          <li>
            <Link
              href="/explore"
              className="text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
            >
              Explore
            </Link>
          </li>
          <li>
            <Link
              href="/create"
              className="text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
            >
              Create
            </Link>
          </li>
          <li>
            <a
              href="https://github.com/edukit-open/edukit-platform"
              target="_blank"
              rel="noopener noreferrer"
              className="text-sm font-medium text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white transition-colors"
            >
              GitHub
            </a>
          </li>
        </ul>

        {/* Auth buttons */}
        <div className="hidden md:flex items-center gap-3">
          <Button asChild variant="ghost" size="sm">
            <Link href="/login">Sign in</Link>
          </Button>
          <Button asChild size="sm">
            <Link href="/register">Get started</Link>
          </Button>
        </div>

        {/* Mobile menu button — screen-reader accessible */}
        <button
          type="button"
          className="md:hidden p-2 rounded-lg text-gray-600 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-800"
          aria-label="Open menu"
          aria-expanded="false"
        >
          <Menu className="h-5 w-5" aria-hidden="true" />
        </button>
      </nav>
    </header>
  );
}
