import * as React from "react";
import { Slot } from "@radix-ui/react-slot";
import { clsx } from "clsx";
import { twMerge } from "tailwind-merge";

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  /** Use Radix Slot to render as a child element (e.g. Link). */
  asChild?: boolean;
  variant?: "primary" | "outline" | "outline-white" | "secondary" | "ghost" | "destructive";
  size?: "sm" | "md" | "lg";
}

const variantStyles: Record<NonNullable<ButtonProps["variant"]>, string> = {
  primary:
    "bg-brand-600 text-white hover:bg-brand-700 dark:bg-brand-500 dark:hover:bg-brand-600 focus-visible:ring-brand-500",
  outline:
    "border border-gray-300 dark:border-gray-700 text-gray-700 dark:text-gray-200 hover:bg-gray-50 dark:hover:bg-gray-800 focus-visible:ring-gray-400",
  "outline-white":
    "border border-white/50 text-white hover:bg-white/10 focus-visible:ring-white",
  secondary:
    "bg-white text-brand-700 hover:bg-brand-50 focus-visible:ring-brand-400",
  ghost:
    "text-gray-700 dark:text-gray-200 hover:bg-gray-100 dark:hover:bg-gray-800 focus-visible:ring-gray-400",
  destructive:
    "bg-red-600 text-white hover:bg-red-700 focus-visible:ring-red-500",
};

const sizeStyles: Record<NonNullable<ButtonProps["size"]>, string> = {
  sm: "h-8 px-3 text-sm gap-1.5",
  md: "h-10 px-4 text-sm gap-2",
  lg: "h-12 px-6 text-base gap-2",
};

/**
 * Primary button component.
 *
 * Supports all button variants, sizes, and can render as any element via `asChild`.
 */
export const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  (
    { className, variant = "primary", size = "md", asChild = false, ...props },
    ref,
  ) => {
    const Comp = asChild ? Slot : "button";

    return (
      <Comp
        ref={ref}
        className={twMerge(
          clsx(
            // Base styles
            "inline-flex items-center justify-center rounded-lg font-medium",
            "transition-colors duration-150",
            "focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-offset-2",
            "disabled:pointer-events-none disabled:opacity-50",
            // Variant and size
            variantStyles[variant],
            sizeStyles[size],
          ),
          className,
        )}
        {...props}
      />
    );
  },
);

Button.displayName = "Button";
