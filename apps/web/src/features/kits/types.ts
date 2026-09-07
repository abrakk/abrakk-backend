export type DifficultyLevel = "beginner" | "intermediate" | "advanced";

export type SubjectArea =
  | "mathematics"
  | "literacy"
  | "science"
  | "arts"
  | "social_studies"
  | "languages"
  | "physical_education"
  | "other";

export interface Kit {
  id: string;
  title: string;
  description: string;
  subject: SubjectArea;
  difficulty: DifficultyLevel;
  age_min: number;
  age_max: number;
  language: string;
  learning_objectives: string[];
  materials_required: string[];
  author_id: string;
  is_published: boolean;
  download_count: number;
  created_at: string;
  updated_at: string;
}

export interface KitListParams {
  subject?: string;
  age_min?: string;
  age_max?: string;
  difficulty?: string;
  language?: string;
  search?: string;
  page?: string;
  per_page?: string;
}
