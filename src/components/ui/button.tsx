import * as React from "react"
import { Button as BaseButton } from "@base-ui/react"
import { cn } from "../../lib/utils"

export interface ButtonProps extends React.ButtonHTMLAttributes<HTMLButtonElement> {
  variant?: "primary" | "secondary" | "outline" | "ghost" | "danger" | "success"
  size?: "sm" | "md" | "lg" | "icon"
}

const Button = React.forwardRef<HTMLButtonElement, ButtonProps>(
  ({ className, variant = "primary", size = "md", ...props }, ref) => {
    const variants = {
      primary: "bg-blue-600 text-white hover:bg-blue-700 active:bg-blue-800 shadow-sm",
      secondary: "bg-slate-100 text-slate-900 hover:bg-slate-200 active:bg-slate-300 dark:bg-slate-800 dark:text-slate-100 dark:hover:bg-slate-700",
      outline: "border border-slate-200 bg-white hover:bg-slate-50 text-slate-700 dark:border-slate-800 dark:bg-slate-950 dark:text-slate-200 dark:hover:bg-slate-900",
      ghost: "bg-transparent hover:bg-slate-100 text-slate-600 dark:text-slate-400 dark:hover:bg-slate-900",
      danger: "bg-red-500 text-white hover:bg-red-600 active:bg-red-700 shadow-sm",
      success: "bg-emerald-600 text-white hover:bg-emerald-700 active:bg-emerald-800 shadow-sm",
    }

    const sizes = {
      sm: "h-8 px-3 text-xs",
      md: "h-9 px-4 text-sm font-medium",
      lg: "h-10 px-6 text-base",
      icon: "h-9 w-9",
    }

    return (
      <BaseButton
        className={cn(
          "inline-flex items-center justify-center rounded-xl transition-all duration-200 disabled:opacity-50 disabled:pointer-events-none focus:outline-none focus:ring-2 focus:ring-blue-500/20 active:scale-[0.98]",
          variants[variant],
          sizes[size],
          className
        )}
        ref={ref}
        {...props}
      />
    )
  }
)
Button.displayName = "Button"

export { Button }
