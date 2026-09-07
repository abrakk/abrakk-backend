import type { Metadata } from "next";
import { notFound } from "next/navigation";
import { Download, BookOpen, Clock, Globe, Users } from "lucide-react";
import { getKit } from "@/features/kits/services/kitService";
import { Button } from "@/components/ui/Button";
import { Badge } from "@/components/ui/Badge";

interface KitPageProps {
  params: { id: string };
}

export async function generateMetadata({ params }: KitPageProps): Promise<Metadata> {
  try {
    const kit = await getKit(params.id);
    return {
      title: kit.title,
      description: kit.description,
    };
  } catch {
    return { title: "Kit not found" };
  }
}

export default async function KitPage({ params }: KitPageProps) {
  let kit;
  try {
    kit = await getKit(params.id);
  } catch {
    notFound();
  }

  return (
    <div className="container-page py-12 max-w-4xl">
      {/* Header */}
      <div className="mb-8">
        <div className="flex flex-wrap gap-2 mb-4">
          <Badge variant="subject">{kit.subject.replace("_", " ")}</Badge>
          <Badge variant="difficulty">{kit.difficulty}</Badge>
          <Badge variant="neutral">Ages {kit.age_min}–{kit.age_max}</Badge>
        </div>

        <h1 className="text-3xl font-bold text-gray-900 dark:text-white">{kit.title}</h1>

        <div className="mt-4 flex flex-wrap gap-4 text-sm text-gray-600 dark:text-gray-400">
          <span className="flex items-center gap-1">
            <Globe className="h-4 w-4" aria-hidden="true" />
            {kit.language.toUpperCase()}
          </span>
          <span className="flex items-center gap-1">
            <Download className="h-4 w-4" aria-hidden="true" />
            {kit.download_count.toLocaleString()} downloads
          </span>
        </div>
      </div>

      <div className="grid gap-8 lg:grid-cols-3">
        {/* Main content */}
        <div className="lg:col-span-2 flex flex-col gap-8">
          {/* Description */}
          <section>
            <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-3">
              About this kit
            </h2>
            <p className="text-gray-700 dark:text-gray-300 leading-relaxed">
              {kit.description}
            </p>
          </section>

          {/* Learning objectives */}
          {kit.learning_objectives.length > 0 && (
            <section>
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-3 flex items-center gap-2">
                <BookOpen className="h-5 w-5 text-brand-600 dark:text-brand-400" aria-hidden="true" />
                Learning Objectives
              </h2>
              <ul className="space-y-2" role="list">
                {kit.learning_objectives.map((obj, i) => (
                  <li key={i} className="flex items-start gap-2 text-gray-700 dark:text-gray-300">
                    <span
                      className="mt-1.5 h-2 w-2 rounded-full bg-brand-500 shrink-0"
                      aria-hidden="true"
                    />
                    {obj}
                  </li>
                ))}
              </ul>
            </section>
          )}

          {/* Materials */}
          {kit.materials_required.length > 0 && (
            <section>
              <h2 className="text-xl font-semibold text-gray-900 dark:text-white mb-3">
                Materials Required
              </h2>
              <ul className="grid grid-cols-2 gap-2" role="list">
                {kit.materials_required.map((mat, i) => (
                  <li
                    key={i}
                    className="rounded-lg border border-gray-200 dark:border-gray-800 px-3 py-2 text-sm text-gray-700 dark:text-gray-300"
                  >
                    {mat}
                  </li>
                ))}
              </ul>
            </section>
          )}
        </div>

        {/* Sidebar */}
        <aside className="flex flex-col gap-4">
          <div className="rounded-xl border border-gray-200 dark:border-gray-800 p-6">
            <Button className="w-full" size="lg" asChild>
              <a href={`/api/kits/${kit.id}/download`}>
                <Download className="mr-2 h-4 w-4" aria-hidden="true" />
                Download Kit
              </a>
            </Button>

            <div className="mt-6 space-y-3 text-sm">
              <InfoRow label="Subject" value={kit.subject.replace("_", " ")} />
              <InfoRow label="Difficulty" value={kit.difficulty} />
              <InfoRow label="Age Range" value={`${kit.age_min}–${kit.age_max} years`} />
              <InfoRow label="Language" value={kit.language.toUpperCase()} />
            </div>
          </div>
        </aside>
      </div>
    </div>
  );
}

function InfoRow({ label, value }: { label: string; value: string }) {
  return (
    <div className="flex justify-between text-gray-600 dark:text-gray-400">
      <span className="font-medium">{label}</span>
      <span className="capitalize">{value}</span>
    </div>
  );
}
