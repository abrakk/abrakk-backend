import * as React from "react";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export interface BadgeProps extends React.HTMLAttributes<HTMLSpanElement> {
  variant?: "subject" | "difficulty" | "neutral" | "success" | "warning";
}

const variantStyles: Record<NonNullable<BadgeProps["variant"]>, string> = {
  subject:  "bg-blue-100 text-blue-800 dark:bg-blue-900 dark:text-blue-200",
  difficulty: "bg-amber-100 text-amber-800 dark:bg-amber-900 dark:text-amber-200",
  neutral:  "bg-gray-100 text-gray-700 dark:bg-gray-800 dark:text-gray-300",
  success:  "bg-green-100 text-green-800 dark:bg-green-900 dark:text-green-200",
  warning:  "bg-red-100 text-red-800 dark:bg-red-900 dark:text-red-200",
};

/**
 * Small status/label badge.
 */
export function Badge({ className, variant = "neutral", children, ...props }: BadgeProps) {
  return (
    <span
      className={twMerge(
        clsx(
          "inline-flex items-center rounded-full px-2.5 py-0.5 text-xs font-medium capitalize",
          variantStyles[variant],
        ),
        className,
      )}
      {...props}
    >
      {children}
    </span>
  );
}
