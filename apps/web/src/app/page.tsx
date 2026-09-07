import Link from "next/link";
import { ArrowRight, BookOpen, Users, Download } from "lucide-react";
import { Button } from "@/components/ui/Button";

export default function HomePage() {
  return (
    <div className="flex flex-col gap-24 pb-24">
      {/* Hero */}
      <section className="bg-gradient-to-b from-brand-50 to-white dark:from-gray-900 dark:to-gray-950 pt-20 pb-16">
        <div className="container-page text-center">
          <span className="inline-block rounded-full bg-brand-100 dark:bg-brand-900 px-3 py-1 text-sm font-medium text-brand-700 dark:text-brand-300 mb-6">
            Open Source · Free to Use
          </span>

          <h1 className="text-4xl sm:text-5xl lg:text-6xl font-bold tracking-tight text-gray-900 dark:text-white max-w-4xl mx-auto">
            Quality learning resources,{" "}
            <span className="text-brand-600 dark:text-brand-400">open to everyone</span>
          </h1>

          <p className="mt-6 text-xl text-gray-600 dark:text-gray-300 max-w-2xl mx-auto">
            Discover, create, and share educational learning activities and printable kits.
            Built by educators, for educators — completely free and open source.
          </p>

          <div className="mt-10 flex flex-col sm:flex-row gap-4 justify-center">
            <Button asChild size="lg">
              <Link href="/explore">
                Explore Kits
                <ArrowRight className="ml-2 h-4 w-4" aria-hidden="true" />
              </Link>
            </Button>
            <Button asChild variant="outline" size="lg">
              <Link href="/create">Create a Kit</Link>
            </Button>
          </div>
        </div>
      </section>

      {/* Stats */}
      <section className="container-page">
        <div className="grid grid-cols-1 gap-8 sm:grid-cols-3">
          <StatCard
            icon={<BookOpen className="h-6 w-6" aria-hidden="true" />}
            value="500+"
            label="Learning Kits"
          />
          <StatCard
            icon={<Users className="h-6 w-6" aria-hidden="true" />}
            value="1,200+"
            label="Educators"
          />
          <StatCard
            icon={<Download className="h-6 w-6" aria-hidden="true" />}
            value="10,000+"
            label="Downloads"
          />
        </div>
      </section>

      {/* Features */}
      <section className="container-page">
        <div className="text-center mb-12">
          <h2 className="text-3xl font-bold text-gray-900 dark:text-white">
            Everything you need to teach
          </h2>
          <p className="mt-4 text-lg text-gray-600 dark:text-gray-300">
            Tools built for teachers, parents, and educational content creators.
          </p>
        </div>

        <div className="grid grid-cols-1 gap-8 sm:grid-cols-2 lg:grid-cols-3">
          <FeatureCard
            title="Browse & Discover"
            description="Search through hundreds of learning kits filtered by age, subject, difficulty, and language."
          />
          <FeatureCard
            title="Create Your Own"
            description="Build and publish educational kits with our guided creation tools. Share your expertise with the world."
          />
          <FeatureCard
            title="Download & Print"
            description="Download printable materials and activity sheets. Ready to use in classrooms and at home."
          />
          <FeatureCard
            title="Culturally Relevant"
            description="Resources available in multiple languages, designed to respect and reflect different cultural contexts."
          />
          <FeatureCard
            title="Accessibility First"
            description="All content is designed with accessibility in mind, including screen reader support and keyboard navigation."
          />
          <FeatureCard
            title="Open Source"
            description="EduKit is open source and welcomes contributions. Help us improve educational access for everyone."
          />
        </div>
      </section>

      {/* CTA */}
      <section className="container-page">
        <div className="rounded-2xl bg-brand-600 dark:bg-brand-700 px-8 py-16 text-center">
          <h2 className="text-3xl font-bold text-white">
            Ready to contribute?
          </h2>
          <p className="mt-4 text-lg text-brand-100">
            EduKit is built by the community. Find an issue on GitHub and make your first contribution.
          </p>
          <div className="mt-8 flex flex-col sm:flex-row gap-4 justify-center">
            <Button asChild variant="secondary" size="lg">
              <a
                href="https://github.com/edukit-open/edukit-platform"
                target="_blank"
                rel="noopener noreferrer"
              >
                View on GitHub
              </a>
            </Button>
            <Button asChild variant="outline-white" size="lg">
              <Link href="/explore">Browse Kits</Link>
            </Button>
          </div>
        </div>
      </section>
    </div>
  );
}

function StatCard({
  icon,
  value,
  label,
}: {
  icon: React.ReactNode;
  value: string;
  label: string;
}) {
  return (
    <div className="flex flex-col items-center gap-2 rounded-xl border border-gray-200 dark:border-gray-800 p-8 text-center">
      <span className="text-brand-600 dark:text-brand-400">{icon}</span>
      <span className="text-3xl font-bold text-gray-900 dark:text-white">{value}</span>
      <span className="text-gray-600 dark:text-gray-400">{label}</span>
    </div>
  );
}

function FeatureCard({
  title,
  description,
}: {
  title: string;
  description: string;
}) {
  return (
    <div className="rounded-xl border border-gray-200 dark:border-gray-800 p-6">
      <h3 className="text-lg font-semibold text-gray-900 dark:text-white">{title}</h3>
      <p className="mt-2 text-gray-600 dark:text-gray-400">{description}</p>
    </div>
  );
}
