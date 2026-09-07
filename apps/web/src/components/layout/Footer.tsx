import Link from "next/link";
import { BookOpen } from "lucide-react";

/**
 * Site footer.
 */
export function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t border-gray-200 dark:border-gray-800 bg-white dark:bg-gray-950">
      <div className="container-page py-12">
        <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-4">
          {/* Brand */}
          <div>
            <Link
              href="/"
              className="flex items-center gap-2 font-bold text-lg text-gray-900 dark:text-white"
              aria-label="EduKit home"
            >
              <BookOpen className="h-5 w-5 text-brand-600 dark:text-brand-400" aria-hidden="true" />
              EduKit
            </Link>
            <p className="mt-3 text-sm text-gray-600 dark:text-gray-400">
              Open-source educational resources for everyone.
            </p>
          </div>

          {/* Platform */}
          <nav aria-label="Platform links">
            <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-4">Platform</h3>
            <ul className="space-y-3 list-none" role="list">
              <li>
                <Link href="/explore" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Explore Kits
                </Link>
              </li>
              <li>
                <Link href="/create" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Create a Kit
                </Link>
              </li>
              <li>
                <Link href="/dashboard" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Dashboard
                </Link>
              </li>
            </ul>
          </nav>

          {/* Community */}
          <nav aria-label="Community links">
            <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-4">Community</h3>
            <ul className="space-y-3 list-none" role="list">
              <li>
                <a
                  href="https://github.com/edukit-open/edukit-platform"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
                >
                  GitHub
                </a>
              </li>
              <li>
                <a
                  href="https://github.com/edukit-open/edukit-platform/issues"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors"
                >
                  Issues
                </a>
              </li>
              <li>
                <Link href="/contributing" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Contributing
                </Link>
              </li>
            </ul>
          </nav>

          {/* Legal */}
          <nav aria-label="Legal links">
            <h3 className="text-sm font-semibold text-gray-900 dark:text-white mb-4">Legal</h3>
            <ul className="space-y-3 list-none" role="list">
              <li>
                <Link href="/privacy" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Privacy Policy
                </Link>
              </li>
              <li>
                <Link href="/terms" className="text-sm text-gray-600 dark:text-gray-400 hover:text-gray-900 dark:hover:text-white transition-colors">
                  Terms of Use
                </Link>
              </li>
            </ul>
          </nav>
        </div>

        <div className="mt-10 border-t border-gray-200 dark:border-gray-800 pt-8 flex flex-col sm:flex-row items-center justify-between gap-4">
          <p className="text-sm text-gray-600 dark:text-gray-400">
            © {currentYear} EduKit Open. Released under the MIT License.
          </p>
          <p className="text-sm text-gray-600 dark:text-gray-400">
            Built with ❤️ by the open-source community
          </p>
        </div>
      </div>
    </footer>
  );
}
